pub struct Iter2D {
    max_x: usize,
    max_y: usize,
    x: usize,
    y: usize
}

impl Iter2D {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        Self { x: 0, y: 0, max_x, max_y }
    }
}

impl Iterator for Iter2D {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.max_y {
            return None;
        }

        let res = (self.x, self.y);
        
        self.x = (self.x + 1) % self.max_x;
        if self.x == 0 {
            self.y += 1;
        }

        Some(res)
    }
}

