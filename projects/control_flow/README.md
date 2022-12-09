# Control Flow

- Blocks of code associated with the conditions in `if` expressions are sometimes called arms. Just like the arms in `match` expressions.
- You must be explicit and always provide `if` with a Boolean as its condition. Rust will not automatically try to convert non-Boolean types to a Boolean.  
- When using `else if` Rust only executes the block for the first true condition.
- Using multiple `else if` expressions can clutter your code, `match` would be a better fit.