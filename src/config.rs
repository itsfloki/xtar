#[derive(Debug)]
pub struct Config {
    pub options: Vec<char>,
    pub archive_file: String,
    pub paths: Vec<String>,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Self, &'static str> {
        let mut paths: Vec<String> = Vec::new();
        args.next();

        let options = match args.next() {
            Some(op) => op.chars().skip(1).collect::<Vec<char>>(),
            None => return Err("options arguments missing..."),
        };

        let archive_file = match args.next() {
            Some(f) => f,
            None => String::new(),
        };

        for arg in args {
            paths.push(arg);
        }

        Ok(Config {
            options,
            archive_file,
            paths,
        })
    }
}
