pub struct Errors<'a>
{
    source: &'a str,
    error_list: Vec<Error>
}

impl Errors<'_>
{
    pub fn new(source: &str) -> Errors
    {
        Errors{
            source,
            error_list: Vec::new()
        }
    }

    pub fn push(&mut self, message: &'static str, severity: Severity, start: usize, length: usize)
    {
        self.error_list.push(Error{
            message, severity, start, length
        })
    }

    pub fn coalesce(&mut self, target: &mut Errors)
    {
        target.error_list.append(&mut self.error_list);
    }
}

pub struct Error
{
    message: &'static str,
    severity: Severity,
    start: usize,
    length: usize
}

pub enum Severity
{
    Critical,
    Error,
    Warning,
    Info
}