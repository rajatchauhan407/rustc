use std::io;

pub fn str_manipulation(){
    let mut str:String = String::from("Hello, ");

    str.push_str("World!");
    println!("{}", str);
}

pub fn str_slicing(){
    // all the asciii are one byte each but the hindi character is 3 bytes
    let str = String::from("Hello, World!");
    let hello = &str[0..3];
    let world = &str[7..12];
    println!("{} {}", hello, world);

    let str2 = String::from("नमस्ते");
    let slice = &str2[0..18];

    for (i,ch) in str2.char_indices(){
        println!("Index:{}, Char: {}", i,ch);
    }
    // let slice = &str2[0..5]; // this will panic
    println!("{}", slice);
}

pub fn str_replace(){
    let str = String::from("I love programming in javascript");
    let new_str = str.replace("javascript", "Rust");
    println!("{}", new_str);
}

pub fn str_reverse(){
    let mut str:String = String::new();
    println!("Enter a string: ");
    io::stdin().read_line(&mut str).expect("Failed to read input");
    let str = str.trim();
    let rev_str = str.chars().rev().collect::<String>();
    println!("Reverse of {} is {}", str, rev_str);
}