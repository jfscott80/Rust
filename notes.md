### Chapter 3

#### 3.1 Variables and mutability

* Variables are by default immutable
    * `let x: 1`, `let mut x: 2`
* `const` are always immutable. Type must be annotated
  * naming convention: all uppercase with underscore

#### 3.2 Data types
Rust is a statically typed language. It must know the types of all variables at compile time.
* Scalar types
  1. integers: signed or unsigned (i or u), 
     * 8-bit `i8`$\in[{-(2^{n-1})},{(2^{n-1})-1}]=[-256,255]$, `u8`$\in [0, 255]$
     * 16-bit, 32-bit, 64-bit, 128-bit or arch
  2. floating-point: `f32` ~ single-precision, `f64` ~ double precision
  3. Booleans: `bool` ~ true, false. One byte in size.
  4. characters: `char`
* Compound types
    1. Tuple: `tup` 
        * destructuring can be done through `var name.index`
        * tuple without any values: *unit*
    2. Array: useful for allocating data on the stack rather than the heap
        * use when you know the number of elements will not need to change
        * access elements using `array[index]`

#### 3.3 Functions
Rust code uses *snake case* convention for naming function and variable names.
* Parameters
  1. When a function has parameters, you can provide it with concrete values (arguments).
  2. Function signatures *must* declare the type of each parameter.  
Ex:
```{rust}
  fn main() {
    print_labeled_measurement(5, 'h');
  }

  fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
  }
```
* Statements and Expressions
Rust is an expression-based language. Function bodies are made up of a series of statements optionally ending in an expression.  
    + **Statements** are instructions that perform an action and do not return a value.
    + **Expressions** evaluate to a resultant value. They do include ending semicolons.
* Functions with Return Values  
Return values do not have names, but the type must be declared.  
`fn five() -> i32 { 5 }`
#### 3.4 Comments
Anything on a line after `//` is commented out. Chapter 14 discusses documentation comments
#### 3.5 Control Flow
This is the typical conditional decision-tree programming.  
1. If Expressions
   * Rust uses `if`, `else if`, `else`, `while`, `for`
   * Chapter 6 discusses `match` branching.
2. Loops
   * Returning values from Loops
```{rust}
fn main() { 
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!(The result is {result}");
}
```
* Loop Labels: When you have nested loops these come in handy.  
The keywords `break` and `continue` apply to the innermost loops. To label a loop, use a single quote before the label name followed by a colon. (`'counting_up: loop { expression }`)

### Chapter 4 Ownership
#### 4.1 What is it?
* Most modern, high-level languages use garbage collection. (Python, Java, C#, Ruby, JavaScript, etc.)
* In C and C++, you need to explicitly allocate and free the memory.
* Rust uses a third approach: a system of ownership with a set of rules that the compiler checks. If the rules are violated, compiling will fail. None of the features will slow down the program while running.
* Rust is a systems programming language. This means that understanding whether a value is on the stack or the heap and how that affects 
* Ownership Rules
    * Each value in Rust has an *owner*.
    * There can be only one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
* Variable Scope follows similar rules to those in other programming languages
* The String Type
    * Unlike other data types covered in chapter 3, `String` type most likely consists of unknown size; meaning it will be stored on the heap.
    * Hard-coded string literals are convenient, but they are immutable and not every string value can be known at time of compile (like user input).
    * Strings can be created from string literals:  
`let s = String::from("hello");`

* Memory and Allocation
    * To support a mutable, growable piece of text:
    * The memory must be requested from the memory allocator at runtime.  
  `String::from()`
    * We need a way of returning the memory to the allocator when we're done with our `String`.
    * When a variable moves out of scope, Rust automatically calls a special function called `drop`, where the author of `String` can put the code to return the memory.
* Variables and Data Interacting with Move
  * Multiple variables can interact with the same data in different ways in Rust.


