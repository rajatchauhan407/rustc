pub fn arrs(n:usize)->i32{
    let arr:[i32;6] = [3,-9,2,4,56,6];
    return arr[n];
}

// understanding memory with arrays

pub fn print_address(){
    let arr:[i32;5] = [1,2,3,4,5];
    for i in 0..arr.len(){
        let address= &arr[i] as *const i32;
        println!("Address of element {} is: {:?}",i,address);
    }
}

//pointer and unsafe code
pub fn pointer_arithematic(){
    let arr:[i32;5] = [1,2,3,4,5];
    let ptr = arr.as_ptr();

    unsafe{
        println!("Base address: {:?}",ptr);
        println!("Value at base address or index 0: {:?}",*ptr);
        println!("Value at index 2: {:?}",*ptr.add(2));
        println!("address of index 5: {:?}",*ptr.add(5));
    }

     // Allocate more stack variables
     let temp = [10, 20, 30];
     println!("Temporary array: {:?}", temp);
 
     unsafe {
         // Check index 5 again
         println!("Value at index 5 after adding temp: {:?}", *ptr.add(5));
     }
}

// stack and heap memory allocation
pub fn stack_heap_alloc(){
    let a = 100;
    let b = Box::new(200);

    // Stack address (local variable)
    println!("Address of a: {:p}", &a);

    // Heap address (Box points to heap)
    println!("Address of b: {:p}", b);
}