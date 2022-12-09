# Loops

- Rust has three kinds of loop constructs: `loop`, `while`, and `for`.
- `loop` will execute a block of code until you explicitly tell it to stop. You can use `break` to tell the program when to stop executing the loop. `continue` tells a the program to skip over any remaining code in the iteration of a loop and fo to the next iteration. 
- If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. 
- You can specify a loop label on a loop that you can then use with those keywords that will specify that those keywords apply to the labeled loop instead of ht innermost loop. Labels follow the `'label_name` format. 
- `while` loops as long as a condition holds true. 
- The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust. 

Exercise: Rewrite this as a for loop

```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

```