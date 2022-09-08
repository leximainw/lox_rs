pub struct Backtrace {
    backtrace: Vec<(usize, usize)>,
    error: String,
}

impl Backtrace {
    pub fn new(error: String) -> Backtrace {
        Backtrace {
            backtrace: Vec::new(),
            error,
        }
    }

    pub fn starting_at(error: String, error_site: (usize, usize)) -> Backtrace {
        let mut backtrace = Backtrace {
            backtrace: Vec::new(),
            error,
        };
        backtrace.push(error_site);
        backtrace
    }

    pub fn push(&mut self, error_site: (usize, usize)) {
        self.backtrace.push(error_site);
    }

    pub fn get_error(&self) -> String {
        self.error.to_string()
    }

    pub fn iter(&self) -> std::slice::Iter<(usize, usize)> {
        self.backtrace.iter()
    }
}
