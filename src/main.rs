//! Red - Rust EDitor
//! Author: Will Hopkins <willothyh@gmail.com>
//! License: MIT
//! Version: 0.1.0
//! Description: A simple text editor written in Rust.
//! Repository: https://github.com/willothy/red

use sanguine::{
    float::FloatStack, Align, Border, BorderVariant, Label, Layout, Rect, Stack, Ui, Widget,
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
            bordered![label!["Window 1!"].center() => Some(sanguine::SizeHint::Percentage(0.3))],
            bordered![label!["Window 2!"].center() => Some(sanguine::SizeHint::Percentage(0.7))],
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
    // let mut ui = layouts[layout];

    let mut ui = Ui::new(
        horizontal![
            bordered![label!["Window 1!"].center() => Some(sanguine::SizeHint::Percentage(0.3))],
            bordered![label!["Window 2!"].center() => Some(sanguine::SizeHint::Percentage(0.7))],
            => None
        ],
        buf,
    )?;

    let float_x = 60;
    let float_y = 20;
    let float_width = 20;
    let float_height = 10;
    ui.init()?;
    ui.add_float(sanguine::float::Float {
        contents: bordered![label!["Test"] => None],
        rect: Rect {
            x: 10.,
            y: 10.,
            width: 15.,
            height: 10.,
        },
        z_index: 1,
    });
    ui.add_float(sanguine::float::Float {
        contents: bordered![label!["Test"] => None],
        rect: Rect {
            x: float_x as f64,
            y: float_y as f64,
            width: float_width as f64,
            height: float_height as f64,
        },
        z_index: 0,
    });

    while ui.render()? {}
    // let mut main_float = floats.add(sanguine::float::Float {
    //     contents: bordered![label!["Test"] => None],
    //     rect: Rect {
    //         x: float_x as f64,
    //         y: float_y as f64,
    //         width: float_width as f64,
    //         height: float_height as f64,
    //     },
    //     z_index: 0,
    // });
    // floats.add(sanguine::float::Float {
    //     contents: bordered![label!["Test"] => None],
    //     rect: Rect {
    //         x: 10.,
    //         y: 10.,
    //         width: 15.,
    //         height: 10.,
    //     },
    //     z_index: 1,
    // });
    //
    // let mut cycle_float = |main_float, nfloats| {
    //     return if main_float + 1 == nfloats {
    //         0
    //     } else {
    //         main_float + 1
    //     };
    // };

    Ok(())
}
