use crate::parser::Node;

const MEM_SIZE: usize = 0x1000;

pub fn run(nodes: Vec<Node>) {
    let mut memory: Vec<u8> = vec![0; MEM_SIZE];
    let mut i: usize = 0;
    let mut data = 0;
    while i < nodes.len() {
        if let Some(node) = nodes.get(i) {
            match node {
                Node::IncrementPointer => data += 1,
                Node::DecrementPointer => data -= 1,
                Node::IncrementValue => memory[data] += 1,
                Node::DecrementValue => memory[data] -= 1,
                Node::Output => {
                    if let Some(character) = char::from_u32(memory[data] as u32) {
                        print!("{}", character);
                    } else {
                        panic!("Invalid Unicode")
                    }
                }
                Node::Input => todo!(),
                Node::LoopStart(Some(offset)) => {
                    if memory[data] == 0 {
                        i += offset
                    }
                }
                Node::LoopEnd(Some(offset)) => {
                    if memory[data] != 0 {
                        i -= offset
                    }
                }
                _ => panic!("Something went wrong"),
            }
            i += 1;
        } else {
            panic!["Something went wrong"];
        }
    }
}
