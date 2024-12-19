use std::env;

mod lib;
use lib::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //let query = &args[1];
    //let file_path = &args[2];
    //let cfg = Config::build(&args);
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumets: {}", err);
        process::exit(1);
    });
    

    if let Err(e) = run(cfg) {
        eprintln!("Problem parsing argumets: {}", e);
        process::exit(1);
    }

}
