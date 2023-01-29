use std::fmt;

/// Represents a formal logic formula.
/// All of its nodes are stored on the heap.
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

impl Formula {
    fn from_node(node: FormulaNode) -> Self {
        Self {
            root: Box::new(node),
        }
    }

    pub fn var(name: String) -> Self {
        Self::from_node(FormulaNode::Variable(name))
    }

    pub fn and(lhs: Self, rhs: Self) -> Self {
        Self::from_node(FormulaNode::And(lhs.root, rhs.root))
    }

    pub fn or(lhs: Self, rhs: Self) -> Self {
        Self::from_node(FormulaNode::Or(lhs.root, rhs.root))
    }

    pub fn not(formula: Self) -> Self {
        Self::from_node(FormulaNode::Not(formula.root))
    }

    pub fn implication(lhs: Self, rhs: Self) -> Self {
        Self::from_node(FormulaNode::Implication(lhs.root, rhs.root))
    }

    pub fn equivalence(lhs: Self, rhs: Self) -> Self {
        Self::from_node(FormulaNode::Equivalence(lhs.root, rhs.root))
    }
}

/// The formula is always generated in fully-bracketed form.
///
/// ```rust
/// use logik::Formula;
///
/// let a = Formula::var(String::from("a"));
/// let b = Formula::var(String::from("b"));
/// let c = Formula::var(String::from("c"));
/// let f = Formula::and(Formula::or(a, b), c);
///
/// assert_eq!(format!("{}", f), "((a | b) & c)")
/// ```
impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

/// Formula nodes are always displayed in their fully-bracketed state.
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
