use std::arch::x86_64;

pub fn check_positive(){
    let number = 5;
    if number < 0{
        panic!("Number is negative");
    }
    println!("Number is positive");
}

#[derive(Debug)]
pub enum MyError {
    InvalidInput(String),
    CalculationError(String),
    NotFound(String)
}

pub fn example_panic(input:i32)->Result<(),MyError>{
    if input<0 {    
            return Err(MyError::InvalidInput(format!("Invalid input: {}", input)));
    }   
    if input == 0 {
        return Err(MyError::CalculationError("Division by zero error".to_string()));
    }

    println!("Input is value  : {}", input);

    Ok(())

}

pub fn validator(input: &str)->Result<(),MyError>{
        let number:i32 = match input.parse(){
            Ok(num) => num,
            Err(_) => return Err(MyError::InvalidInput("Invalid input".to_string()))
        };

        // Check if the number is non-negative
    if number < 0 {
        return Err(MyError::CalculationError(format!(
            "Number {} is negative. Only non-negative numbers are allowed.",
            number
        )));
    }

    // Check if the number is within a specific range
    if number > 100 {
        return Err(MyError::CalculationError(format!(
            "Number {} is out of range. Must be 0-100.",
            number
        )));
    }

        Ok(())
}

pub fn check_force_panic(){
    let inputs = vec!["-10", "200", "50", "abc"];

    for input in inputs {
        match validator(input) {
            Ok(()) => println!("'{}' is valid!", input),
            Err(e) => println!("Error for '{}': {:?}", input, e),
        }
    }
}