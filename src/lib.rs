#[macro_use]
extern crate seed;
use seed::prelude::*;

mod sudoku;
use sudoku::{Board, Cell};

// Model
struct Model {
    pub board: Board,
    pub warning: String,
    pub selected: Option<(usize, usize)>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            board: Board::new(9),
            warning: String::new(),
            selected: None,
        }
    }
}

// Update
#[derive(Clone)]
enum Msg {
    Solve,
    Clear,
    Select(usize, usize),
    CellUpdate(String),
    KeyDown(web_sys::KeyboardEvent),
}

fn update_cell(model: &Model, value: Option<u8>) -> Board {
    if let Some((x, y)) = model.selected {
        match value {
            Some(v) => model.board.set(x, y, Cell::Constant(v)),
            None => model.board.set(x, y, Cell::Empty),
        }
    } else {
        model.board.clone()
    }
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Solve => {
            model.warning = String::new();
            match model.board.solve() {
                Some(board) => model.board = board,
                None => model.warning = "This Sudoku is unsolvable!".to_string(),
            };
            model.selected = None;
        }
        Msg::Clear => {
            model.warning = String::new();
            model.board = Board::new(9);
            model.selected = None;
        }
        Msg::Select(x, y) => {
            if model.selected == Some((x, y)) {
                model.selected = None;
            } else {
                model.selected = Some((x, y));
            }
        }
        Msg::CellUpdate(s) => {
            if let Ok(v) = s.parse::<u8>() {
                if v > 0 && v as usize <= model.board.n {
                    model.board = update_cell(model, Some(v));
                }
            } else if s == "" {
                model.board = update_cell(model, None);
            }
        }
        Msg::KeyDown(key_event) => {
            let key = key_event.key();
            if let Ok(v) = key.parse::<u8>() {
                if v > 0 && v as usize <= model.board.n {
                    model.board = update_cell(model, Some(v));
                }
            } else if key == "Backspace" || key == "Delete" {
                model.board = update_cell(model, None);
            }
        }
    }
    Render.into()
}

// View
fn row(cells: &[Cell], y: usize, selected: Option<usize>) -> El<Msg> {
    let cells = cells
        .iter()
        .enumerate()
        .map(|(x, cell)| {
            let mut classes = "cell".to_string();
            if let Some(s) = selected {
                if s == x {
                    classes += " selected";
                }
            };
            if let Cell::Constant(_) = cell {
                classes += " constant";
            }

            let text = match cell {
                Cell::Variable(v) | Cell::Constant(v) => format!("{}", v),
                Cell::Empty => String::new(),
            };

            td![
                simple_ev(Ev::Click, Msg::Select(x, y)),
                class![classes.as_str()],
                text
            ]
        })
        .collect::<Vec<El<Msg>>>();
    tr![class!["row"], cells]
}

fn board(cells: &[Cell], n: usize, selected: Option<(usize, usize)>) -> El<Msg> {
    let rows = cells
        .chunks(n)
        .enumerate()
        .map(|(y, chunk)| {
            let select = match selected {
                Some(s) => {
                    if s.1 == y {
                        Some(s.0)
                    } else {
                        None
                    }
                }
                None => None,
            };
            row(chunk, y, select)
        })
        .collect::<Vec<El<Msg>>>();
    table![class!["board"], rows]
}

fn view(model: &Model) -> El<Msg> {
    let squares = &model.board.squares;
    let n = model.board.n;

    let input_value = match model.selected {
        Some((x, y)) => match model.board.get(x, y) {
            Cell::Variable(v) | Cell::Constant(v) => format!("{}", v),
            Cell::Empty => String::new(),
        },
        None => String::new(),
    };
    let input_field: El<Msg> = input![
        class!["input_field"],
        attrs! {
            At::Value => input_value;
            At::AutoFocus => true
        },
        input_ev(Ev::Input, Msg::CellUpdate)
    ];

    div![
        class!["container"],
        div![
            board(squares, n, model.selected),
            button![
                class!["solve_button"],
                simple_ev(Ev::Click, Msg::Solve),
                format!("Solve")
            ],
            button![
                class!["clear_button"],
                simple_ev(Ev::Click, Msg::Clear),
                format!("Clear")
            ],
            input_field,
            p![class!["warning_text"], model.warning]
        ]
    ]
}

fn window_events(_: &Model) -> Vec<seed::dom_types::Listener<Msg>> {
    vec![keyboard_ev("keydown", Msg::KeyDown)]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .window_events(window_events)
        .finish()
        .run();
}
