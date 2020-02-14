use std::fmt;
use std::str::FromStr;

type IndexType = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(pub Pos, pub Pos);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    i: IndexType
}


impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        debug_assert!(x < 8);
        debug_assert!(y < 8);
        Pos {
            i: (x + y * 8) as IndexType
        }
    }

    pub fn from_index(index: usize) -> Pos {
        debug_assert!(index < 64);
        Pos {
            i: index as IndexType
        }
    }

    pub fn index(&self) -> usize {
        self.i as usize
    }

    pub fn col(&self) -> usize {
        (self.i % 8) as usize
    }

    pub fn row(&self) -> usize {
        (self.i / 8) as usize
    }
}


impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.row()], self.col() + 1)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
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
                            return Ok(Pos::new(col, row));
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

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start();
        if s.len() < 4 {
        	Err(())
        } else {
	        match (Pos::from_str(&s[0..2]), Pos::from_str(&s[2..])) {
	            (Ok(a), Ok(b)) => Ok(Move(a, b)),
	            _ => Err(())
	        }
        }
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

#[test]
fn move_from_str() {
    assert_eq!(Move::from_str("a1b1"), Ok(Move(Pos::new(0, 0), Pos::new(1, 0))));
    assert!(Move::from_str("a1c").is_err());
    
}

