use executor::{Executor, Pose};

mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("M");
        // then
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_y_minus_1_given_command_is_m_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("M");

        // when
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_x_minus_1_given_command_is_m_and_facing_is_w() {
        // given
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("M");

        // then
        let expected_pose = Pose::new(-1, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_y_plus_1_given_command_is_m_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("M");

        // then
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}

mod turn_left_tests {
    use super::*;

    #[test]
    fn should_return_facing_n_given_command_is_l_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_l_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_s_given_command_is_l_and_facing_is_w() {
        // given
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_w_given_command_is_l_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }
}

mod turn_right_tests {
    use super::*;

    #[test]
    fn should_return_facing_s_given_command_is_r_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_w_given_command_is_r_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_n_given_command_is_r_and_facing_is_w() {
        // given
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_r_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
}

// ===== B 指令（倒车）测试 =====
// 需求：
//   B：进入/退出倒车状态
//   倒车状态下：
//     M：后退一格，朝向不变
//     L：右转 90 度，位置不变
//     R：左转 90 度，位置不变
mod reverse_tests {
    use super::*;

    // --- B+M：四个朝向各退一格 ---

    #[test]
    fn should_return_x_minus_1_given_b_then_m_and_facing_is_e() {
        // given: 朝东，进入倒车后执行 M
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        // when
        executor.execute("BM");
        // then: x 减 1，朝向不变
        assert_eq!(Pose::new(-1, 0, 'E'), executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_b_then_m_and_facing_is_s() {
        // given: 朝南，进入倒车后执行 M
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'S'));
        // when
        executor.execute("BM");
        // then: y 加 1，朝向不变
        assert_eq!(Pose::new(0, 1, 'S'), executor.query());
    }

    #[test]
    fn should_return_x_plus_1_given_b_then_m_and_facing_is_w() {
        // given: 朝西，进入倒车后执行 M
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'W'));
        // when
        executor.execute("BM");
        // then: x 加 1，朝向不变
        assert_eq!(Pose::new(1, 0, 'W'), executor.query());
    }

    #[test]
    fn should_return_y_minus_1_given_b_then_m_and_facing_is_n() {
        // given: 朝北，进入倒车后执行 M
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        // when
        executor.execute("BM");
        // then: y 减 1，朝向不变
        assert_eq!(Pose::new(0, -1, 'N'), executor.query());
    }

    // --- B+L：倒车状态下 L 变为右转 ---

    #[test]
    fn should_turn_right_given_b_then_l_and_facing_is_n() {
        // given: 朝北，倒车状态下 L 应右转 → 朝东
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        // when
        executor.execute("BL");
        // then: 位置不变，朝向变为 E
        assert_eq!(Pose::new(0, 0, 'E'), executor.query());
    }

    #[test]
    fn should_turn_right_given_b_then_l_and_facing_is_e() {
        // given: 朝东，倒车状态下 L 应右转 → 朝南
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        // when
        executor.execute("BL");
        // then: 位置不变，朝向变为 S
        assert_eq!(Pose::new(0, 0, 'S'), executor.query());
    }

    // --- B+R：倒车状态下 R 变为左转 ---

    #[test]
    fn should_turn_left_given_b_then_r_and_facing_is_n() {
        // given: 朝北，倒车状态下 R 应左转 → 朝西
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        // when
        executor.execute("BR");
        // then: 位置不变，朝向变为 W
        assert_eq!(Pose::new(0, 0, 'W'), executor.query());
    }

    #[test]
    fn should_turn_left_given_b_then_r_and_facing_is_e() {
        // given: 朝东，倒车状态下 R 应左转 → 朝北
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        // when
        executor.execute("BR");
        // then: 位置不变，朝向变为 N
        assert_eq!(Pose::new(0, 0, 'N'), executor.query());
    }

    // --- 再次收到 B：取消倒车状态，恢复正常 ---

    #[test]
    fn should_cancel_reverse_given_b_twice_then_m_facing_n() {
        // given: B 进入倒车，再 B 退出，M 应正常前进
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        // when
        executor.execute("BBM");
        // then: y 加 1（正常前进）
        assert_eq!(Pose::new(0, 1, 'N'), executor.query());
    }

    // --- 组合指令验证 ---

    #[test]
    fn should_handle_combined_reverse_commands() {
        // given: 朝北，倒车状态下 M（退一格）再 L（右转→朝东）
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        // when
        executor.execute("BML");
        // then: y=-1，朝向=E
        assert_eq!(Pose::new(0, -1, 'E'), executor.query());
    }
}

// ===== F 指令（加速）测试 =====
// 需求：
//   F：进入/退出加速状态
//   加速状态下：
//     M：前进 2 格（逐格，不跳跃）
//     L：先前进 1 格，再左转 90 度
//     R：先前进 1 格，再右转 90 度
//
//   B+F 叠加状态下：
//     M：后退 2 格（逐格，不跳跃）
//     L：先后退 1 格，再右转 90 度
//     R：先后退 1 格，再左转 90 度
mod accelerate_tests {
    use super::*;

    // --- F+M：四个朝向各前进 2 格 ---

    #[test]
    fn should_return_x_plus_2_given_f_then_m_and_facing_is_e() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FM");
        assert_eq!(Pose::new(2, 0, 'E'), executor.query());
    }

    #[test]
    fn should_return_y_minus_2_given_f_then_m_and_facing_is_s() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'S'));
        executor.execute("FM");
        assert_eq!(Pose::new(0, -2, 'S'), executor.query());
    }

    #[test]
    fn should_return_x_minus_2_given_f_then_m_and_facing_is_w() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'W'));
        executor.execute("FM");
        assert_eq!(Pose::new(-2, 0, 'W'), executor.query());
    }

    #[test]
    fn should_return_y_plus_2_given_f_then_m_and_facing_is_n() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("FM");
        assert_eq!(Pose::new(0, 2, 'N'), executor.query());
    }

    // --- F+L：先前进 1 格，再左转 ---

    #[test]
    fn should_forward_then_turn_left_given_f_then_l_and_facing_is_n() {
        // 朝北前进 1 格 → (0,1)，再左转 → 朝西
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("FL");
        assert_eq!(Pose::new(0, 1, 'W'), executor.query());
    }

    #[test]
    fn should_forward_then_turn_left_given_f_then_l_and_facing_is_e() {
        // 朝东前进 1 格 → (1,0)，再左转 → 朝北
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FL");
        assert_eq!(Pose::new(1, 0, 'N'), executor.query());
    }

    // --- F+R：先前进 1 格，再右转 ---

    #[test]
    fn should_forward_then_turn_right_given_f_then_r_and_facing_is_n() {
        // 朝北前进 1 格 → (0,1)，再右转 → 朝东
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("FR");
        assert_eq!(Pose::new(0, 1, 'E'), executor.query());
    }

    #[test]
    fn should_forward_then_turn_right_given_f_then_r_and_facing_is_e() {
        // 朝东前进 1 格 → (1,0)，再右转 → 朝南
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FR");
        assert_eq!(Pose::new(1, 0, 'S'), executor.query());
    }

    // --- 再次收到 F：取消加速状态，恢复正常 ---

    #[test]
    fn should_cancel_accelerate_given_f_twice_then_m_facing_n() {
        // FF 取消加速，M 正常前进 1 格
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("FFM");
        assert_eq!(Pose::new(0, 1, 'N'), executor.query());
    }

    // ===== B+F 叠加状态 =====

    // --- B+F+M：后退 2 格 ---

    #[test]
    fn should_return_y_minus_2_given_bf_then_m_and_facing_is_n() {
        // 朝北，B+F 叠加，M 后退 2 格
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("BFM");
        assert_eq!(Pose::new(0, -2, 'N'), executor.query());
    }

    #[test]
    fn should_return_x_minus_2_given_bf_then_m_and_facing_is_e() {
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BFM");
        assert_eq!(Pose::new(-2, 0, 'E'), executor.query());
    }

    // --- B+F+L：先后退 1 格，再右转 ---

    #[test]
    fn should_backward_then_turn_right_given_bf_then_l_and_facing_is_n() {
        // 朝北后退 1 格 → (0,-1)，再右转 → 朝东
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("BFL");
        assert_eq!(Pose::new(0, -1, 'E'), executor.query());
    }

    // --- B+F+R：先后退 1 格，再左转 ---

    #[test]
    fn should_backward_then_turn_left_given_bf_then_r_and_facing_is_n() {
        // 朝北后退 1 格 → (0,-1)，再左转 → 朝西
        let mut executor = Executor::with_pose(Pose::new(0, 0, 'N'));
        executor.execute("BFR");
        assert_eq!(Pose::new(0, -1, 'W'), executor.query());
    }
}
