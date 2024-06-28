use std::fs;
use std::io::Write;
use std::path::PathBuf;

const INPUT_PATH: &str = "../input/file1.txt";
const OUTPUT_PATH: &str = "../output/file1.txt";

pub fn read_text() -> String {
    // Create a PathBuf for the current file
    let current_file = PathBuf::from(file!());
    // Navigate to the parent directory of the current file
    let parent_dir = current_file
        .parent()
        .expect("Failed to get parent directory\n");

    // Create a relative path to file1.txt
    let file1_path = parent_dir.join(INPUT_PATH);

    if file1_path.exists() {
        println!("This exists, the path is correct!\n");
    } else {
        println!("This doesn't exist, please check path.\n");
        return "".to_string();
    }

    match fs::read_to_string(file1_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file at: {:#?}", err);
            "".to_string()
        }
    }
}

pub fn write_text(contents: &str) {
    // Create a PathBuf for the current file
    let current_file = PathBuf::from(file!());
    // Navigate to the parent directory of the current file
    let parent_dir = current_file
        .parent()
        .expect("Failed to get parent directory\n");

    // Create a relative path to the output file
    let file1_path = parent_dir.join(OUTPUT_PATH);

    // Write the string to the output file
    match fs::File::create(&file1_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(contents.as_bytes()) {
                eprintln!("Error writing to file: {:#?}", err);
            }
        }
        Err(err) => {
            eprintln!("Error creating file: {:#?}", err);
        }
    }
}
