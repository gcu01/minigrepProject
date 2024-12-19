use std::env;
use std::fs;
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build<'x>(args: &'x Vec<String>) -> Result<Config<'x>, &'static str>{
        if args.len() < 3 {
            return Err("not enough ArgumentS!");
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    //let contents = fs::read_to_string(config.file_path).expect("ERROR!!! Should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;

    let res = if config.ignore_case == true {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in res {
        println!("{}", line);
    }
    //println!("{}", contents);
    Ok(())
}

pub fn search<'a> (q: &str, c: &'a str) -> Vec<&'a str> {
    
    let mut v:Vec<&str> = vec![];

    for item in c.lines() {
        if item.contains(q) {
            v.push(item);
        }
    }
    //dbg!(&v);
    v
}

pub fn search_case_insensitive<'a> (q: &str, c: &'a str) -> Vec<&'a str> {
    let q = q.to_lowercase();
    let mut res:Vec<&str> = vec![];

    for item in c.lines() {
        if item.to_lowercase().contains(&q) {
            res.push(item);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn build_case_insensitive() {
        let query = "Rust";
        let content = "\
Rust:
safe, fast, productive.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content));
    }
}