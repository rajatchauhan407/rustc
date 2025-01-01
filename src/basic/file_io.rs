use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
// Reasing Files in Rust
pub fn file_io(){
    let file_path = "src/basic/example.txt";
    let path = Path::new(file_path);
    if path.exists(){
        println!("File exists");
        match fs::read_to_string(file_path){
            Ok(content)=>{
                println!("Content:\n {}", content);
            }
            Err(err)=>{
                eprintln!("Could not open the text file: {}", err);
            }
        }
    }else{
        eprintln!("File does not exist");
    }
}

// Writing Files in Rust
pub fn file_io_write(){
    let file_path = "src/basic/output.txt";
    // let path = Path::new(file_path);
    let content = "Hello, World from Rust Learning Basics!";
    match fs::write(file_path, content){
        Ok(_)=>{
            println!("File created successfully");
        }
        Err(err)=>{
            eprintln!("Could not create the file: {}", err);
        }
    }
}
// Appending using open options
pub fn file_io_append() {
    let file_path = "src/basic/example.txt";
    let content = "This is the recent appended line.\n";

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .write(true)   // Enable writing
        .append(true)  // Enable appending
        .open(file_path)
        .expect("Cannot open the file");

    // Write the new content
    file.write_all(content.as_bytes())
        .expect("Failed to append data");

    println!("Content appended successfully!");
}