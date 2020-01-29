use std::cmp::PartialEq;

#[cfg(test)]
mod tests {
    use crate::solver::Operation;
    use crate::solver::Operator;

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
}

#[derive(PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct Operation {
    operand_1: u16,
    operand_2: u16,
    operator: Operator,
}

impl Operation {
    pub fn new(operator: Operator, operand_1: u16, operand_2: u16) -> Option<Self> {
        match operator {
            Operator::Div => {
                if operand_1 % operand_2 == 0 {
                    Some(Operation {
                        operand_1,
                        operand_2,
                        operator,
                    })
                } else {
                    None
                }
            }
            _ => Some(Operation {
                operand_1,
                operand_2,
                operator,
            }),
        }
    }
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        if self.operator != other.operator {
            false
        } else {
            match self.operator {
                Operator::Add | Operator::Mul => {
                    (self.operand_1 == other.operand_1 && self.operand_2 == other.operand_2)
                        || (self.operand_1 == other.operand_2 && self.operand_2 == other.operand_1)
                }
                _ => self.operand_1 == other.operand_1 && self.operand_2 == other.operand_2,
            }
        }
    }
}
