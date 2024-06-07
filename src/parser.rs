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
    let mut loop_start_positions = Vec::new();
    let mut nodes = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        let n = match t {
            Token::LoopStart => {
                loop_start_positions.push(i);
                Node::LoopStart(None)
            }
            Token::LoopEnd => {
                let offset;
                if let Some(loop_start_position) = loop_start_positions.pop() {
                    offset = i - loop_start_position;
                    nodes[loop_start_position] = Node::LoopStart(Some(offset));
                } else {
                    panic!("Unmatched brackets");
                }
                Node::LoopEnd(Some(offset))
            }
            Token::IncrementPointer => Node::IncrementPointer,
            Token::DecrementPointer => Node::DecrementPointer,
            Token::IncrementValue => Node::IncrementValue,
            Token::DecrementValue => Node::DecrementValue,
            Token::Output => Node::Output,
            Token::Input => Node::Input,
        };
        nodes.push(n)
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::LoopStart,
            Token::IncrementValue,
            Token::DecrementValue,
            Token::LoopStart,
            Token::DecrementPointer,
            Token::IncrementPointer,
            Token::LoopEnd,
            Token::IncrementValue,
            Token::LoopStart,
            Token::LoopEnd,
            Token::LoopEnd,
        ];
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

    #[test]
    #[should_panic(expected = "Unmatched brackets")]
    fn test_parse_unmatched_brackets() {
        let tokens = vec![Token::LoopStart, Token::LoopEnd, Token::LoopEnd];
        parse(tokens);
    }
}
