#[derive(Debug, PartialEq)]
pub enum Token {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

pub fn lex(code: &str) -> Vec<Token> {
    code.chars()
        .filter_map(|c| match c {
            '>' => Some(Token::IncrementPointer),
            '<' => Some(Token::DecrementPointer),
            '+' => Some(Token::IncrementValue),
            '-' => Some(Token::DecrementValue),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            _ => None, // Comments are ignored
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let code = "><+-.,[]abcdef";
        let tokens = lex(code);
        assert_eq!(
            tokens,
            vec![
                Token::IncrementPointer,
                Token::DecrementPointer,
                Token::IncrementValue,
                Token::DecrementValue,
                Token::Output,
                Token::Input,
                Token::LoopStart,
                Token::LoopEnd
            ]
        );
    }
}
