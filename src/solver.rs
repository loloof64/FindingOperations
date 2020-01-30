mod types;

#[cfg(test)]
mod tests {
    use super::types::{Operation, Operator, Solution};

    #[test]
    fn test_invalid_division_creation_1() {
        assert_eq!(None::<Operation>, Operation::new(Operator::Div, 5, 2));
    }

    #[test]
    fn test_invalid_division_creation_2() {
        assert_eq!(None::<Operation>, Operation::new(Operator::Div, 16, 3));
    }

    #[test]
    fn test_add_operation_equality() {
        assert_eq!(
            Operation::new(Operator::Add, 3, 5),
            Operation::new(Operator::Add, 3, 5)
        );
        assert_eq!(
            Operation::new(Operator::Add, 3, 5),
            Operation::new(Operator::Add, 5, 3)
        );
        assert_ne!(
            Operation::new(Operator::Add, 3, 5),
            Operation::new(Operator::Add, 13, 5)
        );
    }

    #[test]
    fn test_sub_operation_equality() {
        assert_eq!(
            Operation::new(Operator::Sub, 10, 8),
            Operation::new(Operator::Sub, 10, 8)
        );
        assert_ne!(
            Operation::new(Operator::Sub, 10, 8),
            Operation::new(Operator::Sub, 8, 10)
        );
    }

    #[test]
    fn test_mul_operation_equality() {
        assert_eq!(
            Operation::new(Operator::Mul, 3, 5),
            Operation::new(Operator::Mul, 3, 5)
        );
        assert_eq!(
            Operation::new(Operator::Mul, 3, 5),
            Operation::new(Operator::Mul, 5, 3)
        );
        assert_ne!(
            Operation::new(Operator::Mul, 3, 5),
            Operation::new(Operator::Mul, 13, 5)
        );
    }

    #[test]
    fn test_div_operation_equality() {
        assert_eq!(
            Operation::new(Operator::Div, 10, 2),
            Operation::new(Operator::Div, 10, 2)
        );
        assert_ne!(
            Operation::new(Operator::Div, 10, 2),
            Operation::new(Operator::Div, 5, 1)
        );
    }

    #[test]
    fn test_operations_results() {
        assert_eq!(5, Operation::new(Operator::Add, 3, 2).unwrap().value());
        assert_eq!(2, Operation::new(Operator::Sub, 6, 4).unwrap().value());
        assert_eq!(6, Operation::new(Operator::Mul, 3, 2).unwrap().value());
        assert_eq!(5, Operation::new(Operator::Div, 10, 2).unwrap().value());
    }

    #[test]
    fn test_operations_display() {
        assert_eq!(
            "3 + 2 = 5",
            format!("{}", Operation::new(Operator::Add, 3, 2).unwrap())
        );
        assert_eq!(
            "6 - 4 = 2",
            format!("{}", Operation::new(Operator::Sub, 6, 4).unwrap())
        );
        assert_eq!(
            "3 * 2 = 6",
            format!("{}", Operation::new(Operator::Mul, 3, 2).unwrap())
        );
        assert_eq!(
            "10 / 2 = 5",
            format!("{}", Operation::new(Operator::Div, 10, 2).unwrap())
        );
    }

    #[test]
    fn test_solutions_equality() {
        let solution_1_operations = vec![Operation::new(Operator::Add, 3, 2).unwrap()];
        let solution_2_operations = vec![Operation::new(Operator::Add, 3, 2).unwrap()];
        let solution_1 = Solution::new(solution_1_operations);
        let solution_2 = Solution::new(solution_2_operations);
        assert_eq!(solution_1, solution_2);

        let solution_3_operations = vec![
            Operation::new(Operator::Add, 3, 2).unwrap(),
            Operation::new(Operator::Mul, 10, 6).unwrap(),
        ];
        let solution_4_operations = vec![
            Operation::new(Operator::Mul, 10, 6).unwrap(),
            Operation::new(Operator::Add, 3, 2).unwrap(),
        ];
        let solution_3 = Solution::new(solution_3_operations);
        let solution_4 = Solution::new(solution_4_operations);
        assert_eq!(solution_3, solution_4);

        let solution_5_operations = vec![
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Add, 3, 2).unwrap(),
            Operation::new(Operator::Mul, 10, 6).unwrap(),
        ];
        let solution_6_operations = vec![
            Operation::new(Operator::Mul, 10, 6).unwrap(),
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Add, 3, 2).unwrap(),
        ];
        let solution_5 = Solution::new(solution_5_operations);
        let solution_6 = Solution::new(solution_6_operations);
        assert_eq!(solution_5, solution_6);

        let solution_7_operations = vec![
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Add, 3, 2).unwrap(),
            Operation::new(Operator::Mul, 10, 6).unwrap(),
        ];
        let solution_8_operations = vec![
            Operation::new(Operator::Mul, 10, 6).unwrap(),
            Operation::new(Operator::Mul, 4, 5).unwrap(),
        ];
        let solution_7 = Solution::new(solution_7_operations);
        let solution_8 = Solution::new(solution_8_operations);
        assert_ne!(solution_7, solution_8);

        let solution_9_operations = vec![
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Add, 3, 2).unwrap(),
            Operation::new(Operator::Mul, 10, 6).unwrap(),
        ];
        let solution_10_operations = vec![
            Operation::new(Operator::Mul, 10, 6).unwrap(),
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Mul, 4, 5).unwrap(),
        ];
        let solution_9 = Solution::new(solution_9_operations);
        let solution_10 = Solution::new(solution_10_operations);
        assert_ne!(solution_9, solution_10);

        let solution_11_operations = vec![
            Operation::new(Operator::Mul, 10, 6).unwrap(),
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Mul, 4, 5).unwrap(),
        ];
        let solution_12_operations = vec![
            Operation::new(Operator::Mul, 4, 5).unwrap(),
            Operation::new(Operator::Add, 3, 2).unwrap(),
            Operation::new(Operator::Mul, 10, 6).unwrap(),
        ];
        let solution_11 = Solution::new(solution_11_operations);
        let solution_12 = Solution::new(solution_12_operations);
        assert_ne!(solution_11, solution_12);
    }
}
