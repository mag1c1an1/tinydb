use sqlparser::{ast::Statement, parser::ParserError, dialect:: GenericDialect};

pub struct Parser {
}

impl Parser {
    fn parse_sql(query: &str) -> Result<Vec<Statement>, ParserError> {
      let dialect = GenericDialect{};
        sqlparser::parser::Parser::parse_sql(&dialect,query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_select() {
        Parser::parse_sql("select * from t1").unwrap();
    }
}
