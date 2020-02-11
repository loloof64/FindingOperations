use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter, Result};
use std::slice::Iter;

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn iterator() -> Iter<'static, Self> {
        [Operator::Add, Operator::Sub, Operator::Mul, Operator::Div].iter()
    }
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
    operand_1: u32,
    operand_2: u32,
    operator: Operator,
}

/// Operation builds an Operation data from two operands and an operator, but :
/// - Any operation which has at least a 0 as operand, as unuseful is considered invalid and returns None
/// - Any subtraction which leads to a 0 or negative value is also considered invalid
/// - Any division which do not leads to an integer result (without any remainder) is considered invalid
/// - Any multiplication by 1 is considered invalid
impl Operation {
    pub fn new(operator: Operator, operand_1: u32, operand_2: u32) -> Option<Self> {
        if operand_1 == 0 || operand_2 == 0 { None }
        else {
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
                },
                Operator::Sub => {
                    if operand_1 > operand_2 {
                        Some(Operation {operand_1, operand_2, operator})
                    } else {None}
                },
                Operator::Mul => {
                    if operand_1 > 1 && operand_2 > 1 {
                        Some(Operation {operand_1, operand_2, operator})
                    } else {None}
                },
                _ => Some(Operation {
                    operand_1,
                    operand_2,
                    operator,
                }),
            }
        }
    }

    pub fn value(&self) -> u32 {
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
        write!(
            f,
            "{} {} {} = {}",
            self.operand_1,
            self.operator,
            self.operand_2,
            self.value()
        )
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

impl Clone for Operation {
    fn clone(&self) -> Self {
        Operation {
            operand_1: self.operand_1,
            operand_2: self.operand_2,
            operator: self.operator.clone(),
        }
    }
}

pub struct Solution {
    pub operations: Vec<Operation>,
}

impl Solution {
    pub fn new(operations: Vec<Operation>) -> Self {
        Solution {
            operations: operations,
        }
    }

    pub fn new_empty() -> Self {
        Solution {
            operations: Vec::new(),
        }
    }

    pub fn push(&mut self, operation: Operation) {
        self.operations.push(operation);
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        if self.operations.len() != other.operations.len() {
            false
        } else {
            let mut are_equals = true;
            // each member of self must be on other
            let self_operations_clone = self.operations.clone();
            for this_member in self_operations_clone {
                let found = other
                    .operations
                    .clone()
                    .into_iter()
                    .filter(|x| *x == this_member)
                    .collect::<Vec<_>>()
                    .len()
                    > 0;
                if !found {
                    are_equals = false;
                    break;
                }
            }
            // each member of other must be in self
            let other_operations_clone = other.operations.clone();
            for other_member in other_operations_clone {
                let found = self
                    .operations
                    .clone()
                    .into_iter()
                    .filter(|x| *x == other_member)
                    .collect::<Vec<_>>()
                    .len()
                    > 0;
                if !found {
                    are_equals = false;
                    break;
                }
            }

            are_equals
        }
    }
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Solution {
            operations: self.operations.clone(),
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Solution[{}]",
            self.operations
                .clone()
                .into_iter()
                .map(|curr| curr.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
