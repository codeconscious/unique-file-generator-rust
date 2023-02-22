// use clap::Parser;
use clap::{Parser, Subcommand};
// use clap::{Arg, App};

/// Test
#[derive(Parser)]
#[command(name = "Unique File Generator")]
#[command(author = "CodeConscious (http://www.github/codeconscious/unique-file-generator-rust)")]
#[command(version = "1.0")]
#[command(about = "Quickly and easily create an arbitrary number of unique (by name and content) files.", long_about = None)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// The count of files to generate
    // #[arg(short, long)]
    Count { name: String },

    /// The text to prefix to filenames
    Prefix { name: Option<String> },

    /// The desired extension for all files
    Extension { name: Option<String> },

    /// The desired output directory (Defaults to "output")
    OutputDirectory { name: Option<String> },

    /// The delay in milliseconds between each file's creation
    Delay { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();
}

// fn main() {
//    let matches = App::new("Unique File Generator")
//         .version("0.1.0")
//         .author("CodeConscious (http://www.github/codeconscious/unique-file-generator-rust)")
//         .about("Quickly and easily create an arbitrary number of unique (by name and content) files.")
//         .arg(Arg::with_name("count")
//                     .short("c")
//                     .long("count")
//                     .takes_value(true)
//                     .help("The count of files to generate"))
//         .arg(Arg::with_name("num")
//                     .short("n")
//                     .long("number")
//                     .takes_value(true)
//                     .help("Five less than your favorite number"))
//         .get_matches();

//     let count_str = matches.value_of("count");
//     match count_str {
//         None => println!("Please specify a file count."),
//         Some(s) => {
//             match s.parse::<u32>() {
//                 Ok(n) => println!("Will generate {} file(s).", n),
//                 Err(_) => println!("\"{}\" is not a number!", s),
//             }
//         }
//     }
// }
