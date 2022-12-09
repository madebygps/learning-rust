# Functions

- Rust uses snake case as the conventional style for function and variable names.
- Rust doesn't care where you define your functions, only that they're defined somewhere. 
- In function signatures, you must declare the type of each parameter. 
- Rust is an expression-based language. 
- Function bodies are made up of a series of statements optionally ending in an expression. 
    - Statements are instructions that perform some action and do not return a value.
    - Expressions evaluate to a resulting value. 
    - Expressions can be part of statements. 
    - Expressions do not include semicolons, if you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 
    - Return values do not get names but types are declared after an arrow (->)
    - The return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    - You can return early with the `return` keyword.  