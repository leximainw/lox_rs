pub use self::vm::VM;

mod vm
{
    pub struct VM
    {
        
    }
    
    impl VM
    {
        pub fn new() -> VM
        {
            VM{

            }
        }

        pub fn run(&mut self, code: &str)
        {
            println!("{code}");
        }
    }
}
