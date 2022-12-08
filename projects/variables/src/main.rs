fn main() {
    
    // x =1
    let mut x = 1;
    {
        let mut x = x;
        x += 2;
        // x = 3
    }
    println!("{x}");
    // x = 1
}
