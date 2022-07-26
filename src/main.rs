use std::{env, io};

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() > 2
    {
        println!("usage: {} [script]", &args[0])
    }
    else if args.len() == 2
    {
        run_file(&args[1])
    }
    else
    {
        run_prompt()
    }
}

fn run_file(_file_name: &String)
{
    println!("TODO")
}

fn run_prompt()
{
    for line in io::stdin().lines()
    {
        match line
        {
            Ok(text) => println!("{}", text),
            Err(err) => println!("{}", err)
        }
    }
}
