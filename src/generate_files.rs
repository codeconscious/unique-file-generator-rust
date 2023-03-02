use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use num_format::Locale;
use num_format::ToFormattedString;

use crate::arg_parsing::Arguments;

pub fn verify_continue(args: &Arguments) -> bool {
    println!(
        "This operation will take approximately {} bytes of space on your drive.",
        args.expected_operation_size(20)
            .to_formatted_string(&Locale::en)
    );

    loop {
        let mut response = String::new(); // Must be within this loop.
        print!("Continue? (Y/n)  ");
        io::stdout().flush().unwrap(); // The previous one will not display immediately with this.
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to read from stdin!");

        match response.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => {
                continue;
            }
        }
    }
}

pub fn create_file(filename: String, content: String, subfolder: String) -> std::io::Result<()> {
    prepare_subdirectory(&subfolder)?;

    let path = Path::new(".").join(subfolder).join(filename);
    let mut file = File::create(&path).expect(
        format!(
            "Error encountered while creating file \"{}\"!",
            path.to_string_lossy()
        )
        .as_str(),
    );

    file.write_all(content.as_bytes()).expect(
        format!(
            "Error while writing content to file \"{}\"!",
            path.to_string_lossy()
        )
        .as_str(),
    );
    Ok(())
}

fn prepare_subdirectory(directory_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(directory_name)?;
    Ok(())
}
