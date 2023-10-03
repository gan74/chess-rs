use crate::pos::*;

use std::fmt;
use std::ops;



#[derive(Debug, Clone, Copy)]
pub struct BitBoard {
    board: u64,
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
    pub fn contains(&self, pos: Pos) -> bool {
        (self.board >> pos.index()) & 1u64 == 1
    }



    pub fn shift_left(&self) -> BitBoard {
        BitBoard {
            board: self.board >> 8,
        }
    }

    pub fn shift_right(&self) -> BitBoard {
        BitBoard {
            board: self.board << 8,
        }
    }

    pub fn intersect(&self, board: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board & board.board
        }
    }

    pub fn bit_count(&self) -> usize {
        self.board.count_ones() as _
    }

    pub fn set_positions(&self) -> BitBoardIterator {
        BitBoardIterator {
            board: self.board,
        }
    }
}




pub struct BitBoardIterator {
    board: u64,
}

impl Iterator for BitBoardIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.board.trailing_zeros();
        if i == 64 {
            None
        } else {
            self.board = self.board ^ (1u64 << i);
            Some(Pos::from_index(i as _))
        }
    }
}



impl<T: Iterator<Item = Pos>> From<T> for BitBoard {
    fn from(value: T) -> Self {
        let mut board = BitBoard::empty();
        for p in value {
            board += p;
        }
        board
    }
}


impl ops::Add<BitBoard> for BitBoard {
    type Output = BitBoard;
    fn add(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board | rhs.board,
        }
    }
}

impl ops::AddAssign<BitBoard> for BitBoard {
    fn add_assign(&mut self, rhs: BitBoard) {
        self.board = self.board | rhs.board;
    }
}

impl ops::Add<Pos> for BitBoard {
    type Output = BitBoard;
    fn add(self, rhs: Pos) -> BitBoard {
        let mask = 1u64 << rhs.index();
        BitBoard {
            board: self.board | mask,
        }
    }
}

impl ops::AddAssign<Pos> for BitBoard {
    fn add_assign(&mut self, rhs: Pos) {
        let mask = 1u64 << rhs.index();
        self.board = self.board | mask;
    }
}




impl ops::Sub<BitBoard> for BitBoard {
    type Output = BitBoard;
    fn sub(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            board: self.board & !rhs.board,
        }
    }
}

impl ops::SubAssign<BitBoard> for BitBoard {
    fn sub_assign(&mut self, rhs: BitBoard) {
        self.board = self.board & !rhs.board;
    }
}

impl ops::Sub<Pos> for BitBoard {
    type Output = BitBoard;
    fn sub(self, rhs: Pos) -> BitBoard {
        let mask = 1u64 << rhs.index();
        BitBoard {
            board: self.board & !mask,
        }
    }
}

impl ops::SubAssign<Pos> for BitBoard {
    fn sub_assign(&mut self, rhs: Pos) {
        let mask = 1u64 << rhs.index();
        self.board = self.board & !mask;
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
                let bit = if self.contains(Pos::new(x, row)) {
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