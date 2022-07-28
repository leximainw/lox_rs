mod lox_rs;

use std::{
    env,
    fs,
    io::{self, Write},
    str
};

use lox_rs::Lexer;
use lox_rs::TokenType;   // TODO: remove when not printing TokenTypes


use lox_rs::LoxValue;
use lox_rs::expr::{
    AstPrinter,
    Expr,
    Binary,
    Grouping,
    Unary,
    Literal
};

fn main()
{
    let expression: Box<dyn Expr> = Box::new(Binary{
        left: Box::new(Unary{
            oper: TokenType::Minus,
            expr: Box::new(Literal{
                value: LoxValue::Num(123.0)
            })
        }),
        oper: TokenType::Star,
        right: Box::new(Grouping{
            expr: Box::new(Literal{
                value: LoxValue::Num(45.67)
            })
        })
    });
    let printer = AstPrinter{};
    println!("{}", expression.visit(&printer));

    return;

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
    let mut lexer: Lexer = Lexer::new(code);
    loop
    {
        let token = lexer.next();
        let text = token.text;
        let kind = token.kind;
        let value = token.value;
        if kind == TokenType::EOF
        {
            break
        }
        println!("{kind:?} (value: {value:?}, text: \"{text}\")");   // NOTE: remove derive(Debug) from LoxValue and TokenType when removing this
    }
}
