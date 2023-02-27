use std::fs::File;
use std::io::prelude::*;

pub fn create_file(filename: String, content: String, subfolder: String) -> Result<(), String> {
    // let path = subfolder + "/" + filename.as_ref(); // TODO: OS-specific separator?
    let mut file = File::create(filename).expect("Error encountered while creating file!");
    file.write_all(content.as_bytes())
        .expect("Error while writing content to file!");
    Ok(())
}
