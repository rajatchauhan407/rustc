pub fn iflet_1(){
    let user_favourite_color:Option<&str> = Some("Blue");

    if let Some(color) = user_favourite_color{
        println!("User's favourite color is {}", color);
    }else{
        println!("User has not set any favourite color");
    }
}

pub fn match_tuples(){
    let point:(i32, i32) = (3, 0);


    match point {
        (x,y) if x>0 && y>0 => println!("Point lies in the first quadrant"),
        (x,y) if x<0 && y>0 => println!("Point lies in the second quadrant"),
        (x,y) if x<0 && y<0 => println!("Point lies in the third quadrant"),
        (x,y) if x>0 && y<0 => println!("Point lies in the fourth quadrant"),
        (0,0) => println!("Point is at origin"),
        (x,0) => println!("Point lies on x-axis at x = {}",x),
        (0,y) => println!("Point lies on y-axis at y = {}",y),
        _ => println!("Point is somewhere else")
    }
}

pub fn match_guard(){
    let number:Option<i32> = Some(8);

    match number {
        Some(x) if x%2 == 0 => println!("Even number"),
        Some(x) if x%2 != 0 => println!("Odd number"),
        None => println!("No value"),
        _ => println!("Invalid value")
    }
}

pub fn even_odd_input(){
    let mut input = String::new();
    println!("Enter a number: ");
    std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    let new_num = input.trim();

    match new_num.parse::<i32>() {
        Ok(num)=>{
            match num {
                x if x%2 == 0 => println!("Even number"),
                _ => println!("Odd number")
            }
        }
        Err(_) => println!("Invalid value! Please enter a valid integer")        
    }
}