use core::panic;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    LoopStart(Option<usize>),
    LoopEnd(Option<usize>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut offsets_end = Vec::new();
    let mut offsets_start = Vec::new();
    let mut nodes: Vec<Node> = tokens
        .iter()
        .enumerate()
        .map(|(i, t)| -> Node {
            match t {
                Token::LoopStart => {
                    offsets_end.push(i);
                    Node::LoopStart(None)
                }
                Token::LoopEnd => {
                    let offset = i - offsets_end.pop().expect("Unmatched brackets");
                    offsets_start.push(offset);
                    Node::LoopEnd(Some(offset))
                }
                Token::IncrementPointer => Node::IncrementPointer,
                Token::DecrementPointer => Node::DecrementPointer,
                Token::IncrementValue => Node::IncrementValue,
                Token::DecrementValue => Node::DecrementValue,
                Token::Output => Node::Output,
                Token::Input => Node::Input,
            }
        })
        .collect();

    for n in nodes.iter_mut() {
        if let Node::LoopStart(None) = n {
            *n = Node::LoopStart(Some(offsets_start.pop().expect("Something went wrong")));
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn test_parse() {
        let code = "[+-[<>]+[]]";
        let tokens = lex(code);
        let nodes = parse(tokens);
        assert_eq!(
            nodes,
            vec![
                Node::LoopStart(Some(10)),
                Node::IncrementValue,
                Node::DecrementValue,
                Node::LoopStart(Some(3)),
                Node::DecrementPointer,
                Node::IncrementPointer,
                Node::LoopEnd(Some(3)),
                Node::IncrementValue,
                Node::LoopStart(Some(1)),
                Node::LoopEnd(Some(1)),
                Node::LoopEnd(Some(10))
            ]
        );
    }
}
