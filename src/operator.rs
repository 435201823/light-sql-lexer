#[derive(Debug, PartialEq)]
pub enum Operator {
    Arithmetic(ArithmeticOperator),
    Logical(LogicalOperator),
    Comparison(ComparisonOperator),
    Bitwise(BitwiseOperator),
    Json(JsonOperator),
}