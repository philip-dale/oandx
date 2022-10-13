use super::MoveType;

pub struct Board {
    pub dim: usize,
    cells: Vec<Vec<MoveType>>,
}

pub enum HasWon {
    Yes(String),
    No,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            cells: vec![vec![String::from(" "); size]; size],
            dim: size,
        }
    }

    pub fn display(&self) {
        println!("----------");
        for r in 0..self.dim {
            for c in 0..self.dim
            {
                print!("{}",self.cells[r][c]);
                if c != self.dim-1 {
                    print!("|");
                }
            }
            println!("");
        }
        println!("----------");
    }

    pub fn add_move(&mut self, r: usize, c: usize, p: &MoveType) -> bool {
        if self.cells[r][c] == " " {
            self.cells[r][c] = (*p).clone();
            true
        } else {
            false
        }
    }

    pub fn has_won(&self) -> HasWon{
        //rows
        for r in 0..self.dim {
            let p = &self.cells[r][0];
            for c in 1..self.dim
            {
                if self.cells[r][c] != *p {
                    break
                }
                if c == self.dim-1 {
                    // if we get here all items in row are of value p
                    return HasWon::Yes((*p).clone());
                }
            }
        }
        // cols
        for c in 0..self.dim {
            let p = &self.cells[0][c];
            for r in 0..self.dim
            {
                if self.cells[r][c] != *p {
                    break
                }
                if r == self.dim-1 {
                    // if we get here all items in row are of value p
                    return HasWon::Yes((*p).clone());
                }
            }
        }
        // diag
        let p = &self.cells[0][0];
        for rc in 1..self.dim {
            if self.cells[rc][rc] != *p {
                break
            }
            if rc == self.dim-1 {
                // if we get here all items in dig are of value p
                return HasWon::Yes((*p).clone());
            }
        }
        let p = &self.cells[0][self.dim-1];
        for rc in 1..self.dim {
            if self.cells[rc][self.dim-1-rc] != *p {
                break
            }
            if rc == self.dim-1 {
                // if we get here all items in dig are of value p
                return HasWon::Yes((*p).clone());
            }
        }

        HasWon::No
    } 
}