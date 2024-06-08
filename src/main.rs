use interpreter::run;
use lexer::lex;
use parser::parse;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let code = r#"++++++++++
[
>+++++++>++++++++++>+++>+<<<<-
]
>++. Loop iniziale (dopo viene stampata una H)
>+. e
+++++++. l
. l
+++. o
>++.
<<+++++++++++++++.
>.
+++.
------.
--------.
>+.
>."#;

    let tokens = lex(code);
    let nodes = parse(tokens);
    run(nodes);
}
