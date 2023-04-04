//! Red - Rust EDitor
//! Author: Will Hopkins <willothyh@gmail.com>
//! License: MIT
//! Version: 0.1.0
//! Description: A simple text editor written in Rust.
//! Repository: https://github.com/willothy/red

use clap::Parser;

use crop::Rope;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use termwiz::caps::Capabilities;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers, MouseEvent};
use termwiz::surface::Change;
use termwiz::terminal::buffered::BufferedTerminal;
use termwiz::terminal::{new_terminal, Terminal};
use termwiz::widgets::layout::{
    self, ChildOrientation, HorizontalAlignment, LayoutState, VerticalAlignment,
};
use termwiz::widgets::{CursorShapeAndPosition, RenderArgs, Ui, UpdateArgs, Widget, WidgetEvent};

#[derive(Debug, Parser)]
struct Args {
    files: Vec<PathBuf>,
}

struct Buffer {
    data: Rope,
}

struct Window {
    buffer: Arc<Mutex<Buffer>>,
}

struct Editor<'a> {
    buffers: Vec<Arc<Mutex<Buffer>>>,
    windows: Vec<Window>,
    ui: Ui<'a>,
}

struct Screen;

impl Widget for Screen {
    fn render(&mut self, args: &mut RenderArgs) {}

    fn get_size_constraints(&self) -> termwiz::widgets::layout::Constraints {
        let mut c = layout::Constraints::default();
        c.child_orientation = ChildOrientation::Vertical;
        c
    }
}

/// This is the main text input area for the app
struct TextInput<'a> {
    /// Holds the input text that we wish the widget to display
    text: &'a mut String,
}

impl<'a> TextInput<'a> {
    /// Initialize the widget with the input text
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }
}

impl<'a> Widget for TextInput<'a> {
    fn process_event(&mut self, event: &WidgetEvent, _args: &mut UpdateArgs) -> bool {
        match event {
            WidgetEvent::Input(InputEvent::Key(KeyEvent {
                key: KeyCode::Char(c),
                ..
            })) => self.text.push(*c),
            WidgetEvent::Input(InputEvent::Key(KeyEvent {
                key: KeyCode::Enter,
                ..
            })) => {
                self.text.push_str("\r\n");
            }
            WidgetEvent::Input(InputEvent::Paste(s)) => {
                self.text.push_str(&s);
            }
            _ => {}
        }

        true // handled it all
    }

    /// Draw ourselves into the surface provided by RenderArgs
    fn render(&mut self, args: &mut RenderArgs) {
        args.surface.add_change(Change::ClearScreen(
            ColorAttribute::TrueColorWithPaletteFallback(
                (0x31, 0x1B, 0x92).into(),
                AnsiColor::Black.into(),
            ),
        ));
        args.surface
            .add_change(Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0xB3, 0x88, 0xFF).into(),
                    AnsiColor::Purple.into(),
                ),
            )));
        args.surface.add_change(self.text.clone());

        // Place the cursor at the end of the text.
        // A more advanced text editing widget would manage the
        // cursor position differently.
        *args.cursor = CursorShapeAndPosition {
            coords: args.surface.cursor_position().into(),
            shape: termwiz::surface::CursorShape::SteadyBar,
            ..Default::default()
        };
    }

    fn get_size_constraints(&self) -> layout::Constraints {
        let mut c = layout::Constraints::default();
        c.set_valign(VerticalAlignment::Top);
        c
    }
}

// This is a little status line widget that we render at the bottom
struct StatusLine {}

impl StatusLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for StatusLine {
    /// Draw ourselves into the surface provided by RenderArgs
    fn render(&mut self, args: &mut RenderArgs) {
        let dims = args.surface.dimensions();
        args.surface
            .add_change(Change::ClearScreen(AnsiColor::Grey.into()));
        args.surface
            .add_change(format!("🤷 status surface size is {:?}", dims));
    }

    fn get_size_constraints(&self) -> layout::Constraints {
        let mut c = layout::Constraints::default();
        c.set_fixed_height(1);
        c.set_valign(VerticalAlignment::Bottom);
        c
    }

    fn process_event(&mut self, event: &WidgetEvent, args: &mut UpdateArgs) -> bool {
        match event {
            WidgetEvent::Input(InputEvent::Mouse(MouseEvent {
                x,
                y,
                mouse_buttons,
                modifiers,
            })) => {
                return true;
            }
            _ => return false,
        }
    }
}

fn main() -> termwiz::Result<()> {
    let mut typed_text = String::new();

    {
        let mut red = Editor {
            buffers: Vec::new(),
            windows: Vec::new(),
            ui: Ui::new(),
        };
        let root_id = red.ui.set_root(Screen);
        let buffer_id = red.ui.add_child(root_id, TextInput::new(&mut typed_text));

        let caps = Capabilities::new_from_env()?;
        let mut term = new_terminal(caps)?;
        // term.enter_alternate_screen()?;
        term.set_raw_mode()?;

        let mut buf = BufferedTerminal::new(term)?;

        red.ui.render_to_screen(&mut buf)?;

        buf.flush()?;

        red.ui.add_child(root_id, StatusLine::new());

        red.ui.set_focus(buffer_id);

        loop {
            red.ui.process_event_queue()?;

            // After updating and processing all of the widgets, compose them
            // and render them to the screen.
            if red.ui.render_to_screen(&mut buf)? {
                // We have more events to process immediately; don't block waiting
                // for input below, but jump to the top of the loop to re-run the
                // updates.
                continue;
            }
            // Compute an optimized delta to apply to the terminal and display it
            buf.flush()?;

            // Wait for user input
            match buf.terminal().poll_input(None) {
                Ok(Some(InputEvent::Resized { rows, cols })) => {
                    // TODO: this is working around a bug where we don't realize
                    // that we should redraw everything on resize in BufferedTerminal.
                    buf.add_change(Change::ClearScreen(Default::default()));
                    buf.resize(cols, rows);
                }
                Ok(Some(input)) => match input {
                    InputEvent::Key(KeyEvent {
                        key: KeyCode::Char('q'),
                        modifiers: Modifiers::CTRL,
                    }) => {
                        // Quit the app when escape is pressed
                        buf.add_change(Change::ClearScreen(Default::default()));
                        buf.flush()?;
                        break;
                    }
                    input @ _ => {
                        // Feed input into the Ui
                        red.ui.queue_event(WidgetEvent::Input(input));
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    print!("{:?}\r\n", e);
                    break;
                }
            }
        }
    }

    // After we've stopped the full screen raw terminal,
    // print out the final edited value of the input text.
    println!("The text you entered: {}", typed_text);

    Ok(())
}
