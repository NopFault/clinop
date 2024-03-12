use std::str::FromStr;

pub struct CliNop {
    pub args: Vec<String>,
}

impl CliNop {
    pub fn new(args: Vec<String>) -> CliNop {
        CliNop { args }
    }

    pub fn get<T: FromStr>(&self, arg: &str) -> Option<T> {
        for (indx, argument) in self.args.iter().enumerate() {
            if argument.trim_start_matches('-') == arg
                && indx + 1 < self.args.len()
                && !self.args[indx + 1].starts_with("--")
            {
                return T::from_str(self.args[indx + 1].as_str()).ok();
            }
        }
        None
    }
}
