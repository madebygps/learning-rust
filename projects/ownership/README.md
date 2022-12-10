# Ownership

Ownership is a set of rules that governs how Rust manages memory and enables Rust to make memory safety guarantees without needing a garbage collector. 

## The Stack and the Heap

The stack and the heap are parts of memory that are available to your code to use at runtime. 

### The Stack

- Stores values in the order it gets them and removes them in the opposite order (last in, first out).
- Adding data is called pushing onto the stack, and removing data is called popping off the stack. 
- All data stored on the stack must have a known, fixed size. 
- Pushing to the stack is faster than allocating on the h eap because the OS never has to search for a place to store new data, it will always be on the top. 

### The heap 

- Is less organized, when you put data on the heap, you request a certain amount of space, the OS finds an empty spot in the heap that is big enough, marks it as being in use, and returns a point which is the address of that location. 
- This process is called allocating on the heap (allocating).
- The point is a known, fixed size, so it can be stored on the stack, and you must follow it to get the actual data. 
- Allocating space n the heap requires more work, because the OS must first find a big enough space to hold the data and  the perform bookkeeping to prepare for the next allocation. 
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 

Ownership addresses keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap. 

## Rules

- Each value has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped. 