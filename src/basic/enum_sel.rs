pub enum TrafficLight {
    Red,
    Yellow,
    Green
}

pub fn enum_1(){
    let mut input: String = String::new();
    println!("Enter the traffic light color: ");

    std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    let input = input.trim().to_lowercase();

    let light = match input.as_str(){
        "red" => TrafficLight::Red,
        "yellow" => TrafficLight::Yellow,
        "green" => TrafficLight::Green,
        _ => {
            println!("Invalid value !! Please enter Red, Yellow or Green");
            return;
        }
    };

    match light{
        TrafficLight::Green => println!("Go"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Red => println!("Stop")
    }
}

pub fn action(light: TrafficLight) -> &'static str{
    match light{
        TrafficLight::Green => "Go",
        TrafficLight::Yellow => "Slow down",
        TrafficLight::Red => "Stop"
    }
}

pub fn get_number(option:Option<i32>){
    match option{
        Some(number) => println!("Option is {}", number),
        None => println!("Nothing is Found")
    }
}
