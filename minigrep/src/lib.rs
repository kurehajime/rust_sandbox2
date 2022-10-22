use std::{error::Error, fs::File, io::Read};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please input query and filename");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
        })
    }
}
