mod types;

#[cfg(test)]
mod tests {
    use super::types::{Operation, Operator};

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
        assert_eq!(Operation::new(Operator::Add, 3, 5), Operation::new(Operator::Add, 3, 5));
        assert_eq!(Operation::new(Operator::Add, 3, 5), Operation::new(Operator::Add, 5, 3));
        assert_ne!(Operation::new(Operator::Add, 3, 5), Operation::new(Operator::Add, 13, 5));
    }

    #[test]
    fn test_sub_operation_equality() {
        assert_eq!(Operation::new(Operator::Sub, 10, 8), Operation::new(Operator::Sub, 10, 8));
        assert_ne!(Operation::new(Operator::Sub, 10, 8), Operation::new(Operator::Sub, 8, 10));
    }

    #[test]
    fn test_mul_operation_equality() {
        assert_eq!(Operation::new(Operator::Mul, 3, 5), Operation::new(Operator::Mul, 3, 5));
        assert_eq!(Operation::new(Operator::Mul, 3, 5), Operation::new(Operator::Mul, 5, 3));
        assert_ne!(Operation::new(Operator::Mul, 3, 5), Operation::new(Operator::Mul, 13, 5));
    }

    #[test]
    fn test_div_operation_equality() {
        assert_eq!(Operation::new(Operator::Div, 10, 2), Operation::new(Operator::Div, 10, 2));
        assert_ne!(Operation::new(Operator::Div, 10, 2), Operation::new(Operator::Div, 5, 1));
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
        assert_eq!("3 + 2 = 5", format!("{}", Operation::new(Operator::Add, 3, 2).unwrap()));
        assert_eq!("6 - 4 = 2", format!("{}", Operation::new(Operator::Sub, 6, 4).unwrap()));
        assert_eq!("3 * 2 = 6", format!("{}", Operation::new(Operator::Mul, 3, 2).unwrap()));
        assert_eq!("10 / 2 = 5", format!("{}", Operation::new(Operator::Div, 10, 2).unwrap()));
    }
}
