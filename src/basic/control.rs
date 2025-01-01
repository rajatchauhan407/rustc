pub fn square(x:i32)->i32{
    x*x
}

pub fn loop_for(n:i32)->i32{
    let mut sum = 0;
    let iter = 1..n;
    for i in iter{
        sum += i;
    }
    return sum;
}

pub fn loop_while(n:i32)->i32{
    let mut sum:i32 = 0;
    let mut i:i32 = 0;
    while i<=5{
        // println!("the value of i is : {}",i);
        sum+=i;
        i+=1;
    }
    return sum;
}

pub fn loop_loop(n:i32)->i32{
    let mut sum:i32 = 0;
    let mut i:i32 = 0;
    loop{
        if(i>n){
            break;
        }
        sum=sum+i;  
        i+=1;
    }
    return sum;
}
pub fn loop_iterable(arr:[i32;5])->i32{
   return arr.iter().sum();
}
pub fn contro_if(x:i32){
    
    if x % 2 == 0 {
        println!("x is even");
    } else if x % 2 == 1 {
        println!("x is odd");
    } else {
        println!("No Idea");
    }
}

pub fn factorial_calc_assign(n:i32)->i32{
    let mut fact:i32 = 1;
    let mut i:i32 = 1;
    while i<=n{
        fact*=i;
        i+=1;
    }
    return fact;

}