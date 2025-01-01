mod basic;
mod math;
mod prog_mod;
// use math::operations;
// use basic::basic10;
use basic::tuple_array;
use basic::input_output;
use basic::file_io;
use basic::control;
fn main() {
    let ans: i32 = tuple_array::arrs(5);
    println!("Array: {}", ans);


    /***************************************
     *                                     *
     *  src file path: src/basic/control.rs*
     *                                     *
     * ************************************/
     let mut x:i32 = control::square(5);
     println!("square of {} is {}",5,x);
     x = 7;
     println!("square of {} is {}",7,control::square(x));

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
