use std::fs;
use std::path::PathBuf;

pub fn read_text() -> String {
    // Create a PathBuf for the current file
    let current_file = PathBuf::from(file!());
    // Navigate to the parent directory of the current file
    let parent_dir = current_file
        .parent()
        .expect("Failed to get parent directory\n");

    // Create a relative path to file1.txt
    let file1_path = parent_dir.join("../input/file1.txt");

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
