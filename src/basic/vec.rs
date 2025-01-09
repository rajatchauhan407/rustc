pub fn vectors(){
    // create an empty vector
    // let mut v: Vec<i32> = Vec::new();
    
    //creating a vector with values
    let v = vec![1, 2, 3];
    // safely accessing elements in a vector
    match v.get(5){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    // Immutable Iterations
    for i in &v{
        println!("{}", i);
    }
}

pub fn vectors_2()->Vec<i32> {
    let mut v:Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{}",v[1]);

    // Mutable Iterations
    for value in &mut v{
        *value *= 2;
    }
    println!("{:?}", v);

    return v;
}


pub fn calculate_stats(v: &Vec<i32>) -> (f32, i32, i32) {
    let sum: i32 = v.iter().sum(); // Sum of all elements
    let avg = sum as f32 / v.len() as f32; // Average
    let min = *v.iter().min().unwrap(); // Minimum
    let max = *v.iter().max().unwrap(); // Maximum

    (avg, min, max)
}