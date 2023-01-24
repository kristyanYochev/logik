use std::fmt;

pub struct Formula {
    root: FormulaLink,
}

type FormulaLink = Box<FormulaNode>;

enum FormulaNode {
    Variable(String),
    And(FormulaLink, FormulaLink),
    Or(FormulaLink, FormulaLink),
    Not(FormulaLink),
    Implication(FormulaLink, FormulaLink),
    Equivalence(FormulaLink, FormulaLink),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl fmt::Display for FormulaNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FormulaNode::*;
        match self {
            Variable(name) => write!(f, "{}", name),
            And(lhs, rhs) => write!(f, "({} & {})", lhs, rhs),
            Or(lhs, rhs) => write!(f, "({} | {})", lhs, rhs),
            Not(formula) => write!(f, "!({})", formula),
            Implication(lhs, rhs) => write!(f, "({} => {})", lhs, rhs),
            Equivalence(lhs, rhs) => write!(f, "({} <=> {})", lhs, rhs),
        }
    }
}
