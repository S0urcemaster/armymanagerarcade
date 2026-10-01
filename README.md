# Army Manager Arcade

A mobile web game about building an army, fighting other armies, and advancing
through history.

The original game concept is preserved in [README-init.md](README-init.md).

## Project development

The project is developed with the decision-guided process documented in
[project-nutcracker](project-nutcracker/README.md).

Ask **"What comes next?"** to get a short, ranked list of the most important
current project concerns and a recommendation for the next decision or action.

## Development

The client is a Rust application rendered into a browser canvas with Macroquad.

```sh
cargo run
./scripts/build-web.sh
python3 -m http.server 8080 -d dist
```

Then open `http://127.0.0.1:8080`.

The code is separated by responsibility:

- `src/game.rs` contains state, commands, and configurable game rules without
  rendering code.
- `src/ui.rs` contains the responsive canvas layout and custom game widgets.
- `src/main.rs` owns the application loop and passes UI commands to the game.

Gameplay numbers belong in `GameRules`; persistent values belong in `Game`;
visual dimensions and colors belong in the UI. This keeps balancing and rules
independent from presentation while avoiding unnecessary framework layers.

The browser client automatically stores one current campaign in `localStorage`.
There are no save slots; starting a new game replaces that campaign after a
confirmation step.
