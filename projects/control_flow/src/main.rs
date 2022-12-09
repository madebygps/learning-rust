use std::num;

fn main() {
    
    let condition = true;

    let number = if condition {
        5
    } else {
        6
        //"Six" wouldn't work because of mismatch
    };
    println!("The value of number is: {number}");
    
}
