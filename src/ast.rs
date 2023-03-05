pub enum NodeType {
    Program,
    NumericLiteral,
    NullLiteral,
    Identifier,
    BinExpr { left: NodeType, right: NodeType },
}

struct Stmt {}
