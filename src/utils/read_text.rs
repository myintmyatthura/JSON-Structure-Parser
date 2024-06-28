use std::fs; // Import the fs module from the standard library for file operations
use std::io::Write; // Import the Write trait for writing to files
use std::path::PathBuf; // Import the PathBuf struct for handling file paths

const INPUT_PATH: &str = "../input/file1.txt"; // Define a constant for the input file path
const OUTPUT_PATH: &str = "../output/file1.txt"; // Define a constant for the output file path

// Function to read text from the input file
pub fn read_text() -> String {
    // Create a PathBuf for the current file
    let current_file = PathBuf::from(file!());
    // Navigate to the parent directory of the current file
    let parent_dir = current_file
        .parent()
        .expect("Failed to get parent directory\n");

    // Create a relative path to file1.txt
    let file1_path = parent_dir.join(INPUT_PATH);

    // Check if the input file exists
    if file1_path.exists() {
        println!("This exists, the path is correct!\n");
    } else {
        // Print an error message if the file does not exist and return an empty string
        println!("This doesn't exist, please check path.\n");
        return "".to_string();
    }

    // Read the contents of the input file into a string
    match fs::read_to_string(file1_path) {
        Ok(contents) => contents, // Return the contents if reading is successful
        Err(err) => {
            // Print an error message if reading fails and return an empty string
            eprintln!("Error reading file at: {:#?}", err);
            "".to_string()
        }
    }
}

// Function to write text to the output file
pub fn write_text(contents: &str) {
    // Create a PathBuf for the current file
    let current_file = PathBuf::from(file!());
    // Navigate to the parent directory of the current file
    let parent_dir = current_file
        .parent()
        .expect("Failed to get parent directory\n");

    // Create a relative path to the output file
    let file1_path = parent_dir.join(OUTPUT_PATH);

    // Attempt to create (or truncate if it exists) the output file
    match fs::File::create(&file1_path) {
        Ok(mut file) => {
            // Write the provided contents to the output file
            if let Err(err) = file.write_all(contents.as_bytes()) {
                // Print an error message if writing fails
                eprintln!("Error writing to file: {:#?}", err);
            }
        }
        Err(err) => {
            // Print an error message if file creation fails
            eprintln!("Error creating file: {:#?}", err);
        }
    }
}
