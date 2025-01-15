use std::collections::HashMap;

pub fn hashmap_init(){
    let mut student_scores:HashMap<String, i32> = HashMap::new();
    student_scores.insert(String::from("Robin"), 10);
    student_scores.insert(String::from("John"), 20);
    student_scores.insert(String::from("Doe"), 30);
    student_scores.insert(String::from("Jane"), 40);

    println!("{:?}", student_scores);

    
    let student_name:String = String::from("Rajat");

    student_scores.entry(student_name.to_string()).and_modify(
        |score| {*score += 10}
    ).or_insert(50);

    match student_scores.get(&student_name){
        Some(score) => println!("Score of Robin is {}", score),
        None => println!("No score found for {}", student_name)
    }
    for (key, value) in &student_scores{
        println!("{}: {}", key, value);
    }   
}


pub fn frequency_counter(){
    let text = "Hello world wonderful world";

    let mut word_count:HashMap<String,i32> = HashMap::new();

    for word in text.split_whitespace(){
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    // Print the frequency table
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

}

