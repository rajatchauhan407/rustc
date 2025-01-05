pub fn string_slice(){
    let s = String::from("Rust is a systems programming language");
    let s1 = &s[0..4];
    println!("{}", s1);
    

    //original string still accessible
    println!("{}", s);

    let mut arr:[i32;5] = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..4];
    slice[0]=200;

    println!("{:?}", arr);
}