use std::io;

fn main()
{
    for line in io::stdin().lines()
    {
        match line
        {
            Ok(str) => println!("{}", str),
            Err(err) => println!("{}", err)
        }
    }
}
