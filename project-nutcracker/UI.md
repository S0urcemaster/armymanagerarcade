# User Interface Rules

## Format

- The game uses a portrait-only mobile layout.
- UI coordinates correspond directly to canvas pixels; there is no virtual
  resolution or independent horizontal and vertical scaling.
- On phones, the canvas uses the full available width with no outer left or
  right margin.
- On wider desktop displays, the portrait canvas is centered and the surrounding
  page remains outside the game surface.

## Layout

- Avoid nested horizontal indentation and decorative side margins.
- Prefer full-width sections separated vertically.
- Keep the future army tree horizontally flat. Hierarchy is communicated through
  grouping, labels, scale, connectors, or vertical order rather than increasingly
  indented branches.
- Touch targets must remain comfortably usable on a phone.

## Navigation

- The title appears on a dedicated splash/menu screen, not on every screen.
- Every primary screen has its own concise screen title; the full game title is
  still reserved for the splash/menu screen.
- The primary screens have no persistent tab bar.
- The splash/menu participates in the same horizontal sequence as the primary
  screens and can be reached by swiping left or right.
- Small bottom position controls communicate all screens and may be tapped for
  direct navigation.
- On desktop, a full-width bottom bar with previous and next buttons replaces
  the position controls.
- There is no persistent menu button.

## Persistence

- The browser keeps one automatically updated current campaign in local storage.
- There are no selectable save-game slots.
- Starting a new game requires confirmation before replacing the current
  campaign.
- A future high score is conceptually separate from the current campaign and
  should survive a new-game reset.

## Rendering

- Visible game UI is drawn by the Rust/Macroquad client, not with HTML controls.
- HTML only hosts and sizes the canvas and loads static WebAssembly assets.
