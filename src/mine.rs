use iced::border::Border;
use iced::widget::{button, center, container, mouse_area, row, text, Column, Row};
use iced::window;
use iced::Size;
use iced::Color;
use iced::Length;
use rand::Rng;
use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Message {
    Click(usize),
    Flag(usize),
    Hover(usize),
    // HACK I don't like this
    Unhover(usize),
    // NOTE should we merge with Click?
    Chord(usize),
    TimeTick,
    GameOver(bool),
    Replay,
}

pub struct Board {
    width: usize,
    height: usize,
    nummines: usize,
    numflag: usize,
    numclose: usize,
    time: usize,
    playing: bool,
    hover: usize,
    unhover: usize,
    // coords are of form i + j*width - 0,0 = top-left
    // store locations of mines
    mines: HashSet<usize>,
    // 0-8 for number of mines around it, 9 for unopened, 10 for flagged, 11 for bomb
    board: Vec<usize>,
}

impl Board {
    fn open(&mut self, ind: usize) {
        if self.board[ind] != 9 || self.mines.contains(&ind) {
            return;
        }
        let mut cnt = 0;
        let i = ind % self.width;
        let j = ind / self.width;
        let si = max(i, 1) - 1;
        let ei = min(i, self.width - 2) + 1;
        let sj = max(j, 1) - 1;
        let ej = min(j, self.height - 2) + 1;
        for x in si..=ei {
            for y in sj..=ej {
                if (x == i) & (y == j) {
                    continue;
                }
                if self.mines.contains(&(x + y * self.width)) {
                    cnt += 1;
                }
            }
        }
        self.board[ind] = cnt;
        self.numclose -= 1;
        if cnt == 0 {
            let i = ind % self.width;
            let j = ind / self.width;
            let si = max(i, 1) - 1;
            let ei = min(i, self.width - 2) + 1;
            let sj = max(j, 1) - 1;
            let ej = min(j, self.height - 2) + 1;
            for x in si..=ei {
                for y in sj..=ej {
                    if (x == i) & (y == j) {
                        continue;
                    }
                    self.open(x + y * self.width);
                }
            }
        }
    }
    pub fn update(&mut self, message: Message) {
        if !self.playing {
            if matches!(message, Message::Replay) {
                *self = Self::new(self.width, self.height, self.nummines);
            }
            return;
        }
        match message {
            Message::GameOver(b) => {
                self.playing = false;
                if b {
                    println!("Won");
                } else {
                    println!("Lost");
                    for mine in &self.mines {
                        self.board[*mine] = 11;
                    }
                }
            }
            Message::Click(c) => {
                if self.mines.contains(&c) {
                    self.board[c] = 11;
                    self.update(Message::GameOver(false));
                    return;
                }
                self.open(c);
                if self.numclose == self.nummines {
                    self.update(Message::GameOver(true));
                }
            }
            Message::Flag(c) => {
                if self.board[c] == 9 {
                    self.board[c] = 10;
                    self.numflag += 1;
                } else if self.board[c] == 10 {
                    self.board[c] = 9;
                    self.numflag -= 1;
                }
            }
            Message::Hover(c) => {
                self.hover = c;
                self.unhover = c + 1;
            }
            Message::Unhover(c) => {
                self.unhover = c;
            }
            Message::Chord(c) => {
                let i = c % self.width;
                let j = c / self.width;
                let si = max(i, 1) - 1;
                let ei = min(i, self.width - 2) + 1;
                let sj = max(j, 1) - 1;
                let ej = min(j, self.height - 2) + 1;
                let mut fl = 0;
                for x in si..=ei {
                    for y in sj..=ej {
                        if (x == i) & (y == j) {
                            continue;
                        }
                        if self.board[x + y * self.width] == 10 {
                            fl += 1;
                        }
                    }
                }
                if fl == self.board[c] {
                    for x in si..=ei {
                        for y in sj..=ej {
                            if (x == i) & (y == j) {
                                continue;
                            }
                            self.open(x + y * self.width);
                        }
                    }
                }
                if self.numclose == self.nummines {
                    self.update(Message::GameOver(true));
                }
            }
            Message::TimeTick => {
                self.time += 1;
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        // HACK ugly
        let mut c = Column::new();
        c = c.push(
            row![
                text!("{}", self.nummines - self.numflag)
                    .width(Length::Fill)
                    .center(),
                text!("{}", self.time).width(Length::Fill).center()
            ]
            .width(Length::Fixed(self.width as f32 * 50.0)),
        );
        for j in 0..self.height {
            let mut r = Row::new();
            for i in 0..self.width {
                let ind = i + j * self.width;
                r = match self.board[ind] {
                    9 => r.push(
                        mouse_area(container("").width(50).height(50).style(move |_theme| {
                            container::Style {
                                background: Some(
                                    Color::new(
                                        0.0,
                                        0.0,
                                        1.0,
                                        if self.hover == ind && self.unhover != ind {
                                            0.8
                                        } else {
                                            1.0
                                        },
                                    )
                                    .into(),
                                ),
                                border: Border {
                                    color: Color::new(0.0, 0.0, 0.8, 0.8),
                                    width: 1.0,
                                    ..Border::default()
                                },
                                ..container::Style::default()
                            }
                        }))
                        .on_press(Message::Click(ind))
                        .on_right_press(Message::Flag(ind))
                        .on_enter(Message::Hover(ind))
                        .on_exit(Message::Unhover(ind)),
                    ),
                    10 => r.push(
                        mouse_area(container("").width(50).height(50).style(move |_theme| {
                            container::Style {
                                background: Some(
                                    Color::new(
                                        1.0,
                                        0.0,
                                        0.0,
                                        if self.hover == ind && self.unhover != ind {
                                            0.8
                                        } else {
                                            1.0
                                        },
                                    )
                                    .into(),
                                ),
                                ..container::Style::default()
                            }
                        }))
                        .on_right_press(Message::Flag(ind))
                        .on_enter(Message::Hover(ind))
                        .on_exit(Message::Unhover(ind)),
                    ),
                    11 => r.push(center(text("B")).width(50).height(50)),
                    _ => r.push(
                        mouse_area(center(text(self.board[ind])).width(50).height(50))
                            .on_press(Message::Chord(ind)),
                    ),
                }
            }
            c = c.push(r);
        }
        if !self.playing {
            c = c.push(button("Play again").on_press(Message::Replay));
        }
        c
    }

    fn new(width: usize, height: usize, nummines: usize) -> Self {
        // TODO how to resize?
        // iced::window::get_latest().map(move |i| window::resize::<()>(i.expect("Must have id"), Size::new((50*width) as f32, (50*height+100) as f32)));
        // NOTE are all of these fields needed?
        let mut ret = Self {
            width,
            height,
            nummines,
            numflag: 0,
            hover: 0,
            unhover: 0,
            time: 0,
            numclose: width * height,
            playing: true,
            mines: HashSet::new(),
            board: Vec::with_capacity(width * height),
        };
        let mut rng = rand::thread_rng();
        while ret.mines.len() < nummines {
            ret.mines.insert(rng.gen_range(0..width * height));
        }
        for _ in 0..width * height {
            ret.board.push(9);
        }
        ret
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(10, 8, 10)
    }
}
