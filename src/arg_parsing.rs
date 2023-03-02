use num_format::{Locale, ToFormattedString};
use std::fmt;

/// Parse CLI arguments using the crate Clap.
pub fn parse_args() -> Result<Arguments, String> {
    use clap::{Arg, Command};

    let default_output_subdirectory = "generated_files";

    let count_arg_id = "count";
    let size_arg_id = "size";
    let prefix_arg_id = "prefix";
    let extension_arg_id = "extension";
    let delay_arg_id = "delay";
    let subdirectory_arg_id = "output_subdirectory";

    let matches = Command::new("Unique File Generator")
        .version("0.1.0")
        .author("CodeConscious (http://www.github.com/codeconscious/unique-file-generator-rust)")
        .about(
            "Easily create an arbitrary number of unique (by name and content) files.".to_owned()
                + "\nEach filename contains a random collection of characters."
                + "\nSupply optional parameters to customize files according to your needs.",
        )
        .arg(
            Arg::new(count_arg_id)
                .required(true) // Not ideal, per documentation
                .short('c')
                .long("count")
                .value_name("FILE_COUNT")
                .help("File count (Required)"),
        )
        .arg(
            Arg::new(size_arg_id)
                .required(false)
                .short('s')
                .long("size")
                .value_name("SIZE_IN_BYTES")
                .visible_alias("bytes")
                .help("Size in bytes of each file"),
        )
        .arg(
            Arg::new(prefix_arg_id)
                .required(false)
                .short('p')
                .long("prefix")
                .value_name("PREFIX")
                .help("Desired filename prefix"),
        )
        .arg(
            Arg::new(extension_arg_id)
                .required(false)
                .short('e')
                .long("extension")
                .value_name("FILE_EXTENSION")
                .help("File extension (Opening period is optional)"),
        )
        .arg(
            Arg::new(delay_arg_id)
                .required(false)
                .short('d')
                .long("delay")
                .value_name("DELAY_IN_MS")
                .default_value("0")
                .help("Number of milliseconds to sleep between each file"),
        )
        .arg(
            Arg::new(subdirectory_arg_id)
                .required(false)
                .long("output_directory")
                .short('o')
                .value_name("SUBDIRECTORY_NAME")
                .default_value(default_output_subdirectory)
                .help("The output subdirectory, which will be created if needed"),
        )
        .get_matches();

    let count = matches.get_one::<String>(count_arg_id).unwrap().to_owned();
    let size = matches.get_one::<String>(size_arg_id).map(String::from);
    let prefix = matches.get_one::<String>(prefix_arg_id).map(String::from);
    let extension = matches
        .get_one::<String>(extension_arg_id)
        .map(String::from);
    let subdirectory = matches
        .get_one::<String>(subdirectory_arg_id)
        .unwrap()
        .to_owned();
    let delay = matches.get_one::<String>(delay_arg_id).map(String::from);

    Arguments::new(count, size, prefix, extension, subdirectory, delay)
}

#[derive(Debug)]
pub struct Arguments {
    /// The number of files to create.
    pub count: usize,
    /// The size of each file in bytes.
    pub size: Option<usize>,
    /// The subdirectory into which the files should be created.
    pub subdirectory: String,
    /// Text that should be prepended to the filename.
    prefix: Option<String>,
    /// The extension of each file.
    extension: Option<String>,
    /// The durations, in milliseconds, to sleep between each file's creation.
    pub delay: u64,
}

impl Arguments {
    fn new(
        count: String,
        size: Option<String>,
        prefix: Option<String>,
        extension: Option<String>,
        subdirectory: String,
        delay: Option<String>,
    ) -> Result<Self, String> {
        let parsed_count = match count.parse::<usize>() {
            Ok(n) => {
                if n == 0 {
                    return Err("The file count must be greater than 0.".to_string());
                } else {
                    n
                }
            }
            Err(_) => {
                return Err(format!(
                    "\"{}\" is not a valid number for the count.",
                    count
                ))
            }
        };

        let parsed_size = match size {
            None => None,
            Some(s) => match s.parse::<usize>() {
                Ok(n) => {
                    if n == 0 {
                        return Err("The size must be greater than 0.".to_string());
                    } else {
                        Some(n)
                    }
                }
                Err(_) => return Err(format!("\"{}\" is not a valid number of bytes.", s)),
            },
        };

        let delay = match delay {
            None => 0,
            Some(s) => match s.parse::<u64>() {
                Ok(n) => n,
                Err(_) => return Err(format!("\"{}\" is not a valid number of milliseconds.", s)),
            },
        };

        Ok(Arguments {
            count: parsed_count,
            size: parsed_size,
            prefix,
            extension,
            subdirectory,
            delay,
        })
    }

    pub fn expected_operation_size(&self, default_file_size_bytes: usize) -> usize {
        match self.size {
            Some(s) => self.count * s,
            None => self.count * default_file_size_bytes,
        }
    }

    /// Prepends a specified prefix, if any, and appends a specified
    /// extension, if any, converting a base filename into a full one.
    pub fn full_filename(&self, base: &str) -> String {
        let prefix = match &self.prefix {
            Some(p) => p.to_owned(),
            None => "".to_owned(),
        };

        match &self.extension {
            Some(ext) => {
                let sanitized_extension = if ext.starts_with(".") {
                    ext[1..].to_owned()
                } else {
                    ext.to_owned()
                };
                prefix + base + "." + sanitized_extension.as_ref()
            }
            None => prefix + base,
        }
    }
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unspecified_text = "(Unspecified)";

        let size_phrase = match self.size {
            Some(s) => {
                let byte_label = if s == 1 { " byte" } else { " bytes" };
                s.to_formatted_string(&Locale::en) + byte_label
            }
            None => unspecified_text.to_string(),
        };

        let prefix_phrase = match &self.prefix {
            Some(p) => p,
            None => unspecified_text,
        };

        let extension_phrase = match &self.extension {
            Some(e) => e,
            None => unspecified_text,
        };

        let file_label = if self.count == 1 { "file" } else { "files" };
        let mut output = format!(
            "{:14} {} {}",
            "Count:",
            self.count.to_formatted_string(&Locale::en),
            file_label
        );
        output.push_str(&format!("\n{:14} {}", "Size:", size_phrase));
        output.push_str(&format!("\n{:14} {}", "Prefix:", prefix_phrase));
        output.push_str(&format!("\n{:14} {}", "Extension:", extension_phrase));
        output.push_str(&format!("\n{:14} {}", "Subdirectory:", self.subdirectory));

        writeln!(f, "{}", output)
    }
}
