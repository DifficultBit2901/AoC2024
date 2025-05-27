#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    x: usize,
    y: usize,
    value: u8,
}

impl Node {
    pub fn new(value: u8, x: usize, y: usize) -> Self {
        Self { x, y, value }
    }

    pub fn get_valid_neighbors(&self, map: &Vec<Vec<u8>>) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            let n_val = map[self.y][self.x - 1];
            if self.value + 1 == n_val {
                neighbors.push(Self::new(n_val, self.x - 1, self.y));
            }
        }
        if self.y > 0 {
            let n_val = map[self.y - 1][self.x];
            if self.value +1 == n_val {
                neighbors.push(Self::new(n_val, self.x, self.y - 1));
            }
        }
        if self.x < map[0].len() - 1 {
            let n_val = map[self.y][self.x + 1];
            if self.value + 1 == n_val {
                neighbors.push(Self::new(n_val, self.x + 1, self.y));
            }
        }
        if self.y < map.len() - 1 {
            let n_val = map[self.y + 1][self.x];
            if self.value + 1 == n_val {
                neighbors.push(Self::new(n_val, self.x, self.y + 1));
            }
        }
        neighbors
    }
}
