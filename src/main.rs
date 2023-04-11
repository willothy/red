//! Red - Rust EDitor
//! Author: Will Hopkins <willothyh@gmail.com>
//! License: MIT
//! Version: 0.1.0
//! Description: A simple text editor written in Rust.
//! Repository: https://github.com/willothy/red

use red_tui::{Align, Border, BorderVariant, Label, Layout, Stack, Widget};
use termwiz::{
    caps::Capabilities,
    input::{InputEvent, KeyCode, KeyEvent, Modifiers, MouseEvent},
    surface::Change,
    terminal::{buffered::BufferedTerminal, new_terminal, Terminal},
    Result,
};

fn main() -> Result<()> {
    let caps = Capabilities::new_from_env()?;
    let mut term = new_terminal(caps)?;
    // term.enter_alternate_screen()?;
    term.set_raw_mode()?;

    let mut buf = BufferedTerminal::new(term)?;

    buf.flush()?;

    macro_rules! horizontal {
        [$($v:expr),*$(,)?] => {
            Layout::h(vec![
                $($v),*
            ])
        };
    }

    macro_rules! vertical {
        [$($v:expr),*$(,)?] => {
            Layout::v(vec![
                $($v),*
            ])
        };
    }

    macro_rules! label {
        [$($v:expr),+$(,)?] => {
            Label::new(&format!($($v),+))
        };
    }

    macro_rules! bordered {
        ($v:expr) => {
            Box::new(Border::new(BorderVariant::Rounded, $v))
        };
    }

    let layer1 = vertical![
        horizontal![
            label!["Hello, world 1!"].center(),
            label!["Hello, world 2!"].center(),
            label!["Hello, world 3!"].center()
        ],
        horizontal![
            label![""].center(),
            // label!["Hello, world 5!"].center(),
            // label!["Hello, world 6!"].center()
        ],
    ];
    let layer2 = vertical![
        horizontal![
            bordered![label!["Hello, world 7!"].center()],
            bordered![label!["Hello, world 8!"].center()],
            bordered![label!["Hello, world 9!"].center()]
        ],
        horizontal![
            bordered![label!["Hello, world 10!"].center()],
            bordered![label!["Hello, world 11!"].center()],
        ],
    ];

    let stack = Stack::new().with_layer(0, layer1).with_layer(1, layer2);
    loop {
        let screen = buf.terminal().get_screen_size()?;
        let rect = red_tui::Rect {
            x: 0.,
            y: 0.,
            width: screen.cols as f64,
            height: screen.rows as f64,
        };
        buf.add_change(Change::ClearScreen(Default::default()));
        stack.render(&rect, &mut buf);

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
                #[allow(unused_variables)]
                InputEvent::Mouse(MouseEvent {
                    x,
                    y,
                    mouse_buttons,
                    modifiers,
                }) => {
                    // Hacky fix for mouse events registering one row too low
                    let y = y - 1;
                    // TODO: Feed input into the Ui
                    // Get widget under mouse
                    // Send input to widget
                }
                _input @ _ => {
                    // TODO: Feed input into the Ui
                    // Get focused widget
                    // Send input to widget
                }
            },
            Ok(None) => {}
            Err(e) => {
                print!("{:?}\r\n", e);
                break;
            }
        }
    }

    Ok(())
}
