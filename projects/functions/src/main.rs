fn main() {
    
    // This is a statement but the 6 is an expression that evaluates to 6.
    // Expressions can be part of statements.
    let y = plus_one(6);
    println!("The value is: {y}");
    
}

fn plus_one(x: i32) -> i32 {
    x+1
    // Note no semicolon here, if it was it would be a statement which returns
    // an empty value and would give a mismatched types error.
}