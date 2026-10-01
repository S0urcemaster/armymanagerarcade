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
- The primary screens have no persistent tab bar.
- The player navigates between adjacent primary screens by swiping left or right.
- Small position indicators may communicate the current screen without behaving
  as tabs.
- A minimal menu control returns to the splash/menu screen.

## Rendering

- Visible game UI is drawn by the Rust/Macroquad client, not with HTML controls.
- HTML only hosts and sizes the canvas and loads static WebAssembly assets.
