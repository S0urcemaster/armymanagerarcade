mod game;
mod ui;

use game::{Game, GameCommand, GameRules};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Army Manager Arcade".into(),
        window_width: 1100,
        window_height: 760,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let rules = GameRules::default();
    let mut game = Game::new(&rules);
    let mut ui_state = ui::UiState::default();

    loop {
        clear_background(ui::colors::BACKGROUND);

        if let Some(command) = ui::draw(&game, &rules, &mut ui_state) {
            match command {
                GameCommand::ChangeScreen(screen) => ui_state.screen = screen,
                other => game.apply(other, &rules),
            }
        }

        next_frame().await;
    }
}
