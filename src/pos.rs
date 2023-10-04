use std::fmt;
use std::str::FromStr;


type IndexType = u8;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    i: IndexType
}


impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        debug_assert!(x >= 0 && x < 8);
        debug_assert!(y >= 0 && y < 8);
        Pos {
            i: (x * 8 + y) as IndexType
        }
    }

    pub fn try_new(x: i32, y: i32) -> Option<Pos> {
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            Some(Pos::new(x, y))
        } else {
            None
        }
    }

    pub fn from_index(index: usize) -> Pos {
        debug_assert!(index < 64);
        Pos {
            i: index as IndexType
        }
    }

    pub fn index(&self) -> usize {
        self.i as _
    }

    pub fn col(&self) -> i32 {
        (self.i / 8) as _
    }

    pub fn row(&self) -> i32 {
        (self.i % 8) as _
    }

    pub fn file_symbol(&self) -> char {
        ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.col() as usize]
    }
}


impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file_symbol(), self.row() + 1)
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();
        let c = chars.next();
        let r = chars.next();
        match (c, r) {
            (Some(c), Some(r)) => {
                if let Some(col) = "abcdefgh".find(c) {
                    if let Some(row) = "12345678".find(r) {
                        if chars.next().is_none() {
                            return Ok(Pos::new(col as _, row as _));
                        }
                    }
                }
            }
            
            _ => {
            }
        }
        
        Err(())
    }
}



#[test]
fn pos_from_str() {
    assert_eq!(Pos::from_str("a1"), Ok(Pos::new(0, 0)));
    assert_eq!(Pos::from_str("h8"), Ok(Pos::new(7, 7)));
    assert!(Pos::from_str("d4c").is_err());
    assert!(Pos::from_str("j7").is_err());
    assert!(Pos::from_str("e0").is_err());
    
}
