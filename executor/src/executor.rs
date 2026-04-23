use crate::pose::Pose;
use crate::state::State;

pub struct Executor {
    pose: Pose,
    /// 当前行驶状态，初始为正常状态
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: State::Normal,
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                // B 指令：切换倒车状态
                'B' => self.state.toggle_reverse(),
                'M' => self.pose.forward(),
                'L' => self.pose.turn_left(),
                'R' => self.pose.turn_right(),
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}

