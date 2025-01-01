use std::io;

pub fn input_output(){
    loop{
        println!("Enter a number: ");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
    
        match num.trim().parse::<i32>() {
            Ok(n) => {
                let result = n * 2;
                println!("Result: {}", result);
                break;
            }
            Err(_) => {println!("Please enter a valid integer")},
        }
    }
    
}
