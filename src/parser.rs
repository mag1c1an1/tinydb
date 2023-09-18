pub use sqlparser::ast::*;
pub use sqlparser::parser::ParserError;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

pub fn parse(sql: &str) -> Result<Vec<Statement>, ParserError>
{
    let dialect = PostgreSqlDialect {};
    Parser::parse_sql(&dialect, sql)
}