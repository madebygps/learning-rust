# Variables and Mutability

- The Rust compiler guarantees that when you state a value won't change, it really won't change, so you don't have to keep track yourself. 
- Adding `mut` makes a variable mutable and conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable's value. 

## Constants

- You aren't allowed to use `mut` with constants. They are always immutable.
- The `type` myst be annotated. 
- They can be declared in any scope, including the global scope. `let` can only be in the function scope. 
- Constants may be set only to a constant expression, no the result of a value that could only be computed at runtime:
    ```rs
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    ```
- Use all uppercase with underscores between words.

## Shadowing

- You can declare a new variable with the same name as a previous variable. The first variable is shadowed by the second until either it itself is shadowed or the scope ends.
- We'll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.
- Since we are using the `let` keyword, we can also change the type of the variable. 