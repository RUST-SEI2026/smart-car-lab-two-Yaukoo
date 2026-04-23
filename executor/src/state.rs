/// 小车行驶状态
/// - Normal：正常状态
/// - Reverse：倒车状态（B 指令触发，再次收到 B 则取消）
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Normal,
    Reverse,
}

impl State {
    /// 收到 B 指令时切换状态
    pub fn toggle_reverse(&mut self) {
        *self = match self {
            State::Normal => State::Reverse,
            State::Reverse => State::Normal,
        };
    }

    /// 当前是否处于倒车状态
    pub fn is_reverse(&self) -> bool {
        *self == State::Reverse
    }
}
