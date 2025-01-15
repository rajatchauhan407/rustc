use std::result::Result;

pub fn safe_divide(a:f64, b:f64) -> Result<f64, String>{
    if b==0.0{
        Err("Cannot divide by zero".to_string())
    }else{
        Ok(a/b)
    }
}


pub fn calculate(a:f64,b:f64, c:f64)->Result<f64,String>{
    let result = safe_divide(a,b)?;
    let final_result = safe_divide(result,c)?;
    Ok(final_result)
}

pub fn result_err_hand(){
    match calculate(10.0,2.0,0.0){
        Ok(result) => println!("Result is {}", result),
        Err(e) => println!("Error: {}", e)
    }
}