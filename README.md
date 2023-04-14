# red

Text editor and TUI multiplexing library, for fun and education.

## Features:

- [x] Floating windows
  - [ ] Render optimizations
- [x] Horizontal and vertical splits
- [x] Global keyboard events
- [x] Extensible widget trait
- [x] Conventient layout primitives
- [ ] Focus
- [ ] Focused-widget keyboard events
- [ ] Mouse events

## Demo

Watch in fullscreen, the lines don't render properly in a small viewport.

[FloatingDemo.webm](https://user-images.githubusercontent.com/38540736/231884015-44bb77ce-2111-4d92-b463-b6a02b29be8b.webm)

## Demo usage

The TUI library is not yet ready for use so it hasn't been published yet, but you can run the `demo` if you want to try it out.

```sh

$ git clone git@github.com:willothy/red

$ cd red

$ cargo run --example demo

```

### Demo keymaps:

- `<C-q>`: quit
- `<Tab>`: switch layout (not working with new UI manager yet)
- `<Shift-Tab>`: switch current float
- `<Up>`: move the current float up
- `<Down>`: move the current float down
- `<Left>`: move the current float left
- `<Right>`: move the current float right
- `<Shift-Up>`: resize the current float
- `<Shift-Down>`: resize the current float
- `<Shift-Left>`: resize the current float
- `<Shift-Right>`: resize the current float
