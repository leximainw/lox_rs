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
    while let Some(expr) = parser.next()
    {
        match expr.run()
        {
            Ok(value) => println!("{value}"),
            Err(err) =>
            {
                let (msg, pos) = err;
                let (start, len) = pos;
                print_error(code, "Runtime", msg, start, len);
            }
        }
    }
    parser.coalesce_errors(&mut errors);
    errors.print_errors(Box::new(|code, msg, sev, start, len| print_error(code, msg, sev, start, len)));
}

fn run_lexer(code: &str)
{
    let mut lexer: Lexer = Lexer::new(code);
    while let Some(token) = lexer.next()
    {
        let text = token.text;
        let kind = token.kind;
        let value = token.value;
        println!("{kind:?} (value: {value:?}, text: \"{text}\")");   // NOTE: remove derive(Debug) from TokenType when removing this
    }
}

fn print_error(code: &str, sev: &str, msg: &str, start: usize, len: usize)
{
    let mut index = 0;
    let mut line = 1;
    let mut line_start = 0;
    let mut line_next = 0;
    loop
    {
        if let Some(needle) = code[index..]
            .find('\n').map(|i| i + index)
        {
            if index > start { break }
            line_start = line_next;
            line_next = needle + 1;
            index = line_next;
            line += 1;
        }
        else
        {
            line_start = line_next;
            line_next = code.len() + 1;
            break;
        }
    }
    println!("{sev}: {msg}");
    let line_prefix = format!("line {line}: ");
    println!("{line_prefix}{}", &code[line_start .. line_next - 1]);
    if len != 0
    {
        println!("{}{}", format!("{:>1$}", "here --", start + line_prefix.len()),
            format!("{:^<1$}", "", len));
    }
    else if start < (line_start + line_next - 1) / 2
    {
        println!("{}\\__ here", format!("{:>1$}", "", start + line_prefix.len()));
    }
    else
    {
        println!("{}", format!("{:>1$}", "here __/", start + line_prefix.len()));
    }
}
