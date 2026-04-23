#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

impl Pose {
    pub(crate) fn forward(&mut self) {
        match self.heading {
            'E' => self.x += 1,
            'S' => self.y -= 1,
            'W' => self.x -= 1,
            'N' => self.y += 1,
            _ => (),
        }
    }

    /// 沿当前朝向后退一格（与 forward 方向相反）
    pub(crate) fn backward(&mut self) {
        match self.heading {
            'E' => self.x -= 1,
            'S' => self.y += 1,
            'W' => self.x += 1,
            'N' => self.y -= 1,
            _ => (),
        }
    }
    pub(crate) fn turn_left(&mut self) {
        match self.heading {
            'E' => self.heading = 'N',
            'S' => self.heading = 'E',
            'W' => self.heading = 'S',
            'N' => self.heading = 'W',
            _ => (),
        }
    }
    pub(crate) fn turn_right(&mut self) {
        match self.heading {
            'E' => self.heading = 'S',
            'S' => self.heading = 'W',
            'W' => self.heading = 'N',
            'N' => self.heading = 'E',
            _ => (),
        }
    }
}
