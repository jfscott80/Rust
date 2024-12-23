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
`let x = 5; // bind the value 5 to x`  
`let y = x; // make a copy of the value in x and bind it into y`
We now have two variables, `x` and `y`, and both equal 5. Simple, normal stuff: integers are a known, fixed size. These two `5` values are pushed onto the stack.  
`let s1 = String::from("hello"); // [5 bytes] len 5: how much memory being used. capacity 5: total memory received from the allocator`  
`let s2 = s1; // s1 is no longer valid. there is no memory to free up when s1 moves out of scope. `  
The `s1` value "hello" has **move**d to `s2`
* Variables and Data Interacting with Clone
  * If you do want a *deep* copy of the heap data variable, and not just the stack data, we `clone`  
```{Rust}
let s1 = String::from("hello");
let s2 = s1.clone(); // clone() is expensive
println!("s1 = {s1}, s2 = {s2}");
```
* Stack-Only Data: Copy
  * integers, floats, chars, Booleans, tuples containing only data types that can be copied
* Ownership and Functions
  * Passing a value to a function behaves similarly to assigning a value to a variable. Passing a variable to a function will `move` or `copy`, just as assignment does.
* Return Values and Scope
  * Returning values can also transfer ownership. Assigning a value to another variable moves it; when a variable that contains data stored on the heap goes out of scope, it is dropped unless ownership has been moved to another variable.
  * This is complicated and tedious - `move` a value, use it for some purpose, and `move` it back. 
  * It's easier to have a function use a value without taking ownership and returning ownership.
  * One method is to have the function return a tuple. This allows us to use the value in a function, return the value and any new data created by the function for use later. Essentially returning (original value, new data). This is also tedious and doesn't really make things easier.
### 4.2 References and Borrowing (`&`)
  * Using this tool, we can use a value without changing ownership.
  * Creating a reference is called borrowing.
  * Modifying a referenced variable depends on whether the variable is mutable or immutable. Remember that variables are immutable by default.
* Mutable References
  * Only mutable variables can have mutable references
  * Only one mutable reference can exist. Once created, no other references to the original variable can be created.
    * Multiple mutable references can be created, just not *simultaneously* 
  * Making immutable references to variables (mutable or not) does not have a restriction on the number that can be created.
  * Rust can prevent 'data races'.
  * A *data race* is similar to a race condition and happens when:
    * Two or more pointers access the same data at the same time
    * At least one of the pointers is being used to write the data
    * There's no mechanism being used to synchronize access to the data
* Dangling References
  * A *dangling pointer* refers to a pointer that references a location in memory that may have been given to someone else. Rust prevents this.
#### 4.3 The Slice Type
  * *Slices* let you reference a contiguous sequence of elements in a collection (chapter 8) rather than the whole collection. A Slice is another type of reference. It does not have ownership.
```{rust}
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```
  * Slicing applies similarly to non-String types
### Chapter 5 Using Structs to Structure Related Data
#### 5.1 Defining and Instantiating Structs
* The *Struct* data type behaves like an OOP class object
Example of a structure:  
```{rust}
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64 // examples show a comma here as well
};
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```
* The **entire** instance must be mutable or immutable. 
* Creating a build {structure name} function is common practice, and Rust allows for field inits when the parameter names match the field value.
* Creating Instances from Other Instances with Struct Update Syntax
  * This means calling the other instance's field value with dot notation when defining the new instance.
  * Rust also provides for an even shorter solution to this:  
```{rust}
fn main() {
  //--user1 instance--//
  let user2 = User {
    email: String::from("user2@email.com")
    ..user1
    }
```
* Doing these structure updates can be tricky. In the above example, the values for the unnamed fields is **moved** from user1 to user2. Since this example has moved the value for username, user1 is no longer available.
* Using Tuple Structs Without Named Fields to Create Different Types
  * This is a special tuple type that only defines the data types.  
`struct Color(i32, i32, i32)`
* Unit-Like Structs Without Any Fields
  * This section describes a struct object with no fields and no data type defined. A further explanation is to be explained in Chapter 10; it appears to be useful in validating references and possibly for testing.
  * The ability to store references in Structs requires the use of *lifetimes* (Chapter 10)
#### 5.2 An Example Program Using Structs
[Hover for code path](C:\Users\johnf\Rust\Rust\structs\src\main.rs)
* Using Structs as parameters in functions adds context and improves readability of the code. 
* One problem with Structs (again, similar to Python class objects): calling the normal `println!` on a Struct will cause a compile error. The macro is calling `std::fmt::Display` which is not implemented in Structs.
* Structs also do not implement the trait `Debug` by default. It can be added to a Struct explicitly by beginning with the line:  
`#[derive(Debug)]`
* More details on the `derive` attribute and others in Appendix C.
#### 5.3 Method Syntax
**Defining Methods**
* Similar to Python, methods behave like functions defined within a struct, enum or a trait object.  
```{rust}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
* The main reason for using methods instead of functions is for organization. 
* Another common use of methods is to implement one that shares a name with the Struct's field name and set it to return the field value  -- creating a *getter*.  

**Methods with More Parameters**

### Chapter 6 Enums and Pattern Matching

---
#### 6.1 Defining an Enum
Enums give you a way of saying a value is one of a set of possible values. One case where this may be useful is when working with IP addresses. We would need a way to account for v.4 and a v.6 version --but only one or the other at any given time.  
`enum IpAddrKind {V4, V6,} //again with this extra comma`  
**Enum Values**  
* We can create an instance of each like this:  
`let four = IpAddrKind::V4;`  
`let six = IpAddrKind::V6;`  
* So variants of an `enum` are namespaced under its identifier, which is useful because both instances are of the same type: `IpAddrKind`; allowing us to define a function that takes any instance of tha type.  
`fn route(ip_kind: IpAddrKind) {}`  
`route(IpAddrKind::V4);`  
* We can also update our `enum` to store the actual IP address *data*:  
```{rust}
enum IpAddrKind {
  V4(String),
  V6(String),
}
```
This demonstrates another aspect of the `enum`: the name of each variant that we define also becomes a function that constructs an instance of the enum.  
* Enums are not limited to defining each variant with the same data type or number of them.
```{rust}
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
```
* The IP address example is common enough that the standard library has it built in. Only, They created a Struct for both versions and then embedded them into an Enum. Any kind of data can be stored in an enum, including other enums.  
Ex:  
```{rust}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
This enum has four variants with different types:
  * **Quit** has no data associated with it
  * **Move** has named fields
  * **Write** includes a String
  * **ChangeColor** has three i32 values  

Each variant behaves like a Struct, except it would take four different Structs. This method allows us to write a function that takes any of these variants because the variants of the enum are all grouped together as the **Message** type.  
We can also use `impl` to create methods for an enum.  

**The Option Enum and its Advantages Over Null Values**  
* `Option` is a special type of enum to handle the case where the value could be something *or* it could be nothing. One common reason to use these is to prevent bugs.  
* Rust doesn't use *Null*. Compile errors cannot result in Rust due to a Null value being called when a not-Null value is expected. To express the concept of an invalid or absent value, the enum `Option<T>` was created in the standard library.  
```{rust}
enum Option<T> {
  None,
  Some<T>,
}
```
Chapter 10 will cover generic type parameters like `<T>`. For now, knowing that `T` can hold one piece of any kind of data. In general, in order to use an `Option<T>` value, you want to have code that can handle each variant. Code that will run only if `Some<T>` value exists, and code that will run only if `None` is found. Next, we will look at the `match` expression.

#### 6.2 The match Control Flow Construct
**Match** compares a value against a series of patterns and then executes code that matches the indicated pattern.  
Patterns can be made up of literal values, variable names, wildcards, and many other things. Covers will be covered more in Chapter 18.  
```{rust}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
Create an instance of **Coin** using the enum, and the function returns a value that matches. It is simple to add more to the process. Adding  
`Coin::Penny => { println!( "Lucky penny!" ); 1 }`  
will print the indicated text *and* return the value.  
**Patterns That Bind Values**  
```{rust}
#[derive(Debug)]
// so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```
The pattern of combining a match against an enum, bind a variable to the data inside, and then execute code based on it.  
**Matches Are Exhaustive**  
Another aspect of `match`: **all** cases must be covered in order for the code to be valid. Rust takes measures to identify missing cases during compile.  
**Catch-all Patterns and the _ Placeholder**  
Using enums, we can take special actions for a few particular values, but for all other values take one default action.  
```{rust}
    let dice_roll = 9; // hardcoded instead of random for explanatory reason
    match dice_roll {
        3 => add_fancy_hat(), // not 3, pass
        7 => remove_fancy_hat(), // not 7, pass
        other => move_player(other), // calls move_player(other)
    } // 'other' acts as catch-all
    
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
---
// another way to do this would be to replace the 'other' arm of match with:
        _ => reroll()
        //--snip--and replace move_player with
     fn reroll() {}
// the _ placeholder replaces the catch-all other. still meets exhaustiveness requirement.
// explicitly ignoring any value that reaches the last _ arm
---
// if you want nothing to happen --no move_player, no reroll-- we can use
        //--snip--
        _ => ()
// match understands that we don't want to use any value except the two explicit values
// in this implementation, match is told to run no further code.
```
#### 6.3 Concise Control Flow with `if let`
This syntax is a less verbose way to handle values that match one pattern while ignoring the rest. A comparison of `match` and `if let` to handle the same pattern.  
```{rust}
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    let config_max = Some(3u8);
    if let Some(max) = config_max { // note the single '='
        println!("The maximum is configured to be {max}");
    }
```
**Match** enforces exhaustive checking, **if let** trades that for less code, less indentation, and less boilerplate code. Use cautiously. `if let` allows for the use of `else`, but this seems to lose any advantage of this method.  

---
### Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
As a project grows, code should be organized by splitting into multiple modules and then multiple files. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. On a larger scale, Rust provides Workspaces (covered in chapter 14).  
* **Packages**: Cargo feature to build, test, and share crates
* **Crates**: a tree of modules that produces a library or executable
* **Modules** and **use**: control the organization, scope and privacy of paths
* **Paths**: a way of naming an item, such as struct, function, or module  

#### 7.1 Packages and Crates

#### 7.2 Defining Modules to Control Scope and Privacy

#### 7.3 Paths for Referring to an Item in the Module Tree

#### 7.4 Bringing Paths Into Scope with the use Keyword


#### 7.5 Separating Modules into Different Files

### Chapter 8: Common Collections

#### 8.1 Storing Lists of Values with Vectors

#### 8.2 Storing UTF-8 Encoded Text with Strings

#### 8.3 Storing Keys with Associated Values in Hash Maps

### Chapter 9: Error Handling

#### 9.1 Unrecoverable Errors with panic!

#### 9.2 Recoverable Errors with Result

#### 9.3 To panic! or Not to panic!

### Chapter 10: Generic Types, Traits, and Lifetimes

#### 10.1 Generic Data Types

#### 10.2 Traits: Defining Shared Behaviors

#### 10.3 Validating References with Lifetimes

### Chapter 11: Writing Automated Tests

#### 11.1 How to Write Tests

#### 11.2 Controlling How Tests Are Run

#### 11.3 Test Organization