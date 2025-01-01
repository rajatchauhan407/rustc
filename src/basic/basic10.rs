pub fn greet() {
    println!("Hello, world!");
}

pub fn calculate(a: i32, b: i32) {
    println!("Sum: {}", a + b);
    println!("Sub: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);
}

pub fn fac(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * fac(n - 1)
    }
}

pub fn even_odd(n: i32) -> String {
    if n % 2 == 0 {
        return "it is even".to_string();
    } else {
        "✅ it is odd".to_string()
    }
}
