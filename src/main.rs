#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Token {
    Identifier(String),
    Number(f64),
  //  Name(String),
    Operator(char),
    Punctuation(char),
    Keyword(String),
    EOF,
}
#[allow(dead_code)]
impl Token {
    fn is_value(&self) -> bool {
        matches!(self, 
            Token::Number(_) | 
            Token::Identifier(_) | 
            Token::Keyword(_)|
            Token::Name(_)
        )
    }

    fn as_number(&self) -> Option<f64> {
        if let Token::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

fn main() {
    let tokens = vec![
        Token::Keyword("fn".to_string()),
   //     Token::Keyword("fn".to_string()),
        Token::Identifier("main".to_string()),
        Token::Punctuation('('),
        Token::Punctuation(')'),
        Token::Number(42.0),
    ];

    for token in tokens {
        println!("{:?} → is_value? {}", token, token.is_value());
    }
    
}
