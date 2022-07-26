use std::io;

fn main()
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
