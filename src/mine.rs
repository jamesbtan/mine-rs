use std::collections::HashSet;
use iced::widget::{button, text, Row, Column};

#[derive(Debug, Clone)]
pub enum Message {
    Click(usize),
    // Flag(usize),
    // Chord(usize),
}

pub struct Board {
    width: usize,
    height: usize,
    nummines: usize,
    // coords are of form i + j*width - 0,0 = top-left
    // store locations of mines
    mines: HashSet<usize>,
    // 0-8 for number of mines around it, 9 for unopened, 10 for flagged
    board: Vec<usize>,
}

impl Board {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Click(c) => {
                if self.board[c] != 9 {
                    return;
                }
                if self.mines.contains(&c) {
                    // TODO game over
                    return;
                }
                let mut cnt = 0;
                for i in 0..=2 {
                    for j in 0..=2 {
                        if (i == 1) & (j == 1) { continue; }
                        let mut a = 0;
                        let mut b = c;
                        if i == 0 {
                            a += 1;
                        } else {
                            b += i-1;
                        }
                        if j == 0 {
                            a += self.width;
                        } else {
                            b += (j-1)*self.width;
                        }
                        if a > b {
                            continue;
                        } else if self.mines.contains(&(b - a)) {
                            cnt += 1;
                        }
                    }
                }
                self.board[c] = cnt;
            },
        }
    }

    pub fn view(&self) -> Column<Message> {
        let mut c = Column::new();
        for j in 0..self.height {
            let mut r = Row::new();
            for i in 0..self.width {
                let ind = i + j*self.width;
                if self.board[ind] == 9 {
                    r = r.push(button("").on_press(Message::Click(ind)));
                } else {
                    r = r.push(text(self.board[ind]));
                }
            }
            c = c.push(r);
        }
        c
    }

    fn new(width: usize, height: usize, nummines: usize) -> Self {
        let mut ret = Self {
            width,
            height,
            nummines,
            mines: HashSet::new(),
            board: Vec::with_capacity(width*height),
        };
        for _ in 0..nummines {
            // TODO insert random mine
        }
        for _ in 0..width*height {
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
