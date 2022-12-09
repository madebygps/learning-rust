use std::io;
fn main() {
    loop {
        println!("Input your number: ");
        let mut number = String::new();
        loop {
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line");
            let number: i32 = match number.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            if number == 0 {
                println!("f(0) doesn't exist");
            } else if number == 1 {
                println!("f(1): 1")
            } else {
                calculate_fibonacci(number);
            }

            break;
        }
    }
}

fn calculate_fibonacci(n: i32) {
    let mut sum = 0;
    let mut current = 1;
    let mut last = 0;

    for _i in 1..n {
        sum = last + current;
        last = current;
        current = sum;
    }

    println!("f({n}) = {sum}");
}
