pub mod arg_parsing;

use arg_parsing::parse_args;
use colored::*;

fn main() {
    let args = parse_args();
    match args {
        Ok(a) => println!("{}", a),
        Err(e) => println!("{}", e.red()),
    };
}
