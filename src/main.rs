mod arg_parsing;
mod generate_files;
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

    for i in 0..safe_args.count {
        let filename_base = random_alphanumeric_string(20);
        let filename = safe_args.full_filename(&filename_base);
        let file_body = match safe_args.size {
            Some(s) => random_alphanumeric_string(s),
            None => filename_base,
        };
        // dbg!("{}", i);
        // dbg!("- Full filename: {}", &filename);
        // dbg!("- Body contents: {}", &file_body);

        let write_result = generate_files::create_file(filename.clone(), file_body, safe_args.subdirectory.clone());
        match write_result {
            Ok(_) => println!("Wrote file \"{}\" successfully.", filename),
            Err(e) => println!("{}", e.red())
        }
    }
}
