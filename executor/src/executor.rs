use crate::pose::Pose;

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'M' => self.pose.forward(),
                'L' => self.pose.turn_left(),
                'R' => self.pose.turn_right(),
                _ => (),
            }
        }
    }
    /// 向当前朝向前进一格
    fn forward(&mut self) {
        match self.pose.heading {
            'E' => self.pose.x += 1,
            'S' => self.pose.y -= 1,
            'W' => self.pose.x -= 1,
            'N' => self.pose.y += 1,
            _ => (),
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}

