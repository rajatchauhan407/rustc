pub fn take_ownership(s: String) {
    println!("I own this string {}", s);
}
pub fn borrow_ownership(s: &mut String)-> usize{

    println!("I borrow this string {}", s);
    s.push_str(" and I change it now");
    s.len()
}

//assignment

pub fn modufy_string(s: &mut String){
    s.push_str(" and I changed it");
}