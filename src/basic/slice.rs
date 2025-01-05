pub fn string_slice(){
    let s = String::from("Rust is a systems programming language");
    let s1 = &s[0..4];
    println!("{}", s1);
    

    //original string still accessible
    println!("{}", s);

    let mut arr:[i32;5] = [1, 2, 3, 4, 5];
    let slice:&mut [i32] = &mut arr[1..4];
    slice[0]=200;

    println!("{:?}", arr);
}

pub fn first_word(){
    let str = String::from("Let there be light");

    // convert string to bytes
    let bytes:&[u8] = str.as_bytes();
    for(i,&byte) in bytes.iter().enumerate(){
        if byte == b' '{
            println!("First word: {}", &str[0..i]);
            break;
        }
    }
}

pub fn array_middle(){
    let arr:[i32;5] = [1, 2, 3, 4, 5];
    let str_len:usize = arr.len();
    let slice:&[i32] = &arr[1..str_len-1];

    println!("{:?}", slice);
}


pub fn word_counter(){
    let mut str_input:String = String::new();
    println!("Enter a string: ");

    std::io::stdin()
         .read_line(&mut str_input)
            .expect("Failed to read line");
    // Remove trailing newline and whitespace
    let trimmed_str:&str = str_input.trim();

    let word_count:usize = trimmed_str.split_whitespace().count();
    println!("Word count: {}", word_count);
}