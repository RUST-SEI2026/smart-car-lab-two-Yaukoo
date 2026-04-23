[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/5pSShGos)

# Smart Car Lab Two — 支持倒车与加速功能

## 项目简介

本实验在 Lab One（基础移动指令）的基础上，为智能小车的 `Executor` 组件新增 **倒车（B）** 和 **加速（F）** 两种状态指令，并采用"指令翻译 → 原子操作序列 → 批量执行"的架构实现生成与执行分离。

---

## 项目结构

```
executor/
├── src/
│   ├── lib.rs          # 模块注册与公开导出
│   ├── pose.rs         # 小车位姿（坐标 + 朝向），提供移动/转向原子方法
│   ├── action.rs       # 原子操作枚举（Forward/Backward/TurnLeft/TurnRight）
│   ├── state.rs        # 行驶状态（reverse/accelerate 标志），负责将指令翻译为操作序列
│   └── executor.rs     # 编排层：解析指令字符串，驱动 State 生成序列并批量执行
└── tests/
    └── executor_test.rs  # 集成测试（move / turn / reverse / accelerate）
```

### 各模块职责

| 模块 | 职责 |
|------|------|
| `pose.rs` | 纯数据层，维护 `(x, y, heading)`，提供 `forward/backward/turn_left/turn_right` |
| `action.rs` | 定义原子操作枚举，持有 `perform(&mut Pose)` 执行逻辑 |
| `state.rs` | 维护 `reverse/accelerate` 两个状态标志，为 M/L/R 生成对应 `Vec<Action>` |
| `executor.rs` | 遍历指令字符串，B/F 切换状态，M/L/R 翻译为序列后批量执行 |

---

## 如何运行

### 运行测试

```bash
cargo test
```

### 预期输出

```
running 35 tests
... (全部 ok)
test result: ok. 35 passed; 0 failed
```
---

## 需求原文

**B：倒车指令，接收到该指令，车进入倒车状态，该状态下：**

- M：在当前朝向上后退一格，朝向不变。注：比如朝向为 N 时收到 M 指令，y 坐标减 1，朝向保持 N。
- L：右转 90 度，位置不变
- R：左转 90 度，位置不变

**F：加速指令，接收到该指令，车进入加速状态，该状态下：**

- M：前进 2 格（不能跳跃，只能一格一格前进）
- L：先前进 1 格，然后左转 90 度
- R：先前进 1 格，然后右转 90 度

**B 和 F 两个状态可以叠加，叠加状态下：**

- M：倒退 2 格（不能跳跃，只能一格一格后退）
- L：先倒退一格，然后右转 90 度
- R：先倒退一格，然后左转 90 度

再接收一次 B 或者 F 指令，对应的状态取消。
