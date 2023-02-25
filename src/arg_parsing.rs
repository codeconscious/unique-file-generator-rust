use std::fmt;

/// Parse CLI arguments using the crate Clap.
pub fn parse_args() -> Result<Arguments, String> {
    use clap::{Arg, Command};

    let matches = Command::new("Unique File Generator")
        .version("0.1.0")
        .author("CodeConscious (http://www.github.com/codeconscious/unique-file-generator-rust)")
        .about(
            "Quickly and easily create an arbitrary number of unique (by name and content) files.".to_owned() +
            "\nEach filename contains a random collection of characters." +
            "\nSupply optional parameters to customize files according to your needs.")
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

    let count = matches.get_one::<String>("count").unwrap().to_owned();
    let size = matches.get_one::<String>("size").map(String::from);
    let prefix = matches.get_one::<String>("prefix").map(String::from);
    let extension = matches.get_one::<String>("ext").map(String::from);
    let subdirectory = matches.get_one::<String>("dir").unwrap().to_owned();

    Arguments::new(count, size, prefix, extension, subdirectory)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Arguments {
    /// The number of files to create.
    count: u32,
    /// The size of each file in bytes.
    size: Option<u32>,
    /// Text that should be prepended to the filename.
    prefix: Option<String>,
    /// The extension of each file.
    extension: Option<String>,
    /// The subdirectory into which the files should be created.
    subdirectory: String,
}

impl Arguments {
    fn new(
        count: String,
        size: Option<String>,
        prefix: Option<String>,
        extension: Option<String>,
        subdirectory: String,
    ) -> Result<Self, String> {
        let parsed_count = match count.parse::<u32>() {
            Ok(n) => {
                if n == 0 {
                    return Err("The count must be greater than 0.".to_string());
                } else {
                    n
                }
            }
            Err(_) => return Err("\"{count}\" is not a valid number!".to_string()),
        };

        let parsed_size = match size {
            None => None,
            Some(s) => match s.parse::<u32>() {
                Ok(n) => {
                    if n == 0 {
                        return Err("The size must be greater than 0.".to_string());
                    } else {
                        Some(n)
                    }
                }
                Err(_) => return Err("\"{s}\" is not a valid number!".to_string()),
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

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_phrase = match self.size {
            Some(s) => s.to_string(),
            None => "(None)".to_string(),
        };

        let prefix_phrase = match &self.prefix {
            Some(p) => p,
            None => "(None)",
        };

        let extension_phrase = match &self.extension {
            Some(e) => e,
            None => "(None)",
        };

        let mut output = format!("Count:         {}", self.count);
        output.push_str(&format!("\nFile size:     {}", size_phrase));
        output.push_str(&format!("\nPrefix:        {}", prefix_phrase));
        output.push_str(&format!("\nExtension:     {}", extension_phrase));
        output.push_str(&format!("\nSubdirectory:  {}", self.subdirectory));

        writeln!(f, "{}", output)
    }
}
