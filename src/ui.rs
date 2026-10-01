use crate::game::{Game, GameCommand, GameRules, Screen};
use macroquad::prelude::*;

pub mod colors {
    use macroquad::prelude::Color;
    pub const BACKGROUND: Color = Color::new(0.035, 0.045, 0.055, 1.0);
    pub const FRAME: Color = Color::new(0.70, 0.55, 0.27, 1.0);
    pub const PANEL: Color = Color::new(0.09, 0.11, 0.12, 1.0);
    pub const PANEL_ALT: Color = Color::new(0.13, 0.15, 0.15, 1.0);
    pub const TEXT: Color = Color::new(0.91, 0.88, 0.78, 1.0);
    pub const MUTED: Color = Color::new(0.58, 0.60, 0.56, 1.0);
    pub const ACCENT: Color = Color::new(0.73, 0.22, 0.14, 1.0);
    pub const GOOD: Color = Color::new(0.30, 0.58, 0.35, 1.0);
}

pub struct UiState {
    pub screen: Screen,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            screen: Screen::Campaign,
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

fn panel(rect: Rect) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, colors::PANEL);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.5,
        Color::new(0.3, 0.3, 0.27, 1.0),
    );
}

fn button(rect: Rect, text: &str, active: bool) -> bool {
    let mouse = Vec2::from(mouse_position());
    let hovered = rect.contains(mouse);
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

fn formation_block(rect: Rect, title: &str, value: &str, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, colors::PANEL_ALT);
    draw_rectangle(rect.x, rect.y, 5.0, rect.h, color);
    label(title, rect.x + 14.0, rect.y + 25.0, 19.0, colors::TEXT);
    label(value, rect.x + 14.0, rect.y + 48.0, 16.0, colors::MUTED);
}

pub fn draw(game: &Game, rules: &GameRules, ui: &mut UiState) -> Option<GameCommand> {
    let compact = screen_width() < 720.0;
    let margin = if compact { 10.0 } else { 22.0 };
    let frame = Rect::new(
        margin,
        margin,
        screen_width() - margin * 2.0,
        screen_height() - margin * 2.0,
    );
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        frame.h,
        Color::new(0.055, 0.065, 0.07, 1.0),
    );
    draw_rectangle_lines(frame.x, frame.y, frame.w, frame.h, 3.0, colors::FRAME);

    let pad = if compact { 14.0 } else { 24.0 };
    let x = frame.x + pad;
    let width = frame.w - pad * 2.0;
    label(
        "ARMY MANAGER",
        x,
        frame.y + 35.0,
        if compact { 24.0 } else { 30.0 },
        colors::TEXT,
    );
    label("ARCADE", x, frame.y + 57.0, 16.0, colors::FRAME);
    label(
        &format!("GOLD  {}   INTEL  {}", game.gold, game.espionage),
        x + width - if compact { 170.0 } else { 210.0 },
        frame.y + 38.0,
        17.0,
        colors::TEXT,
    );

    let nav_y = frame.y + 72.0;
    let gap = 8.0;
    let nav_w = (width - gap * 2.0) / 3.0;
    for (i, (screen, name)) in [
        (Screen::Campaign, "CAMPAIGN"),
        (Screen::Army, "ARMY"),
        (Screen::Recruit, "RECRUIT"),
    ]
    .into_iter()
    .enumerate()
    {
        if button(
            Rect::new(x + i as f32 * (nav_w + gap), nav_y, nav_w, 42.0),
            name,
            ui.screen == screen,
        ) {
            return Some(GameCommand::ChangeScreen(screen));
        }
    }

    let content_y = nav_y + 56.0;
    let bottom = frame.y + frame.h - 64.0;
    let content_h = bottom - content_y;
    panel(Rect::new(x, content_y, width, content_h));
    label(
        &format!("SIM {} HZ  ·  RENDER LIVE", rules.target_simulation_hz),
        x,
        frame.y + frame.h - 20.0,
        14.0,
        colors::MUTED,
    );

    match ui.screen {
        Screen::Campaign => draw_campaign(
            game,
            rules,
            Rect::new(x, content_y, width, content_h),
            compact,
        ),
        Screen::Army => draw_army(
            game,
            rules,
            Rect::new(x, content_y, width, content_h),
            compact,
        ),
        Screen::Recruit => draw_recruit(Rect::new(x, content_y, width, content_h)),
    }
}

fn draw_campaign(game: &Game, rules: &GameRules, area: Rect, compact: bool) -> Option<GameCommand> {
    let p = 18.0;
    label(
        &format!("STAGE {:02}  ·  BORDER OUTPOST", game.stage),
        area.x + p,
        area.y + 32.0,
        21.0,
        colors::FRAME,
    );
    label(
        "A scripted road through the ancient world",
        area.x + p,
        area.y + 57.0,
        17.0,
        colors::MUTED,
    );
    let card_y = area.y + 78.0;
    let card_w = if compact {
        area.w - p * 2.0
    } else {
        (area.w - p * 3.0) * 0.5
    };
    formation_block(
        Rect::new(area.x + p, card_y, card_w, 64.0),
        "YOUR POSITION",
        "+12 gold income  ·  1 soldier",
        colors::GOOD,
    );
    let enemy_x = if compact {
        area.x + p
    } else {
        area.x + p * 2.0 + card_w
    };
    let enemy_y = if compact { card_y + 76.0 } else { card_y };
    formation_block(
        Rect::new(enemy_x, enemy_y, card_w, 64.0),
        "NEXT: THE HILL FORT",
        "Enemy strength estimated  ·  Intel I",
        colors::ACCENT,
    );
    let action_y = if compact {
        enemy_y + 86.0
    } else {
        card_y + 88.0
    };
    wrapped_label(
        game.notice,
        area.x + p,
        action_y + 26.0,
        area.w - p * 2.0,
        17.0,
        colors::TEXT,
    );
    let bw = if compact { card_w } else { 210.0 };
    if button(
        Rect::new(area.x + p, action_y + 48.0, bw, 48.0),
        &format!("SCOUT  ·  {} INTEL", rules.espionage_cost),
        false,
    ) {
        return Some(GameCommand::InspectNext);
    }
    None
}

fn draw_army(game: &Game, rules: &GameRules, area: Rect, compact: bool) -> Option<GameCommand> {
    let p = 18.0;
    label(
        &format!(
            "ARMY LEVEL {}  ·  {} SOLDIER",
            game.army_level(rules),
            game.soldiers
        ),
        area.x + p,
        area.y + 34.0,
        21.0,
        colors::FRAME,
    );
    label("FORMATION", area.x + p, area.y + 68.0, 16.0, colors::MUTED);
    let block = Rect::new(
        area.x + p,
        area.y + 82.0,
        area.w - p * 2.0,
        if compact { 105.0 } else { 150.0 },
    );
    draw_rectangle(
        block.x,
        block.y,
        block.w,
        block.h,
        Color::new(0.12, 0.16, 0.13, 1.0),
    );
    draw_rectangle_lines(block.x, block.y, block.w, block.h, 2.0, colors::GOOD);
    label(
        "SPEARMEN BLOCK",
        block.x + 16.0,
        block.y + 30.0,
        19.0,
        colors::TEXT,
    );
    label(
        "1 / 50 assigned",
        block.x + 16.0,
        block.y + 56.0,
        16.0,
        colors::MUTED,
    );
    for row in 0..10 {
        let rw = (block.w - 32.0) / 10.0;
        draw_rectangle(
            block.x + 16.0 + row as f32 * rw,
            block.y + block.h - 28.0,
            rw - 3.0,
            8.0,
            if row == 0 {
                colors::FRAME
            } else {
                colors::PANEL
            },
        );
    }
    if button(
        Rect::new(
            area.x + p,
            block.y + block.h + 20.0,
            if compact { block.w } else { 230.0 },
            48.0,
        ),
        "PREPARE BATTLE",
        false,
    ) {
        return Some(GameCommand::PrepareBattle);
    }
    None
}

fn draw_recruit(area: Rect) -> Option<GameCommand> {
    let p = 18.0;
    label(
        "RECRUIT POOL",
        area.x + p,
        area.y + 34.0,
        21.0,
        colors::FRAME,
    );
    label(
        "New candidates will appear here.",
        area.x + p,
        area.y + 66.0,
        18.0,
        colors::MUTED,
    );
    formation_block(
        Rect::new(area.x + p, area.y + 90.0, area.w - p * 2.0, 68.0),
        "LOCAL SPEARMAN",
        "40 gold  ·  Talent ?  ·  +3 espionage",
        colors::FRAME,
    );
    None
}
