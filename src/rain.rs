pub struct Rain {
    pub x: isize,
    pub y: isize,
    pub trail_length: isize,
    pub inv_velocity: isize,
    pub tick: isize
}

impl Rain {
    pub fn new(x: isize, trail_length: isize, inv_velocity: isize) -> Rain {
        Rain { x: x, y: 0, trail_length: trail_length, inv_velocity: inv_velocity, tick: 0 }
    }

    pub fn refresh(&mut self) {
        self.tick %= self.inv_velocity;
        self.tick += 1;
        let inc = if self.tick == self.inv_velocity { 1 } else { 0 };
        self.y += inc;
    }

    pub fn is_drawble(&self) -> bool {
        self.tick == self.inv_velocity
    }
}
