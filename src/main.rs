mod lox_rs;

use std::{
    env,
    fs,
    io::{self, Write},
    str
};

use lox_rs::Errors;
use lox_rs::Lexer;
use lox_rs::TokenType;   // TODO: remove when not printing TokenTypes
use lox_rs::Parser;
use lox_rs::Expr;
use lox_rs::AstPrinter;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() > 2
    {
        println!("usage: {} [script]", &args[0]);
    }
    else if args.len() == 2
    {
        run_file(&args[1]);
    }
    else
    {
        run_prompt();
    }
}

fn run_file(file_name: &String)
{
    match fs::read(file_name)
    {
        Ok(data) => match str::from_utf8(&data)
        {
            Ok(text) => run(text),
            Err(err) => println!("Error reading {file_name}: {err}")
        },
        Err(err) => println!("Error reading {file_name}: {err}")
    }
}

fn run_prompt()
{
    let mut lines = io::stdin().lines();
    loop
    {
        print!("> ");
        match io::stdout().flush()
        {
            Err(err) => println!("Error printing prompt: {err}"),
            Ok(_) => {}
        }
        match lines.next()
        {
            Some(result) => match result
            {
                Ok(text) => run(&text),
                Err(err) => println!("Error reading stdin: {err}")
            },
            None => break
        }
    }
}

fn run(code: &str)
{
    let mut errors: Errors = Errors::new(code);
    let mut parser: Parser = Parser::new(code);
    let printer: AstPrinter = AstPrinter{};
    while let Some(expr) = parser.next()
    {
        println!("{}", expr.print(&printer));
    }
    parser.coalesce_errors(&mut errors);
    errors.print_errors();
}

fn run_lexer(code: &str)
{
    let mut lexer: Lexer = Lexer::new(code);
    while let Some(token) = lexer.next()
    {
        let text = token.text;
        let kind = token.kind;
        let value = token.value;
        println!("{kind:?} (value: {value:?}, text: \"{text}\")");   // NOTE: remove derive(Debug) from LoxValue and TokenType when removing this
    }
}
