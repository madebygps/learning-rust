use std::io;
fn main() {
    loop {
        println!("Input your temperature: ");
        let mut temperature = String::new();
        loop {
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");
            let temperature: f64 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            let converted_temperature = (temperature - 32.0) * (5.0 / 9.0);
            println!("{temperature} in celsius that would be: {converted_temperature}");
            break;
        }
    }
}
