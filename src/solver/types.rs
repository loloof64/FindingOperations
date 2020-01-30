use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
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

    pub fn value(&self) -> u16 {
        match self.operator {
            Operator::Add => self.operand_1 + self.operand_2,
            Operator::Sub => self.operand_1 - self.operand_2,
            Operator::Mul => self.operand_1 * self.operand_2,
            Operator::Div => self.operand_1 / self.operand_2,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {} = {}", self.operand_1, self.operator, self.operand_2, self.value())
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
