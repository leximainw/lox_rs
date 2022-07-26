use std::{env, fs, io, str};

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

fn run_file(file_name: &String)
{
    match fs::read(file_name)
    {
        Ok(data) => match str::from_utf8(&data)
        {
            Ok(text) => run(text),
            Err(err) => println!("Error reading file: {err}")
        },
        Err(err) => println!("Error reading file: {err}")
    }
}

fn run_prompt()
{
    for line in io::stdin().lines()
    {
        match line
        {
            Ok(text) => run(&text),
            Err(err) => println!("Error reading stdin: {err}")
        }
    }
}

fn run(text: &str)
{
    println!("{}", text);
}
