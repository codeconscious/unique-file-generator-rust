mod arg_parsing;
mod generate_files;
mod random_strings;

use arg_parsing::parse_args;
use colored::*;
use random_strings::random_alphanumeric_string;

use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

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
    let total_size = safe_args.count as u64;
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    // while downloaded < total_size {

    // }

    for i in 0..safe_args.count {
        let new = min(i, safe_args.count) as u64;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(1000));

        let filename_base = random_alphanumeric_string(20);
        let filename = safe_args.full_filename(&filename_base);
        let file_body = match safe_args.size {
            Some(s) => random_alphanumeric_string(s),
            None => filename.clone(),
        };

        // dbg!("{}", i);
        // dbg!("- Full filename: {}", &filename);
        // dbg!("- Body contents: {}", &file_body);

        let write_result =
            generate_files::create_file(filename, file_body, safe_args.subdirectory.clone());
        match write_result {
            Ok(_) => {}
            Err(e) => println!("{}", e.to_string().red()),
        }
    }

    pb.finish_with_message("Done!");
}
