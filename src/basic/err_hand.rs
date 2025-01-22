use std::result::Result;
use std::fs::File;
use std::io::{self, Read, Write};
pub fn safe_divide(a:f64, b:f64) -> Result<f64, String>{
    if b==0.0{
        Err("Cannot divide by zero".to_string())
    }else{
        Ok(a/b)
    }
}


pub fn calculate(a:f64,b:f64, c:f64)->Result<f64,String>{
    let result = safe_divide(a,b)?;
    let final_result = safe_divide(result,c)?;
    Ok(final_result)
}

pub fn result_err_hand(){
    match calculate(10.0,2.0,0.0){
        Ok(result) => println!("Result is {}", result),
        Err(e) => println!("Error: {}", e)
    }
}

// Reading File contents

pub fn read_file_contents(filename:&str)-> Result<String,io::Error>{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn handle_file(){
    let filename = "src/basic/output.txt";
    match read_file_contents(filename){
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => eprintln!("Error reading the file: {}", e)
    }
}

pub fn read_file_safely(filename:&str)-> Result<String,io::Error>{
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(e) => {
            // Check if the error is due to the file not existing
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("File '{}' not found. Creating a new file.", filename);
                // Create the file and return a default message
                let mut file = File::create(filename)?;
                file.write_all(b"This is a new file created by safe_reader.\n")?;
                Ok(String::from("File created. It was empty, so default content has been added."))
            } else {
                // Propagate other errors
                Err(e)
            }
        }
    }
}

pub fn safe_reader() {
    let filename = "src/basic/friendly.txt";
    match read_file_safely(filename) {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => eprintln!("Error reading or creating the file '{}': {}", filename, e),
    }
}