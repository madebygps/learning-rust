# Data Types

- Rust is a statically typed language, it must know the types of all variables at compile time. 

## Scalar types

- Represents a single value

### Integers

- An integer is a number without a fractional component. 
- Integer types default to `i32`
- The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection.
- If you're unsure of which type of integer to use, Rust's defaults are generally a good place to start.
- If an overflow occurs when compiling in release mode with the `--release` flag, Rust performs two's complement wrapping. Values greater than the maximum value the type can hold wrap around to the minimum of the values the type can hold. 

### Floating-Point types

- Numbers with decimal points. 
- There are two types `f32`   and `f64` (default) which are 32 bits and 64 bits in size, respectively.
- All are signed. 

### Numeric Operations

- Rust supports the basic mathematical operations (+, -, *, /).
    ```rs
    fn main() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0

        // remainder
        let remainder = 43 % 5;
    }
    ```
- Integer division rounds down to the nearest integer. 

### The Boolean Type

- `bool` can be `true` or `false` are one byte in size. 

### The Character Type

- `char` literals are specified with `'` single quotes as opposed to string literals which use `"` double quotes. 

## Compound types

- Can group multiple values into one type.
- Rust has two primitive compound types: `tuples` and `arrays`.

### The Tuple Type

- A tuple has a fixed length and once it's declared it cannot grow or shrink.
- Each position in the tuple has a type and then don't have to be the same.
- You can access an individual value with pattern matching or by using a `.` period.
    ```rs 
    fn main() {
        let tup = (500, 6.4, 1);
        // Pattern matching to destructure
        let (x, y, z) = tup;
        println!("The value of y is: {y}");
    }
    ```
    ```rs
    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);
        // Access a value with a .
        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }
    ```
- A tuple without any values is called a unit. It's written `()` and represents an empty value or an empty return type. Expressions implicitly return the unit value if they don't return any other value. 

### The Array Type

- Every element in a array must have the same type. 
- Arrays have a fixed length.
- Values in an array are written as comma-separated list inside square brackets.
    ```rs
        fn main() {
            let a = [1, 2, 3, 4, 5];
        }
    ```
- Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements. 
- You can access elements like this:
    ```rs
        fn main() {
            let a = [1, 2, 3, 4, 5];

            let first = a[0];
            let second = a[1];
        }
    ```` 
- Rust check that the index you've specified is less than the array length, this kind of check is not done in many low-level languages. 
- If you try to access an index that is past the end of the array, Rust will panic, exiting immediately instead of allowing memory access.  
