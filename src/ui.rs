use crate::game::{Game, GameCommand, GameRules, Screen};
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
}

pub struct UiState {
    pub screen: Screen,
    swipe_start: Option<Vec2>,
    confirm_new_game: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            screen: Screen::Menu,
            swipe_start: None,
            confirm_new_game: false,
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
    Screen::Army,
    Screen::Recruit,
];

fn screen_index(screen: Screen) -> usize {
    SCREENS
        .iter()
        .position(|candidate| *candidate == screen)
        .unwrap_or(0)
}

fn adjacent_screen(screen: Screen, offset: isize) -> Option<Screen> {
    let index = screen_index(screen) as isize + offset;
    (index >= 0 && index < SCREENS.len() as isize).then(|| SCREENS[index as usize])
}

fn screen_navigation(screen: Screen) -> Option<GameCommand> {
    if crate::storage::is_desktop() {
        let height = 54.0;
        let y = screen_height() - height;
        let half = screen_width() * 0.5;

        if let Some(previous) = adjacent_screen(screen, -1) {
            if button(Rect::new(0.0, y, half, height), "<", false) {
                return Some(GameCommand::ChangeScreen(previous));
            }
        } else {
            draw_rectangle(0.0, y, half, height, colors::PANEL);
        }

        if let Some(next) = adjacent_screen(screen, 1) {
            if button(Rect::new(half, y, half, height), ">", false) {
                return Some(GameCommand::ChangeScreen(next));
            }
        } else {
            draw_rectangle(half, y, half, height, colors::PANEL);
        }
        return None;
    }

    let current = screen_index(screen);
    let y = screen_height() - 24.0;
    let center = (SCREENS.len() as f32 - 1.0) * 0.5;
    let pointer = Vec2::from(mouse_position());
    for (i, target) in SCREENS.iter().enumerate() {
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
            if let Some(next) = adjacent_screen(ui.screen, direction) {
                return Some(GameCommand::ChangeScreen(next));
            }
        }
    }
    None
}

pub fn draw(game: &Game, rules: &GameRules, ui: &mut UiState) -> Option<GameCommand> {
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
        Screen::Recruit => {
            top_bar(game, "RECRUIT");
            draw_recruit()
        }
    };
    command
        .or_else(|| screen_navigation(ui.screen))
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
    if button(Rect::new(0.0, y, screen_width(), 58.0), "CONTINUE", true) {
        return Some(GameCommand::ChangeScreen(Screen::Campaign));
    }
    if button(
        Rect::new(0.0, y + 70.0, screen_width(), 58.0),
        "ARMY",
        false,
    ) {
        return Some(GameCommand::ChangeScreen(Screen::Army));
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

fn draw_recruit() -> Option<GameCommand> {
    let info = section(102.0, 70.0);
    label(
        "AVAILABLE RECRUITS",
        14.0,
        info.y + 42.0,
        20.0,
        colors::FRAME,
    );
    let recruit = section(184.0, 112.0);
    draw_rectangle(0.0, recruit.y, 6.0, recruit.h, colors::FRAME);
    label("LOCAL SPEARMAN", 14.0, recruit.y + 35.0, 20.0, colors::TEXT);
    label(
        "40 gold  ·  Talent ?",
        14.0,
        recruit.y + 65.0,
        16.0,
        colors::MUTED,
    );
    label("+3 ESPIONAGE", 14.0, recruit.y + 91.0, 15.0, colors::FRAME);
    None
}
