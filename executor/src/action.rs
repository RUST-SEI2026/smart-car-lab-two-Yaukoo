/// 小车原子操作类型
/// 每条用户指令（M/L/R）会被翻译为一个或多个 Action，
/// 由 Executor 按顺序逐一执行。
use crate::pose::Pose;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// 沿当前朝向前进一格
    Forward,
    /// 沿当前朝向后退一格
    Backward,
    /// 左转 90 度，位置不变
    TurnLeft,
    /// 右转 90 度，位置不变
    TurnRight,
}

impl Action {
    /// 执行当前 Action，更新 Pose
    pub(crate) fn perform(&self, pose: &mut Pose) {
        match self {
            Action::Forward => pose.forward(),
            Action::Backward => pose.backward(),
            Action::TurnLeft => pose.turn_left(),
            Action::TurnRight => pose.turn_right(),
        }
    }
}
