use crate::action::Action;

/// 小车行驶状态

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    /// 是否处于倒车状态（B 指令切换）
    reverse: bool,
    /// 是否处于加速状态（F 指令切换）
    accelerate: bool,
}

impl State {
    pub fn new() -> Self {
        State { reverse: false, accelerate: false }
    }

    /// 收到 B 指令时切换倒车标志
    pub fn toggle_reverse(&mut self) {
        self.reverse = !self.reverse;
    }

    /// 收到 F 指令时切换加速标志
    pub fn toggle_accelerate(&mut self) {
        self.accelerate = !self.accelerate;
    }

    /// 根据当前状态，将 M 指令翻译为原子操作序列
    pub fn actions_for_move(&self) -> Vec<Action> {
        match (self.reverse, self.accelerate) {
            (false, false) => vec![Action::Forward],
            (true,  false) => vec![Action::Backward],
            (false, true)  => vec![Action::Forward, Action::Forward],
            (true,  true)  => vec![Action::Backward, Action::Backward],
        }
    }

    /// 根据当前状态，将 L 指令翻译为原子操作序列
    pub fn actions_for_left(&self) -> Vec<Action> {
        match (self.reverse, self.accelerate) {
            (false, false) => vec![Action::TurnLeft],
            (true,  false) => vec![Action::TurnRight],
            (false, true)  => vec![Action::Forward, Action::TurnLeft],
            (true,  true)  => vec![Action::Backward, Action::TurnRight],
        }
    }

    /// 根据当前状态，将 R 指令翻译为原子操作序列
    pub fn actions_for_right(&self) -> Vec<Action> {
        match (self.reverse, self.accelerate) {
            (false, false) => vec![Action::TurnRight],
            (true,  false) => vec![Action::TurnLeft],
            (false, true)  => vec![Action::Forward, Action::TurnRight],
            (true,  true)  => vec![Action::Backward, Action::TurnLeft],
        }
    }
}
