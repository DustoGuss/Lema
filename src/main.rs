use lema::interpreter::{self};
use lema::lexer::{self};
use lema::parser::{self};
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 
    {
        eprintln!("Usage: {} <source_file>", args[0]);
        return;
    }
    let source_file = &args[1];
    let src = std::fs::read_to_string(source_file).expect("Failed to read source file");
    let mut lexer = lexer::Lexer::new(src);
    let tokens = lexer.tokenize();
    let mut parser = parser::Parser::new(Some(tokens));
    let ast = parser.parse();
    let mut interpreter = interpreter::Interpreter::new(ast);
    interpreter.interpret(); 
}
