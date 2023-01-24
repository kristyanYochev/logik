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
