use crate::pos::*;

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct BitBoard {
    board: u64
}

impl BitBoard {
    pub fn empty() -> Self {
        BitBoard {
            board: 0
        }
    }
    
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.board == 0
    }

    #[inline(always)]
    pub fn piece_at(&self, pos: Pos) -> bool {
        (self.board >> pos.index()) & 1u64 == 1
    }
    
    pub fn iter(&self) -> BitBoardIterator {
        BitBoardIterator {
            board: *self,
            index: 0
        }
    }
    
    pub fn add(&mut self, pos: Pos) {
        self.board = self.with(pos).board;
    }
    
    pub fn remove(&mut self, pos: Pos) {
        self.board = self.without(pos).board;
    }

    pub fn with(&self, pos: Pos) -> BitBoard {
        let mask = 1u64 << pos.index();
        BitBoard {
            board: self.board | mask
        }
    }

    pub fn without(&self, pos: Pos) -> BitBoard {
        let mask = 1u64 << pos.index();
        BitBoard {
            board: self.board & !mask
        }
    }


    pub fn add_board(&mut self, board: BitBoard) {
        self.board = self.with_board(board).board;
    }

    pub fn with_board(&self, board: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board | board.board
        }
    }


    
    pub fn add_row(&mut self, row: usize) {
        self.board = self.with_row(row).board;
    }

    pub fn with_row(&self, row: usize) -> BitBoard {
        let mask = 0xFFu64 << (row * 8);
        BitBoard {
            board: self.board | mask
        }
    }
    
    
    pub fn add_col(&mut self, col: usize) {
        self.board = self.with_col(col).board;
    }

    pub fn with_col(&self, col: usize) -> BitBoard {
        let mut row = 0x01u64 << col;
        let mut mask = 0;
        for _ in 0..8 {
            mask = mask | row;
            row = row << 8;
        }
        BitBoard {
            board: self.board | mask
        }
    }
    
    
    pub fn intersect(&mut self, board: BitBoard) {
        self.board = self.intersection(board).board
    }

    pub fn intersection(&self, board: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board & board.board
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct BitBoardIterator {
    board: BitBoard,
    index: usize
}


impl BitBoardIterator {
    pub fn at_end(&self) -> bool {
        !(self.index < 64)
    }
    
    pub fn board(&self) -> BitBoard {
        self.board
    }
    
    pub fn pos(&self) -> Option<Pos> {
        if self.at_end() {
            None
        } else {
            Some(Pos::from_index(self.index))
        }
    }
}

impl Iterator for BitBoardIterator {
    type Item = Pos;
    
    fn next(&mut self) -> Option<Self::Item> {
        while !self.at_end() {
            let p = Pos::from_index(self.index);
            let item = self.board.piece_at(p);
            self.index += 1;
            if item {
                return Some(p)
            }
        }
        None
    }
}




impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   a b c d e f g h\n")?;
        write!(f, " +-----------------+\n")?;
        for y in 0..8  {
            let row = 7 - y;
            write!(f, "{}|", row + 1)?;
            for x in 0..8  {
                let bit = if self.piece_at(Pos::new(x, row)) {
                    1
                } else {
                    0
                };
                write!(f, " {}", bit)?;
            }
            write!(f, " |{}\n", row + 1)?;
        }
        write!(f, " +-----------------+\n")?;
        write!(f, "   a b c d e f g h\n")
    }
}