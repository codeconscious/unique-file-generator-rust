use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Unique File Generator")
        .version("0.1.0")
        .author("CodeConscious (http://www.github/codeconscious/unique-file-generator-rust)")
        .about(
            "Quickly and easily create an arbitrary number of unique (by name and content) files.".to_owned() +
            "\nEach filename contains a random collection of characters.  " +
            "\nSupply optional parameters to customize files according to your needs.  " +
            "\nThe tool checks that there is sufficient drive space available before starting."
        )
        .arg(
            Arg::new("count")
                .required(true) // Not ideal, per documentation
                .short('c')
                .long("count")
                .value_name("FILE_COUNT")
                .help("The count of files to generate (Required)"),
        )
        .arg(
            Arg::new("size")
                .required(false)
                .short('s')
                .long("size")
                .value_name("SIZE_IN_BYTES")
                .visible_alias("bytes")
                .help("The size of files to generate"),
        )
        .arg(
            Arg::new("prefix")
                .required(false)
                .short('p')
                .long("prefix")
                .value_name("PREFIX")
                .help("Desired filename prefix"),
        )
        .arg(
            Arg::new("ext")
                .required(false)
                .short('e')
                .long("extension")
                .value_name("FILE_EXTENSION")
                .help("File extension, with optional opening period"),
        )
        .arg(
            Arg::new("dir")
                .required(false)
                .long("directory")
                .short('d')
                .value_name("DIRECTORY_NAME")
                .help("The output subdirectory, which will be created if needed (Defaults to \"output\")"),
        )
        .get_matches();

    let requested_count = matches.get_one::<String>("count").unwrap();
    match requested_count.parse::<u32>() {
        Ok(count) => println!("Will generate {} file(s).", count),
        Err(_) => println!("\"{}\" is not a valid number!", requested_count),
    };

    let size_str = matches.get_one::<String>("size");
    match size_str {
        None => println!("No file size was specfied."),
        Some(s) => match s.parse::<u32>() {
            Ok(n) => println!("Files will be {} bytes.", n),
            Err(_) => println!("\"{}\" is not a valid number!", s),
        },
    }

    let prefix = matches.get_one::<String>("prefix");
    match prefix {
        None => println!("No prefixes will be prepended."),
        Some(s) => println!("Filenames will be prepended with \"{}\".", s),
    }

    let extension = matches.get_one::<String>("ext");
    match extension {
        None => println!("No extensions will be appended."),
        Some(s) => println!("Files will be given the extension \"{}\".", s),
    }

    let requested_directory = matches.get_one::<String>("dir");
    let directory = match requested_directory {
        None => "output",
        Some(s) => s,
    };
    println!("Files will be saved to subdirectory \"{}\".", directory);
}
