use std::{env, fs, io::{self, Write}, str};

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

fn run(text: &str)
{
    println!("{}", text);
}
