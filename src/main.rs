//! Red - Rust EDitor
//! Author: Will Hopkins <willothyh@gmail.com>
//! License: MIT
//! Version: 0.1.0
//! Description: A simple text editor written in Rust.
//! Repository: https://github.com/willothy/red

use red_tui::{
    float::FloatStack, Align, Border, BorderVariant, Label, Layout, Rect, Stack, Widget,
};
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

    let mut float_width = 50;
    let mut float_height = 25;
    let mut float_x = 80;
    let mut float_y = 20;

    let mut floats = FloatStack::new();
    let mut main_float = floats.add(red_tui::float::Float {
        contents: bordered![label!["Test"] => None],
        rect: Rect {
            x: float_x as f64,
            y: float_y as f64,
            width: float_width as f64,
            height: float_height as f64,
        },
        z_index: 0,
    });
    floats.add(red_tui::float::Float {
        contents: bordered![label!["Test"] => None],
        rect: Rect {
            x: 10.,
            y: 10.,
            width: 15.,
            height: 10.,
        },
        z_index: 1,
    });

    let mut cycle_float = |main_float, nfloats| {
        return if main_float + 1 == nfloats {
            0
        } else {
            main_float + 1
        };
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

        floats.update(
            main_float,
            Rect {
                x: float_x as f64,
                y: float_y as f64,
                width: float_width as f64,
                height: float_height as f64,
            },
        );
        floats.render(&rect, &mut buf);

        /* This is how to render a floating window! */

        // TODO: Implement floats, maybe as HashMap<z-index, Vec<Box<dyn Widget>>>?
        /*
        let l = bordered![label!["Test"] => None];
        let mut l_surf = Surface::new(float_width, float_height);
        l.render(
            &Rect {
                x: 0.,
                y: 0.,
                width: float_width as f64,
                height: float_height as f64,
            },
            &mut l_surf,
        );
        buf.draw_from_screen(&l_surf, float_x, float_y);
        */

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
                    key: KeyCode::UpArrow,
                    modifiers,
                }) => {
                    if modifiers == Modifiers::SHIFT {
                        float_height -= 1;
                    } else {
                        float_y -= 1;
                    }
                }
                InputEvent::Key(KeyEvent {
                    key: KeyCode::DownArrow,
                    modifiers,
                }) => {
                    if modifiers == Modifiers::SHIFT {
                        float_height += 1;
                    } else {
                        float_y += 1;
                    }
                }
                InputEvent::Key(KeyEvent {
                    key: KeyCode::LeftArrow,
                    modifiers,
                }) => {
                    if modifiers == Modifiers::SHIFT {
                        float_width -= 1;
                    } else {
                        float_x -= 1;
                    }
                }
                InputEvent::Key(KeyEvent {
                    key: KeyCode::RightArrow,
                    modifiers,
                }) => {
                    if modifiers == Modifiers::SHIFT {
                        float_width += 1;
                    } else {
                        float_x += 1;
                    }
                }
                InputEvent::Key(KeyEvent {
                    key: KeyCode::Tab,
                    modifiers,
                }) => {
                    if modifiers == Modifiers::SHIFT {
                        let t = cycle_float(main_float, floats.floats.len());
                        let rect = &floats.floats[&t].rect;
                        float_x = rect.x as usize;
                        float_y = rect.y as usize;
                        float_width = rect.width as usize;
                        float_height = rect.height as usize;
                        // floats.update_z_index(main_float, );
                        main_float = t;
                    } else {
                        ui = cycle_layout();
                    }
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
