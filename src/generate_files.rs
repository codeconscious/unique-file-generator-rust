use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn prepare_subdirectory(directory_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(directory_name)?;
    Ok(())
}

pub fn create_file(filename: String, content: String, subfolder: String) -> std::io::Result<()> {
    prepare_subdirectory(&subfolder)?;

    let path = subfolder + "/" + filename.as_ref(); // TODO: OS-specific separator?
    let mut file = File::create(path).expect("Error encountered while creating file!");
    file.write_all(content.as_bytes())
        .expect("Error while writing content to file!");
    Ok(())
}
