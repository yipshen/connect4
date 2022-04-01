use std::collections::HashSet;

use cursive::{
    direction::Direction,
    event::{Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle},
    view::CannotFocus,
    views::{Button, Dialog, LinearLayout, Panel, SelectView},
    Cursive, Printer, Vec2
};

mod game;
mod player;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("Connect 4")
            .padding_lrtb(2, 2, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("   New game   ", show_options))
                    .child(Button::new_raw("     Exit     ", |s| s.quit())),
            ),
    );

    siv.run();
}

fn show_options(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Select game")
            .content(
                SelectView::new()
                    .item(
                        "Classic                    -",
                        game::Options {
                            rules: game::RulesVariation::Classic,
                        },
                    )
                    .on_submit(|s, options| {
                        s.pop_layer();
                        new_game(s, *options);
                    }),
            )
            .dismiss_button("Back"),
    );
}

struct BoardView {
    board: game::Board,
    overlay: Vec<game::Token>,
    focus: usize,
    winner: game::Token,
    winner_plays: HashSet<(usize, usize)>,
}

impl BoardView {
    fn new(options: game::Options) -> Self {
        let board = game::Board::new(options.rules);
        let overlay = vec![game::Token::Invalid; board.cols * board.rows];

        BoardView {
            board: board,
            overlay: overlay,
            focus: 0,
            winner: game::Token::Invalid,
            winner_plays: HashSet::new(),
        }
    }

    pub fn cell_id(&self, col: usize, row: usize) -> usize {
        row * self.board.cols + col
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for i in 0..self.board.cols {
            let color = if !self.winner.is_valid() && i == self.focus {
                match self.board.current_token {
                    game::Token::Invalid => Color::RgbLowRes(3, 3, 3),
                    game::Token::Red => Color::RgbLowRes(5, 0, 0),
                    game::Token::Yellow => Color::RgbLowRes(5, 5, 0),
                    }
                } else {
                    Color::RgbLowRes(3, 3, 3)
            };

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((i * 3, 0), " "),
            );
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), color),
                |printer| printer.print((i * 3 + 1, 0), " "),
            );
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((i * 3 + 2, 0), " "),
            );
        }

        for (i, cell) in self.overlay.iter().enumerate() {
            let col = (i % self.board.cols) * 3;
            let row = self.board.rows - i / self.board.cols;

            let text = if self.winner_plays.contains(&(i % self.board.cols, i / self.board.cols)) { "*" } else { " " };

            let color = match *cell {
                game::Token::Invalid => Color::RgbLowRes(2, 2, 2),
                game::Token::Red => Color::RgbLowRes(5, 0, 0),
                game::Token::Yellow => Color::RgbLowRes(5, 5, 0),
            };

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((col, row), " "),
            );
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), color),
                |printer| printer.print((col + 1, row), text),
            );
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((col + 2, row), " "),
            );
        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if self.winner != game::Token::Invalid {
            return EventResult::Ignored;
        }

        match event {
            Event::Key(Key::Left) => {
                if self.focus > 0 {
                    self.focus -= 1;
                }
                return EventResult::Consumed(None);
            },
            Event::Key(Key::Right) => {
                if self.focus < self.board.cols - 1 {
                    self.focus += 1;
                }
                return EventResult::Consumed(None);
            },
            Event::Char(' ') => {
                match self.board.drop(self.focus) {
                    Ok(_) => {
                        if let Some(winner_plays) = self.board.check_win() {
                            self.winner = self.board.current_token;
                            self.winner_plays = HashSet::from_iter(winner_plays.iter().cloned());
                        }
                        let cell_id = self.cell_id(self.focus, self.board.find_row_for_col(self.focus).unwrap());
                        self.overlay[cell_id] = self.board.current_token;
                        self.board.switch_token();

                        return EventResult::Consumed(None);
                    },
                    Err(_) => (),
                }
            },
            _ => (),
        }

        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(self.board.cols, self.board.rows + 1).map_x(|x| 3 * x)
    }
}

fn new_game(siv: &mut Cursive, options: game::Options) {
    siv.add_layer(
        Dialog::new()
            .title("Connect 4")
            .content(
                LinearLayout::horizontal()
                    .child(Panel::new(BoardView::new(options))),
            )
            .button("Quit game", |s| {
                s.pop_layer();
            }),
    );
}
