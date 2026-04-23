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
                // M：倒车状态下后退，否则前进
                'M' => {
                    if self.state.is_reverse() {
                        self.pose.backward();
                    } else {
                        self.pose.forward();
                    }
                }
                // L：倒车状态下变为右转，否则正常左转
                'L' => {
                    if self.state.is_reverse() {
                        self.pose.turn_right();
                    } else {
                        self.pose.turn_left();
                    }
                }
                // R：倒车状态下变为左转，否则正常右转
                'R' => {
                    if self.state.is_reverse() {
                        self.pose.turn_left();
                    } else {
                        self.pose.turn_right();
                    }
                }
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}

