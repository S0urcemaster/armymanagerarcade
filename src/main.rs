mod game;
mod storage;
mod ui;

use game::{Game, GameCommand, GameRules};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Army Manager Arcade".into(),
        window_width: 390,
        window_height: 844,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let rules = GameRules::default();
    let mut game = storage::load().unwrap_or_else(|| Game::new(&rules));
    let mut ui_state = ui::UiState::default();
    let ui_assets = ui::UiAssets::new();

    loop {
        clear_background(ui::colors::BACKGROUND);

        if let Some(command) = ui::draw(&game, &rules, &mut ui_state, &ui_assets) {
            match command {
                GameCommand::ChangeScreen(screen) => ui_state.screen = screen,
                GameCommand::PrepareBattle => {
                    game.apply(GameCommand::PrepareBattle, &rules);
                    storage::save(&game);
                    ui_state.battle_active = true;
                    ui_state.screen = game::Screen::Battle;
                }
                GameCommand::FleeBattle => {
                    game.apply(GameCommand::FleeBattle, &rules);
                    storage::save(&game);
                    ui_state.battle_active = false;
                    ui_state.screen = game::Screen::Army;
                }
                GameCommand::ResetGame => {
                    game.apply(GameCommand::ResetGame, &rules);
                    storage::save(&game);
                    ui_state.battle_active = false;
                }
                other => {
                    game.apply(other, &rules);
                    storage::save(&game);
                }
            }
        }

        next_frame().await;
    }
}
