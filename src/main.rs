mod lox_rs;

use std::{
    env,
    fs,
    io::{self, Write},
    str
};

use lox_rs::VM;

fn main()
{
    let vm: VM = VM::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2
    {
        println!("usage: {} [script]", &args[0]);
    }
    else if args.len() == 2
    {
        run_file(vm, &args[1]);
    }
    else
    {
        run_prompt(vm);
    }
}

fn run_file(mut vm: VM, file_name: &String)
{
    match fs::read(file_name)
    {
        Ok(data) => match str::from_utf8(&data)
        {
            Ok(text) => vm.run(text),
            Err(err) => println!("Error reading {file_name}: {err}")
        },
        Err(err) => println!("Error reading {file_name}: {err}")
    }
}

fn run_prompt(mut vm: VM)
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
                Ok(text) => vm.run(&text),
                Err(err) => println!("Error reading stdin: {err}")
            },
            None => break
        }
    }
}
