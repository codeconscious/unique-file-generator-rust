use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::arg_parsing::Arguments;

pub fn should_continue(args: &Arguments) -> bool {
    println!(
        "This operation will take approximately {} of space on your drive.",
        args.expected_operation_size(20)
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
            "yes" => return true,
            "n" => return false,
            "no" => return false,
            _ => {
                continue;
            }
        }
    }
}

pub fn create_file(filename: String, content: String, subfolder: String) -> std::io::Result<()> {
    prepare_subdirectory(&subfolder)?;

    let path = Path::new(".").join(subfolder).join(filename);
    let mut file = File::create(&path).unwrap_or_else(|_| {
        panic!(
            "Error encountered while creating file \"{}\"!",
            path.to_string_lossy()
        )
    });

    file.write_all(content.as_bytes()).unwrap_or_else(|_| {
        panic!(
            "Error while writing content to file \"{}\"!",
            path.to_string_lossy()
        )
    });
    Ok(())
}

fn prepare_subdirectory(directory_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(directory_name)?;
    Ok(())
}
