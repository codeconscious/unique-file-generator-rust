mod arg_parsing;
mod generate_files;
mod random_strings;

use arg_parsing::{parse_args, Arguments};
use colored::*;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use random_strings::random_alphanumeric_string;
use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

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
    let progress_bar = prepare_progress_bar(&safe_args);

    for i in 0..safe_args.count {
        let new = min(i, safe_args.count) as u64;
        progress_bar.set_position(new);
        thread::sleep(Duration::from_millis(safe_args.delay));

        let filename_base = random_alphanumeric_string(20);
        let filename = safe_args.full_filename(&filename_base);
        let file_body = match safe_args.size {
            Some(s) => random_alphanumeric_string(s),
            None => filename.clone(),
        };

        let write_result =
            generate_files::create_file(filename, file_body, safe_args.subdirectory.clone());
        match write_result {
            Ok(_) => {}
            Err(e) => println!("{}", e.to_string().red()),
        }
    }

    progress_bar.finish_with_message("Done!");
}

fn prepare_progress_bar(args: &Arguments) -> ProgressBar {
    let total_size = args.count as u64;

    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    progress_bar
}
