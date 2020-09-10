use logos::{Logos, Lexer};

fn argument_slice(lex: &mut Lexer<Token>) -> Option<u8> {
    let slice = lex.slice();
    let result: u8 = slice[slice.find(|c: char| c.is_ascii_digit())?..].parse().ok()?;
    return Some(result);
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // RANDOM definition
    #[regex(r"#RANDOM\s+\d+", argument_slice)]
    Random(u8),

    // IF definition
    #[regex(r"#IF\s+\d+", argument_slice)]
    StartIf(u8),

    // ENDIF definition
    #[token("#ENDIF")]
    EndIf,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn preprocess(plain: &str) -> Result<String, ()> {
    let mut result = String::new();
    let mut stack = Vec::<Token>::new();

    for t in Token::lexer(plain) {
        stack.push(t);
    }

    return Ok(result);
}
