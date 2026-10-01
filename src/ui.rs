use crate::game::{Game, GameCommand, GameRules, Screen};
use crate::recruiting;
use macroquad::prelude::*;

pub mod colors {
    use macroquad::prelude::Color;
    pub const BACKGROUND: Color = Color::new(0.035, 0.045, 0.055, 1.0);
    pub const FRAME: Color = Color::new(0.70, 0.55, 0.27, 1.0);
    pub const PANEL: Color = Color::new(0.075, 0.09, 0.095, 1.0);
    pub const PANEL_ALT: Color = Color::new(0.12, 0.14, 0.14, 1.0);
    pub const TEXT: Color = Color::new(0.91, 0.88, 0.78, 1.0);
    pub const MUTED: Color = Color::new(0.58, 0.60, 0.56, 1.0);
    pub const ACCENT: Color = Color::new(0.73, 0.22, 0.14, 1.0);
    pub const GOOD: Color = Color::new(0.30, 0.58, 0.35, 1.0);
    pub const COIN: Color = Color::new(0.93, 0.69, 0.20, 1.0);
}

pub struct UiState {
    pub screen: Screen,
    swipe_start: Option<Vec2>,
    confirm_new_game: bool,
    confirm_flee: bool,
    recruiting_scroll: f32,
    recruiting_drag_y: Option<f32>,
    pub battle_active: bool,
}

pub struct UiAssets {
    recruit_portraits: [Texture2D; 4],
    group_portraits: [Texture2D; 4],
}

impl UiAssets {
    pub fn new() -> Self {
        let portraits = [
            include_bytes!("../assets/recruits/recruit-young-spearman.png").as_slice(),
            include_bytes!("../assets/recruits/recruit-veteran-spearman.png").as_slice(),
            include_bytes!("../assets/recruits/recruit-archer.png").as_slice(),
            include_bytes!("../assets/recruits/recruit-rider.png").as_slice(),
        ];
        let recruit_portraits = portraits.map(|bytes| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        });
        let groups = [
            include_bytes!("../assets/groups/group-infantry.png").as_slice(),
            include_bytes!("../assets/groups/group-archers.png").as_slice(),
            include_bytes!("../assets/groups/group-riders.png").as_slice(),
            include_bytes!("../assets/groups/group-mixed.png").as_slice(),
        ];
        let group_portraits = groups.map(|bytes| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        });
        Self {
            recruit_portraits,
            group_portraits,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            screen: Screen::Menu,
            swipe_start: None,
            confirm_new_game: false,
            confirm_flee: false,
            recruiting_scroll: 0.0,
            recruiting_drag_y: None,
            battle_active: false,
        }
    }
}

fn label(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x, y, size, color);
}

fn wrapped_label(text: &str, x: f32, y: f32, max_width: f32, size: f32, color: Color) {
    let mut line = String::new();
    let mut baseline = y;
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && measure_text(&candidate, None, size as u16, 1.0).width > max_width {
            label(&line, x, baseline, size, color);
            baseline += size + 5.0;
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        label(&line, x, baseline, size, color);
    }
}

fn coin_price(value: u32, right: f32, baseline: f32) {
    let text = value.to_string();
    let dimensions = measure_text(&text, None, 19, 1.0);
    let coin_x = right - dimensions.width - 17.0;
    let coin_y = baseline - 6.0;
    draw_circle(coin_x, coin_y, 7.0, colors::COIN);
    draw_circle_lines(coin_x, coin_y, 4.0, 1.2, colors::PANEL);
    label(&text, coin_x + 11.0, baseline, 19.0, colors::COIN);
}

fn button(rect: Rect, text: &str, active: bool) -> bool {
    let pointer = Vec2::from(mouse_position());
    let hovered = rect.contains(pointer);
    let fill = if active {
        colors::ACCENT
    } else if hovered {
        colors::PANEL_ALT
    } else {
        colors::PANEL
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        if hovered {
            colors::FRAME
        } else {
            colors::MUTED
        },
    );
    let dim = measure_text(text, None, 20, 1.0);
    label(
        text,
        rect.x + (rect.w - dim.width) * 0.5,
        rect.y + (rect.h + 15.0) * 0.5,
        20.0,
        colors::TEXT,
    );
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn section(y: f32, height: f32) -> Rect {
    let rect = Rect::new(0.0, y, screen_width(), height);
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, colors::PANEL);
    let border = Color::new(0.28, 0.28, 0.24, 1.0);
    draw_line(0.0, y, screen_width(), y, 1.0, border);
    draw_line(0.0, y + height, screen_width(), y + height, 1.0, border);
    rect
}

fn top_bar(game: &Game, title: &str) {
    draw_rectangle(0.0, 0.0, screen_width(), 92.0, colors::PANEL);
    label(
        &format!("GOLD {}   INTEL {}", game.gold, game.espionage),
        14.0,
        32.0,
        17.0,
        colors::TEXT,
    );
    label(title, 14.0, 76.0, 24.0, colors::FRAME);
}

const SCREENS: [Screen; 4] = [
    Screen::Menu,
    Screen::Campaign,
    Screen::Recruiting,
    Screen::Army,
];
const BATTLE_SCREENS: [Screen; 2] = [Screen::Menu, Screen::Battle];

fn navigation_screens(ui: &UiState) -> &'static [Screen] {
    if ui.battle_active {
        &BATTLE_SCREENS
    } else {
        &SCREENS
    }
}

fn screen_index(screens: &[Screen], screen: Screen) -> usize {
    screens
        .iter()
        .position(|candidate| *candidate == screen)
        .unwrap_or(0)
}

fn adjacent_screen(ui: &UiState, offset: isize) -> Option<Screen> {
    let screens = navigation_screens(ui);
    let index = screen_index(screens, ui.screen) as isize + offset;
    (index >= 0 && index < screens.len() as isize).then(|| screens[index as usize])
}

fn screen_navigation(ui: &UiState) -> Option<GameCommand> {
    let screens = navigation_screens(ui);
    if crate::storage::is_desktop() {
        let height = 54.0;
        let y = screen_height() - height;
        let half = screen_width() * 0.5;

        if let Some(previous) = adjacent_screen(ui, -1) {
            if button(Rect::new(0.0, y, half, height), "<", false) {
                return Some(GameCommand::ChangeScreen(previous));
            }
        } else {
            draw_rectangle(0.0, y, half, height, colors::PANEL);
        }

        if let Some(next) = adjacent_screen(ui, 1) {
            if button(Rect::new(half, y, half, height), ">", false) {
                return Some(GameCommand::ChangeScreen(next));
            }
        } else {
            draw_rectangle(half, y, half, height, colors::PANEL);
        }
        return None;
    }

    let current = screen_index(screens, ui.screen);
    let y = screen_height() - 24.0;
    let center = (screens.len() as f32 - 1.0) * 0.5;
    let pointer = Vec2::from(mouse_position());
    for (i, target) in screens.iter().enumerate() {
        let x = screen_width() * 0.5 + (i as f32 - center) * 20.0;
        let hit_area = Rect::new(x - 12.0, y - 12.0, 24.0, 24.0);
        draw_circle(
            x,
            y,
            if i == current { 4.5 } else { 3.0 },
            if i == current {
                colors::FRAME
            } else {
                colors::MUTED
            },
        );
        if hit_area.contains(pointer) && is_mouse_button_pressed(MouseButton::Left) {
            return Some(GameCommand::ChangeScreen(*target));
        }
    }

    None
}

fn swipe_command(ui: &mut UiState) -> Option<GameCommand> {
    let pointer = Vec2::from(mouse_position());
    if is_mouse_button_pressed(MouseButton::Left) {
        ui.swipe_start = Some(pointer);
    }
    if is_mouse_button_released(MouseButton::Left) {
        let start = ui.swipe_start.take()?;
        let delta = pointer - start;
        if delta.x.abs() > 70.0 && delta.x.abs() > delta.y.abs() * 1.4 {
            let direction = if delta.x < 0.0 { 1 } else { -1 };
            if let Some(next) = adjacent_screen(ui, direction) {
                return Some(GameCommand::ChangeScreen(next));
            }
        }
    }
    None
}

pub fn draw(
    game: &Game,
    rules: &GameRules,
    ui: &mut UiState,
    assets: &UiAssets,
) -> Option<GameCommand> {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        colors::BACKGROUND,
    );
    let command = match ui.screen {
        Screen::Menu => draw_menu(game, rules, ui),
        Screen::Campaign => {
            top_bar(game, "CAMPAIGN");
            draw_campaign(game, rules)
        }
        Screen::Army => {
            top_bar(game, "ARMY");
            draw_army(game, rules)
        }
        Screen::Recruiting => {
            let command = draw_recruiting(game, ui, assets);
            top_bar(game, "RECRUITING");
            command
        }
        Screen::Battle => {
            top_bar(game, "BATTLE");
            draw_battle(ui)
        }
    };
    command
        .or_else(|| screen_navigation(ui))
        .or_else(|| swipe_command(ui))
}

fn draw_menu(game: &Game, rules: &GameRules, ui: &mut UiState) -> Option<GameCommand> {
    let center = screen_width() * 0.5;
    label(
        "ARMY",
        center - 83.0,
        screen_height() * 0.24,
        46.0,
        colors::TEXT,
    );
    label(
        "MANAGER",
        center - 112.0,
        screen_height() * 0.24 + 47.0,
        46.0,
        colors::TEXT,
    );
    label(
        "ARCADE",
        center - 35.0,
        screen_height() * 0.24 + 75.0,
        18.0,
        colors::FRAME,
    );
    let y = screen_height() * 0.52;
    let continue_target = if ui.battle_active {
        Screen::Battle
    } else {
        Screen::Campaign
    };
    let continue_label = if ui.battle_active {
        "RETURN TO BATTLE"
    } else {
        "CONTINUE"
    };
    if button(
        Rect::new(0.0, y, screen_width(), 58.0),
        continue_label,
        true,
    ) {
        return Some(GameCommand::ChangeScreen(continue_target));
    }
    if !ui.battle_active {
        if button(
            Rect::new(0.0, y + 70.0, screen_width(), 58.0),
            "ARMY",
            false,
        ) {
            return Some(GameCommand::ChangeScreen(Screen::Army));
        }
    }
    if !ui.confirm_new_game {
        if button(
            Rect::new(0.0, y + 140.0, screen_width(), 58.0),
            "NEW GAME",
            false,
        ) {
            ui.confirm_new_game = true;
        }
    } else {
        label(
            "RESET CURRENT CAMPAIGN?",
            14.0,
            y + 164.0,
            18.0,
            colors::TEXT,
        );
        let half = screen_width() * 0.5;
        if button(Rect::new(0.0, y + 180.0, half, 54.0), "CANCEL", false) {
            ui.confirm_new_game = false;
        }
        if button(Rect::new(half, y + 180.0, half, 54.0), "RESET", true) {
            ui.confirm_new_game = false;
            return Some(GameCommand::ResetGame);
        }
    }
    label(
        &format!("STAGE {:02}  ·  {} SOLDIER", game.stage, game.soldiers),
        14.0,
        screen_height() - 76.0,
        16.0,
        colors::MUTED,
    );
    label(
        &format!("{} HZ", rules.target_simulation_hz),
        screen_width() - 55.0,
        screen_height() - 76.0,
        14.0,
        colors::MUTED,
    );
    None
}

fn draw_campaign(game: &Game, rules: &GameRules) -> Option<GameCommand> {
    let first = section(102.0, 116.0);
    label(
        &format!("STAGE {:02}  ·  BORDER OUTPOST", game.stage),
        14.0,
        first.y + 30.0,
        20.0,
        colors::FRAME,
    );
    label("YOUR POSITION", 14.0, first.y + 62.0, 18.0, colors::TEXT);
    label(
        "+12 gold income  ·  1 soldier",
        14.0,
        first.y + 88.0,
        16.0,
        colors::MUTED,
    );
    let enemy = section(230.0, 128.0);
    draw_rectangle(0.0, enemy.y, 6.0, enemy.h, colors::ACCENT);
    label(
        "NEXT: THE HILL FORT",
        14.0,
        enemy.y + 31.0,
        20.0,
        colors::TEXT,
    );
    label(
        "Enemy strength estimated",
        14.0,
        enemy.y + 61.0,
        16.0,
        colors::MUTED,
    );
    label(
        &format!("INTELLIGENCE LEVEL {}", game.intel_level),
        14.0,
        enemy.y + 88.0,
        15.0,
        colors::FRAME,
    );
    wrapped_label(
        game.notice,
        14.0,
        394.0,
        screen_width() - 28.0,
        17.0,
        colors::TEXT,
    );
    if button(
        Rect::new(0.0, 442.0, screen_width(), 56.0),
        &format!("SCOUT  ·  {} INTEL", rules.espionage_cost),
        false,
    ) {
        return Some(GameCommand::InspectNext);
    }
    None
}

fn draw_army(game: &Game, rules: &GameRules) -> Option<GameCommand> {
    let summary = section(102.0, 70.0);
    label(
        &format!(
            "LEVEL {}  ·  {} SOLDIER",
            game.army_level(rules),
            game.soldiers
        ),
        14.0,
        summary.y + 42.0,
        20.0,
        colors::FRAME,
    );
    let block = section(184.0, 196.0);
    draw_rectangle(0.0, block.y, 6.0, block.h, colors::GOOD);
    label("SPEARMEN BLOCK", 14.0, block.y + 34.0, 20.0, colors::TEXT);
    label("1 / 50 ASSIGNED", 14.0, block.y + 64.0, 16.0, colors::MUTED);
    let row_width = screen_width() / 10.0;
    for row in 0..10 {
        draw_rectangle(
            row as f32 * row_width + 2.0,
            block.y + 116.0,
            row_width - 4.0,
            16.0,
            if row == 0 {
                colors::FRAME
            } else {
                colors::PANEL_ALT
            },
        );
    }
    label("10 COMBAT ROWS", 14.0, block.y + 164.0, 15.0, colors::MUTED);
    if button(
        Rect::new(0.0, 394.0, screen_width(), 56.0),
        "PREPARE BATTLE",
        false,
    ) {
        return Some(GameCommand::PrepareBattle);
    }
    None
}

fn draw_recruiting(game: &Game, ui: &mut UiState, assets: &UiAssets) -> Option<GameCommand> {
    const LIST_TOP: f32 = 172.0;
    const CARD_STEP: f32 = 116.0;
    let list_bottom = screen_height() - 54.0;
    let max_scroll = (8.0 * CARD_STEP - (list_bottom - LIST_TOP) + 12.0).max(0.0);
    let pointer = Vec2::from(mouse_position());

    if pointer.y >= LIST_TOP && pointer.y <= list_bottom {
        ui.recruiting_scroll =
            (ui.recruiting_scroll - mouse_wheel().1 * 34.0).clamp(0.0, max_scroll);
        if is_mouse_button_pressed(MouseButton::Left) {
            ui.recruiting_drag_y = Some(pointer.y);
        }
    }
    if is_mouse_button_down(MouseButton::Left) {
        if let Some(previous_y) = ui.recruiting_drag_y.replace(pointer.y) {
            ui.recruiting_scroll =
                (ui.recruiting_scroll + previous_y - pointer.y).clamp(0.0, max_scroll);
        }
    }
    if is_mouse_button_released(MouseButton::Left) {
        ui.recruiting_drag_y = None;
    }

    let mut recruits = recruiting::generate_offers(0xA11CE, game.stage.min(3), 4);
    recruits.extend(recruiting::generate_offers(0xA11CE, game.stage.max(6), 4));
    for (index, recruit_data) in recruits.iter().enumerate() {
        let y = 184.0 + index as f32 * CARD_STEP - ui.recruiting_scroll;
        if y + 104.0 < LIST_TOP || y > list_bottom {
            continue;
        }
        let recruit = section(y, 104.0);
        let texture = if index < 4 {
            &assets.recruit_portraits[index]
        } else {
            &assets.group_portraits[index - 4]
        };
        draw_texture_ex(
            texture,
            0.0,
            recruit.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(104.0, 104.0)),
                ..Default::default()
            },
        );
        label(
            &recruit_data.leader_name.to_uppercase(),
            116.0,
            recruit.y + 25.0,
            16.0,
            colors::TEXT,
        );
        coin_price(
            recruit_data.hire_cost,
            screen_width() - 12.0,
            recruit.y + 25.0,
        );
        let offer_kind = if recruit_data.is_group() {
            format!(
                "{} {}",
                recruit_data.composition.label(),
                recruit_data.group_size
            )
        } else {
            "Individual".to_owned()
        };
        label(
            &format!(
                "{} · {} · +{} INT",
                recruit_data.profession.label(),
                offer_kind,
                recruit_data.intel
            ),
            116.0,
            recruit.y + 49.0,
            13.0,
            colors::MUTED,
        );
        label(
            &format!(
                "TAL {}   FIT {}   BEL {}",
                recruit_data.talent, recruit_data.fitness, recruit_data.belonging
            ),
            116.0,
            recruit.y + 72.0,
            14.0,
            colors::TEXT,
        );
        label(
            &format!(
                "ARM {} · Q{} {} · DUR {}%",
                recruit_data.armour_type.label(),
                recruit_data.armour_quality.level(),
                recruit_data.armour_quality.label(),
                recruit_data.armour_durability
            ),
            116.0,
            recruit.y + 92.0,
            12.0,
            colors::FRAME,
        );
    }

    draw_rectangle(0.0, 92.0, screen_width(), 80.0, colors::BACKGROUND);
    let info = section(102.0, 70.0);
    label(
        "AVAILABLE RECRUITS  ·  SCROLL",
        14.0,
        info.y + 42.0,
        19.0,
        colors::FRAME,
    );
    None
}

fn draw_battle(ui: &mut UiState) -> Option<GameCommand> {
    let status = section(102.0, 116.0);
    label(
        "BATTLE IN PROGRESS",
        14.0,
        status.y + 34.0,
        20.0,
        colors::FRAME,
    );
    label(
        "Cycle 01  ·  Front row engaged",
        14.0,
        status.y + 67.0,
        16.0,
        colors::TEXT,
    );
    label(
        "Your losses 0%  ·  Enemy 0%",
        14.0,
        status.y + 94.0,
        15.0,
        colors::MUTED,
    );

    if button(
        Rect::new(0.0, 236.0, screen_width(), 56.0),
        "CONTINUE",
        true,
    ) {
        return None;
    }
    if button(
        Rect::new(0.0, 304.0, screen_width(), 56.0),
        "START / PAUSE",
        false,
    ) {
        return None;
    }

    if !ui.confirm_flee {
        if button(
            Rect::new(0.0, 394.0, screen_width(), 56.0),
            "FLEE BATTLE",
            false,
        ) {
            ui.confirm_flee = true;
        }
    } else {
        wrapped_label(
            "This will cost you some troops. Are you sure?",
            14.0,
            402.0,
            screen_width() - 28.0,
            17.0,
            colors::TEXT,
        );
        let half = screen_width() * 0.5;
        if button(Rect::new(0.0, 458.0, half, 54.0), "CANCEL", false) {
            ui.confirm_flee = false;
        }
        if button(Rect::new(half, 458.0, half, 54.0), "FLEE", true) {
            ui.confirm_flee = false;
            return Some(GameCommand::FleeBattle);
        }
    }
    None
}
