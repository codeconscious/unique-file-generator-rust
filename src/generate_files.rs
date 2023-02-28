use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
