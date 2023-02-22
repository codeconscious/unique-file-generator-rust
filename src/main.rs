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
                .help("The size in bytes of files to generate"),
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

    let count = matches.get_one::<String>("count").unwrap();
    let size = matches.get_one::<String>("size");
    let prefix = matches.get_one::<String>("prefix");
    let extension = matches.get_one::<String>("ext");
    let requested_directory = matches.get_one::<String>("dir").unwrap();

    let _arguments = Arguments::new(count, size, prefix, extension, requested_directory);
}

#[allow(dead_code)]
struct Arguments<'a> {
    count: u32,
    size: Option<u32>,
    prefix: Option<&'a String>,
    extension: Option<&'a String>,
    subdirectory: &'a String,
}

impl<'a> Arguments<'a> {
    fn new(
        count: &String,
        size: Option<&String>,
        prefix: Option<&'a String>,
        extension: Option<&'a String>,
        subdirectory: &'a String,
    ) -> Result<Self, &'a str> {
        let parsed_count = match count.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err("\"{count}\" is not a valid number!"),
        };

        let parsed_size = match size {
            None => None,
            Some(s) => match s.parse::<u32>() {
                Ok(n) => Some(n),
                Err(_) => return Err("\"{s}\" is not a valid number!"),
            },
        };

        Ok(Arguments {
            count: parsed_count,
            size: parsed_size,
            prefix,
            extension,
            subdirectory,
        })
    }
}
