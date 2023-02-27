mod arg_parsing;
mod random_strings;

use arg_parsing::parse_args;
use colored::*;
use random_strings::random_alphanumeric_string;

fn main() {
    let args = parse_args();
    match &args {
        Ok(a) => println!("{}", a),
        Err(e) => {
            println!("{}", e.red());
            return;
        }
    };

    let safe_args = args.unwrap();

    let filename_base = random_alphanumeric_string(20);
    let filename = safe_args.full_filename(&filename_base);
    let file_body = match safe_args.size {
        Some(s) => random_alphanumeric_string(s),
        None => filename_base,
    };
    println!("Full filename: {}", filename);
    println!("Random string: {}", file_body);
}
