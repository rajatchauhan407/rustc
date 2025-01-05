mod basic;
mod math;
mod prog_mod;
// use math::operations;
// use basic::basic10;
use basic::tuple_array;
use basic::input_output;
use basic::file_io;
use basic::control;
use basic::own_borrow;
use basic::slice;
fn main() {
    // let ans: i32 = tuple_array::arrs(5);
    // println!("Array: {}", ans);
    
     /******************************************
     *                                        *
     *  src file path: src/basic/slice.rs*
     *                                        *
     * ****************************************/
    slice::string_slice();
    slice::first_word();
    slice::array_middle();
    slice::word_counter();

    /******************************************
     *                                        *
     *  src file path: src/basic/own_borrow.rs*
     *                                        *
     * ****************************************/
    // let my_string = String::from("Hello, Rust!");
    // own_borrow::take_ownership(my_string);
    // println!("my_string: {}", my_string);

    // Borrow Ownership
    // let mut my_string = String::from("Hello, Rust!");
    // let res:usize = own_borrow::borrow_ownership(&mut my_string);
    // println!("my_string: {} and size is {}", my_string, res);
    // println!("my_string: {}", my_string);

    // let mut new_string = String::from("Hello");
    
    
    // {  
    //     let r1:&mut String = &mut new_string; //first mutable borrow
    //     r1.push_str(", World!");
    //     println!("{}", r1);
    // } //now r1 goes out of scope
    
    // let r2:&mut String = &mut new_string;
    // r2.push_str(", How are you!");
    // println!("{}", r2);


    //ownership assignment

    // let mut my_str = String::from("Hello World");
    // own_borrow::modufy_string(&mut my_str);
    // println!("{}", my_str);
    // let r1 = &my_str;
    // println!("{:p}", r1);


   
    /***************************************
     *                                     *
     *  src file path: src/basic/control.rs*
     *                                     *
     * ************************************/
        // control::contro_if(10);
        // let res:i32= control::factorial_calc_assign(10);  
        // println!("Factorial of 10 is {}",res);
        // let a:[i32;5] = [1,2,3,4,5];
        // let b:i32 = control::loop_iterable(a);
        // println!("Sum of 1 to {} is {}",5,b);
        // let x = 42;
        // let y = &x; // Borrow x
        // println!("y is a reference: {:p}", y); // Access the reference
        // println!("Dereferenced y: {}", *y); 
        // let z:i32 = control::loop_loop(5);
        // println!("Sum of 1 to {} is {}",5,z);
        // let x:i32 = control::loop_for(5);
        // let y:i32 = control::loop_while(5);
        // println!("Sum of 1 to {} is {}",5-1,y);
        // let mut x:i32 = control::square(5);
        // println!("square of {} is {}",5,x);
        // x = 7;
        // println!("square of {} is {}",7,control::square(x));

    /***************************************
     *                                     *
     *  src file path: src/basic/file_io.rs*
     *                                     *
     * ************************************/
    // file_io::file_io();
    // file_io::file_io_write();
    // file_io::file_io_append();
    /***************************************
     *                                     *
     *  src file path: src/basic/input.rs  *
     *                                     *
     * ************************************/

    // input_output::input_output();

    /******* 
     * src file path: src/basic/tuple_array.rs 
     * 
     * ********/ 
    // tuple_array::print_address();

    // tuple_array::stack_heap_alloc();

    // tuple_array::pointer_arithematic();


    // let x:i32 = 5;
    // println!("the value of x is : {}",x);
    // let x = x+2;
    // println!("the value of shadowed x is : {}",x);
    // let mut x:i32 = x+2;
    // x=35;
    // println!("the value of mutable x is : {}",x);
    //    let sum = operations::add(9, 5);
    //    let sub = operations::sub(15, 9);
    //    println!("Sum: {}", sum);
    //    println!("Sub: {}", sub);
    //    println!("Multiplication:{}",operations::multiply(5,5));
    // basic10::greet();
    // basic10::calculate(32, 42);
    // println!("Factorial: {}", basic10::fac(6));
    // println!("{}", basic10::even_odd(5));
}
