//! Red - Rust EDitor
//! Author: Will Hopkins <willothyh@gmail.com>
//! License: MIT
//! Version: 0.1.0
//! Description: A simple text editor written in Rust.
//! Repository: https://github.com/willothy/red

use red_tui::{Align, Border, BorderVariant, Label, Layout, Rect, Stack, Widget};
use termwiz::{
    caps::Capabilities,
    input::{InputEvent, KeyCode, KeyEvent, Modifiers, MouseEvent},
    surface::{Change, Surface},
    terminal::{buffered::BufferedTerminal, new_terminal, Terminal},
    Result,
};

struct RenderData {
    rect: Rect,
    data: Vec<Vec<char>>,
}

impl RenderData {
    fn new(widget: Box<dyn Widget>, rect: Rect) -> Self {
        Self {
            data: vec![vec![' '; rect.width as usize]; rect.height as usize],
            rect,
        }
    }

    fn render(&mut self, widget: Box<dyn Widget>) {
        let mut term = Surface::new(self.rect.width as usize, self.rect.height as usize);
        widget.render(&self.rect, &mut term);
    }
}

fn main() -> Result<()> {
    let caps = Capabilities::new_from_env()?;
    let mut term = new_terminal(caps)?;
    // term.enter_alternate_screen()?;
    term.set_raw_mode()?;

    let mut buf = BufferedTerminal::new(term)?;
    buf.add_change(Change::CursorVisibility(
        termwiz::surface::CursorVisibility::Hidden,
    ));

    buf.flush()?;

    macro_rules! horizontal {
        [$($v:expr),*$(,)? => $s:expr] => {
            Layout::h(vec![
                $($v),*
            ], $s)
        };
    }

    macro_rules! vertical {
        [$($v:expr),*$(,)?=> $s:expr] => {
            Layout::v(vec![
                $($v),*
            ], $s)
        };
    }

    macro_rules! label {
        [$($v:expr),+$(,)?] => {
            Label::new(&format!($($v),+))
        };
    }

    macro_rules! bordered {
        ($v:expr => $s:expr) => {
            Box::new(Border::new(BorderVariant::Rounded, $v, $s))
        };
    }

    let layouts = vec![
        horizontal![
            bordered![label!["Window 1!"].center() => Some(red_tui::SizeHint::Percentage(0.3))],
            bordered![label!["Window 2!"].center() => Some(red_tui::SizeHint::Percentage(0.7))],
            => None
        ],
        vertical![
            bordered![label!["Window 1!"].center() => None],
            bordered![label!["Window 2!"].center() => None],
            => None
        ],
        horizontal![
            vertical![
                bordered![label!["Window 1!"].center() => None],
                => None
            ],
            vertical![
                bordered![label!["Window 2!"].center() => None],
                bordered![label!["Window 3!"].center() => None],
                bordered![label!["Window 4!"].center() => None],
                => None
            ],
            => None
        ],
        horizontal![
            vertical![
                bordered![label!["Window 1!"].center() => None],
                bordered![label!["Window 2!"].center() => None],
                => None
            ],
            vertical![
                bordered![label!["Window 3!"].center() => None],
                => None
            ],
            => None
        ],
    ];

    let mut layout = 0;
    let mut ui = &layouts[layout];

    let mut cycle_layout = || {
        layout = if layout + 1 == layouts.len() {
            0
        } else {
            layout + 1
        };
        return &layouts[layout];
    };

    loop {
        let screen = buf.terminal().get_screen_size()?;
        let rect = red_tui::Rect {
            x: 0.,
            y: 0.,
            width: screen.cols as f64,
            height: screen.rows as f64,
        };
        buf.add_change(Change::ClearScreen(Default::default()));
        ui.render(&rect, &mut buf);

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
                    key: KeyCode::Char(c),
                    modifiers,
                }) => {
                    if c == 'q' && modifiers == Modifiers::CTRL {
                        // Quit the app when q is pressed
                        buf.add_change(Change::ClearScreen(Default::default()));
                        buf.add_change(Change::CursorVisibility(
                            termwiz::surface::CursorVisibility::Visible,
                        ));
                        buf.flush()?;
                        break;
                    }
                }
                InputEvent::Key(KeyEvent {
                    key: KeyCode::Tab, ..
                }) => {
                    ui = cycle_layout();
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
