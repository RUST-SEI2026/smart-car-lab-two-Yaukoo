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
            state: State::new(),
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            // 每条指令先翻译为原子操作序列，再批量执行
            let actions = match cmd {
                'B' => {
                    self.state.toggle_reverse();
                    vec![]
                }
                'F' => {
                    self.state.toggle_accelerate();
                    vec![]
                }
                'M' => self.state.actions_for_move(),
                'L' => self.state.actions_for_left(),
                'R' => self.state.actions_for_right(),
                _ => vec![],
            };
            for action in &actions {
                action.perform(&mut self.pose);
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
