use super::MoveType;

pub struct Players {
    count: usize,
    ids: Vec<MoveType>,
    current: usize,
}

impl Players {
    pub fn new(count: usize) -> Players {
        Players {
            count: count,
            ids: {
                let mut v: Vec<MoveType> = Vec::new();
                for p in 0..count {
                    v.push(p.to_string());
                }
                v
            },
            current: 0,
        }
    }

    pub fn get_player_and_next(&mut self) -> MoveType {
        let current = self.current;
        self.current += 1;
        if self.current == self.count {
            self.current = 0;
        }
        self.ids[current].clone()
    }

}