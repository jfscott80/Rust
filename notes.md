## Chapter 3

### 3.1 Variables and mutability

* Variables are by default immutable
    * `let x: 1`, `let mut x: 2`
* `const` are always immutable. Type must be annotated
  * naming convention: all uppercase with underscore

### 3.2 Data types
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

### 3.3 Functions
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
### 3.4 Comments
Anything on a line after `//` is commented out. Chapter 14 discusses documentation comments
### 3.5 Control Flow
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

## Chapter 4 Ownership
### 4.1 What is it?
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
### 4.3 The Slice Type
  * *Slices* let you reference a contiguous sequence of elements in a collection (chapter 8) rather than the whole collection. A Slice is another type of reference. It does not have ownership.
```{rust}
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```
  * Slicing applies similarly to non-String types
## Chapter 5 Using Structs to Structure Related Data
### 5.1 Defining and Instantiating Structs
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
### 5.2 An Example Program Using Structs
* Using Structs as parameters in functions adds context and improves readability of the code. 
* One problem with Structs (again, similar to Python class objects): calling the normal `println!` on a Struct will cause a compile error. The macro is calling `std::fmt::Display` which is not implemented in Structs.
* Structs also do not implement the trait `Debug` by default. It can be added to a Struct explicitly by beginning with the line:  
`#[derive(Debug)]`
* More details on the `derive` attribute and others in Appendix C.
### 5.3 Method Syntax
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

---
## Chapter 6 Enums and Pattern Matching

---
### 6.1 Defining an Enum
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

### 6.2 The match Control Flow Construct
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
### 6.3 Concise Control Flow with `if let`
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
## Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
As a project grows, code should be organized by splitting into multiple modules and then multiple files. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. On a larger scale, Rust provides Workspaces (covered in chapter 14).  
* **Packages**: Cargo feature to build, test, and share crates
* **Crates**: a tree of modules that produces a library or executable
* **Modules** and **use**: control the organization, scope and privacy of paths
* **Paths**: a way of naming an item, such as struct, function, or module  

### 7.1 Packages and Crates
A **crate** is the smallest amount of code that the compiler considers at a time. 
* even a single `main.rs` file with a single function is considered a crate by the compiler
* can contain modules, and the modules may be defined in other files that get compiled with the crate

#### A crate can come in one of two forms:
**binary crates:** programs you can compile to an executable that can be run like a command line or a server
  * must have a function called `main` that defines what happens when the executable runs


**library crates:** *don't* have a `main` function and *don't* compile to an executable
  * define functionality intended to be shared with multiple projects

The **crate root** is a source file that the Rust compiler starts from and make up the root module of the crate.  

A **package** is a bundle of one or more crates that provide a set of functionality. They contain a *Cargo.toml* file that describes how to build those crates.  

**Cargo** is actually a package that contains the binary crate for the CLI tool used to build your code. The Cargo package also contain a library crate that the binary crate depends on. Other projects can depend on Cargo library crate to use the same logic the Cargo CLI tool uses. 

A package can contain as many binary crates as it wants, but at most only one library crate. It must contain at least one crate --of either type.
### 7.2 Defining Modules to Control Scope and Privacy
#### Modules Cheat Sheet
This is a quick reference on how modules, paths, the `use` keyword and the `pub` keyword work in the compiler.
* **Start form the crate root:** the compiler looks in the crate root file for code to compile
* **Declaring modules:** the compiler checks the crate root for new declared modules. Ex: `mod garden`
  * inline, within `{ }` that replace the semicolon following `mod garden`
  * in the file *src/garden.rs*
  * in the file *src/garden/mod.rs*
* **Declaring submodules:** in any file other than crate root, you can declare submodules. Ex: declaring `mod vegetables` in *src/garden.rs*
  * inline, directly following `mod vegetables` within curly brackets instead of the semicolon
  * in the file *src/garden/vegetables.rs*
  * in the file *src/garden/vegetables/mod.rs*
* **Paths to code in modules:** once a module is part of the crate, you can refer to code in the module from anywhere else in the crate. Ex: an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`
* **Private vs. Public:** code within a module is private from its parent modules by *default*. to declare it public, uae `pub mod` instead of `mod`. to make items within a public module publis as well, use `pub` before them as well
* **The `use` keyword:** within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. 
  * each instance of `crate::garden::vegetables::Asparagus` can be replaced with
  * one `use crate::garden::vegetables::Asparagus`
  * any other uses only require `Asparagus` to make use of that type

Ex: we create a binary crate named `backyard`. the crate's directory, also named `backyard`, contains these files and directories:  
```angular2html
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │     └── vegetables.rs
    ├── garden.rs
    └── main.rs     # the crate root file
```
Filename: src/main.rs
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden; //tells the compiler to include code it finds in src/garden.rs

fn main() {
  let plant = Asparagus {};
  println!("I'm growing {plant:?}!");
}
```
Filename: src/main.rs contains `pub mod vegetables;`, meaning the code in *src/garden/vegetables.rs* too: 
```rust
#[derive(Debug)]
pub struct Asparagus {}
```
#### Grouping Related Code in Modules
**Modules:**
* organize code within a crate for readability and easy reuse. 
* control the *privacy* of items
* are private by default

As an example, in path "../Rust/code_samples", terminal command `cargo new restaurant --lib` defines a new library module for organization of a restaurant's "front of house" operations. In this library crate, the module `front_of_house` contains two more modules: `hosting` and `serving`.  

Modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions. Modules allow us to group related definitions together and name why they're related. Programmers using the code can the navigate the code based on the groups rather than having to read through all the definitions. This makes it easier to recognize relevant functions *and* know where to add new functionality to the code and keep the program organized.  
#### The current crate tree module:
```angular2html
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
This tree has two *child* modules nested within the `front_of_house` module: *siblings* `hosting` and `serving`. The entire tree is rooted under the implicit module named `crate`.  
All of this organization looks very similar to file directories. Next, we look at how to find our modules.
### 7.3 Paths for Referring to an Item in the Module Tree
A path can take two forms:
  * An *absolute path* is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name and for code from the current crate, it starts with the literal `crate`.
  * A *relative path* starts from the current module and uses `self`, `super`, or an identifier in the current module.
  * both are followed by one or more identifiers separated by (`::`)

Calling the `add_to_waitlist` function is done by using its path. Next, we will define a new function in the crate root to do this.  
```rust
pub fn eat_at_restaurant() {
  //Absolute path
  crate::front_of_house::hosting::add_to_waitlist();
  //Relative path
  front_of_house::hosting::add_to_waitlist();
}
```
It's important to remember to update any *relative* path usage during re-organization.  
#### Current Compile Error:
The `hosting` module is private. The function `add_to_waitlist()` is not publicly re-exported.  

In Rust, all items are by private to parent modules by default:
* a parent module can't use the private items inside child modules
* child models **can** use the items in their ancestor modules
* this behavior acts similarly to Python class inheritance rules while adding Java private method style implementation
* Rust always provides the `pub` keyword for any items we want to make public
#### Exposing Paths with the `pub` Keyword
This compiling error is not solved by simply making `pub mod hosting`. This only lets the *module* itself be referred to by code in the ancestor module(s), not its contents. In other words, the module is a container. We make it public, *and* we also make public any items within it if we want them accessible publicly. To compile without error:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
To share library crates so other projects can use their code, your Public API is your contract with users of the crate that determines how it can be interacted with.  
More information on [Rust API Guidelines can be found here](https://rust-lang.github.io/api-guidelines/).

#### Best Practices for Packages with a Binary and a Library
In packages with both a *src/main.rs* binary crate root and a *src/lib.rs library crate root, both crates will have the package name by default.
* Typically, packages that fit this pattern will have just enough code in the binary crate to start an executable that calls code within the library crate.  
* This lets other projects benefit from most of the functionality that the package provides because the library crate's code can be shared.  

* The module tree should be defined in *src/main.rs*. Then, any public items can be used in the binary crate by starting paths with the name of the package.  
* The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: **it can only use the public API**, making you author and client of the API.
#### Starting Relative Paths with `super`
To start a relative path in the parent module instead of the current one, use `super`. It's the equivalent of `..` in filesystem path syntax.
#### Making Structs and Enums Public
Rust allows you to make a struct public while maintaining default privacy of its fields. This gives us complete control. It also makes it important to carefully consider all public and private function associations.  

In contrast, `pub enum Language {Rust, Python}` is all it takes to make an enum and all of its variants public. Enums aren't very useful if variants are private, so variants are public by default.
### 7.4 Bringing Paths Into Scope with the use Keyword
There is a way to simplify the process of calling functions, Rust provides the `use` keyword as a shortcut. Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. 
* paths brought into scope with `use` also check privacy like any other paths
* `use` only creates the shortcut for the particular scope in which the `use` occurs
#### Creating Idiomatic use Paths (functions)
```rust
//Idiomatic use path
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// bring the functions parent module into scope with use
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  //specify the parent module when calling the function
    hosting::add_to_waitlist(); 
}

//Unidiomatic, does not make it clear where add_to_waitlist is defined
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```
#### Creating Idiomatic use Paths (structs, enums, and other items)
It's idiomatic to specify the full path. To bring the standard library's `HashMap` struct into the scope of a binary crate:  
`use std::collections::HashMap;`  

There is no strong reason behind this idiom; only the convention that has emerged. The exception to this idiom occurs if we bring in two items with the same name into scope with `use` statements, because Rust doesn't allow that. To bring two `Result` types into scope that have the same name but different parent modules:
```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```
#### Providing New Names with the `as` Keyword
Providing a new local alias is an alternative way to bring two `Result` types with same name into scope.
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```
#### Re-exporting Names with `pub use`
When we bring a name into scope with the `use` keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`. This technique is called re-exporting.  

**Re-exporting:** bringing an item into scope while also making that item available for others to bring into their scope.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Before making this change, external code would have to call the `add_to_waitlist()` function by using the path  
`restaurant::front_of_house::hosting::add_to_waitlist()`  
which also would have required front_of_house to marked public. By re-exporting, external code can use the path  
`restaurant::hosting::add_to_waitlist()`  

This is useful when the internal structure of your code is different from another programmers expectations of your domain. We can write our code with one structure but expose a different structure, making the library well organized for programmers working on the library and programmers calling the library.
#### Using External Packages
**Recall:** in chapter 2 with the guessing game project, we added a dependency to access the `rand` package
```
Filname: Cargo.toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```
Adding the dependency tells Cargo to download the `rand` package and make it available to our project. Then, to bring `rand` definitions (in this case, the `Rng` trait) into the scope of the package,  
`use rand::Rng;` gives access to the functions within the trait until `Rng` goes out of scope.  

This is the procedure for pulling any of the many packages made available by the Rust community at [crates.io](https://crates.io/).  
The [standard `std` library](https://doc.rust-lang.org/std/) is also a crate external to our package. However, because it is shipped with the Rust language, we don't need to take the first step of adding it to the Cargo.toml dependencies. The `use` keyword with the absolute path is all that is required to bring any items from the standard library into scope as previously demonstrated with `use std::collections::HashMap;`.  
#### Using Nested Paths to Clean Up Large `use` Lists
Using multiple items defined within the same crate or module individually has the potential for wasting vertical space in our code, as well as creating unnecessary, repetitive code. Rust allows for nested paths to attempt to adhere to DRY best practices.
```
Filename: src/main.rs
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

// --snip--
use std::{cmp::Ordering, io};
// --snip--

Filename: src/lib.rs
use std::io;
use std::io::Write;

use std::io::{self, Write};
```
Both methods achieve the same end, one decreases repeating ourselves needlessly.
#### The Glob Operator
To bring *all* public items defined in a path into scope, use the `*` glob operator:  
`use std::collections::*;`  
This should be used carefully, as it may make things less precise and harder to recognize what and where names are defined / in scope. The most likely case of applying the glob operator is during testing (covered in chapter 11) and sometimes as part of the [prelude pattern](https://doc.rust-lang.org/std/prelude/index.html#other-preludes).
### 7.5 Separating Modules into Different Files
To optimize organization, it may become necessary to separate large modules into their own files. For example, let's move the front of house module to its own file named *src/front_of_house.rs*
```rust
//Filename: src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
//Filename: src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
The compiler knows to look for the second file because of the module declaration in the crate root.

Rust requires only *one* `mod` declaration in the module tree. Now to look at what happens if we decide to extract the hosting module to its own file. The process is different; hosting is a child of front of house, not of the root module.
```rust
//Filename: src/front_of_house.rs
pub mod hosting;

//new file. name: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```
Cargo looks at the `mod` declarations and tells the compiler to use naming conventions and the module tree structure to find the relevant module.  
#### Alternate File Paths
Besides the idiomatic file paths that the Rust compiler uses, Rust also supports an older style of file path.  

For front of house declared in the crate root:
* src/front_of_house.rs
* src/front_of_house/mod.rs  

For hosting as part of front of house
* src/front_of_house/hosting.rs
* src/front_of_house/hosting/mod.rs

Both styles are supported. Compile errors will occur if both styles are used for the same module and using this older style may result with many files named *mod.rs*, causing confusion. There appears to be no advantage to the older style.  
### Summary
* Packages can be split into multiple crates
* Crates can be split into modules
* Items defined in one module can be referred to in other modules
* Use absolute or relative paths to refer to modules
* Bring paths into scope with `use` to create shortcuts
* Module code is private by default

---
## Chapter 8: Common Collections
Rust's `std` library includes **collections**, a very useful, unique data structure type. Most data types represent one specific value, but collections can contain multiple values.  

Unlike built-in tuples and arrays, the data that the collections point to are stored on the heap. Data stored on the heap follows its own rules. 
#### Recall:
The amount of data does not need to be known at compile time and can grow or shrink as the program runs.

Each kind of collection has different capabilities and costs; choosing an appropriate on for a given situation is an acquired skill. This chapter discusses the three most commonly used:
* **vector:** store a variable number of values next to each other in memory
* **string:** a collection of characters
* **hash map:** associate a value with a specific key; a particular implementation of the more general data structure **map**
### 8.1 Storing Lists of Values with Vectors
Vectors, or `Vec<T>`, can only store values of the same type, but they are useful when you have a list of items. 
#### Creating a New Vector
`let v: Vec<i32> = Vec::new();`  
In this example, we added the type annotation. Because we aren't inserting any values into this vector, Rust doesn't know what kind of elements we intend to store. This is an important point. Vectors are implemented using generics which are covered in chapter 10.  
The `Vec<T>` type provided by the `std` library can hold any type; the type can be specified as shown.  

In most cases, the vector will be created with initial values and Rust will infer the type of value to store. Rust provides the `vec!` macro, which will create a new vector that holds the values given to it.  
`let v = vec![1, 2, 3];`
#### Updating a Vector
As with any variable, Rust requires that mutability must be defined explicitly. To create a new vector and then add elements to it, use `push`:
```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```
#### Reading Elements of Vectors
There are two ways to reference a value stored in a vector: indexing and the `get` method. The following example shows both methods with types of returned values annotated for extra clarity.
```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```
Rust provides these two ways to reference an element to let you choose how the programs behaves when you try to use an index outside the existing element range. What happens in each case?
* index
  * causes the program to *panic*
  * use this method when you want the program to crash during an attempt to access an element past the end of the vector
* `get`
  * returns `None` without panicking
  * use if accessing an element beyond the range of the vector may happen occasionally under normal circumstances
  * the code should also provide some logic to handle having either `Some(&element)` or `None`
  * these circumstances may be presented when handling user inputs

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid.
#### Recall:
The rule states you can't have mutable and immutable references in the same scope. 
```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                     ------- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` (bin "collections") due to 1 previous error
```
Why should a reference to the first element care about the changes at the end of the vector?  
The error is due to the way that vectors work:
* values are stored next to each other in memory
* adding a new element onto the end of the vector might require allocating new memory
* if there isn't enough room, the vector is copied and stored into a new space
* the reference to the first element would now be pointing to deallocated memory
* all of this is prevented by the borrowing rules

For more details of the `Vec<T>` type, look into the "dark side" of Rust: [The Rustonomicon](https://doc.rust-lang.org/book/ch08-01-vectors.html).
#### Iterating Over the Values in a Vector
Using a `for` loop to get immutable references to each element
```
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```
Using a `for` loop to get mutable references to each element
```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```
**Note:** the use of the `*` dereference operator to get to the value in `i` before using the `+=` operator. This is covered more in chapter 15.  

Iterating over a vector, whether mutably or immutably, is safe because of the borrow checker's rules. A compile error **will** result if we attempt to insert or remove items in the `for` loop bodies. The reference to the vector that the `for` loop holds prevents simultaneous modification of the whole vector.
#### Using an Enum to Store Multiple Types
Vectors can only store values of the same value type. Enums, with their variants, are not limited in this manner. All the variants in an enum are defined under the same enum type. So, create a vector that holds that enum, and you have carefully sidestepped any compile errors.
```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. It is necessary to be explicit about what types are allowed in the vector. Using an enum along with a `match` expression ensures that every possible case is handled at compile time.
#### Dropping a Vector Drops its Elements
Like any other `struct`, a vector is freed when it goes out of scope.  
When the vector gets dropped:
* all of its contents are also dropped
* the integers it holds will be cleaned up
* the borrow checker ensures that any references to contents of the vector are only used while the vector is valid
### 8.2 Storing UTF-8 Encoded Text with Strings
New Rust users run into a combination of three common difficulties with strings:
1. Rust is designed to expose errors
2. strings are more complicated as a data structure type than they appear
3. UTF-8
#### What is a String?
* In the core language, Rust has only one string type: *string slice* `str` and the borrowed form `&str`.
* String literals are string slices stored in the program's binary.
* The `String` type provided by the `std` library is a growable, mutable, owned, UTF-8 encoded string type.
* Both types are UTF-8 encoded.
#### Creating a New String
Many of the same operations available with `Vec<T>` are available to `String` because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
```{rust}
// create a new instance
let mut s = String::new();

// convert some initial data
let data = "initial contents";
let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
// another way:
let s = String::from("initial contents");
```
Because strings are used for so many things, we can use many different generic APIs. Strings are UTF-8 encoded; they may include any properly encoded data in them.
```{rust}
// Storing greetings in different languages in strings:
let hello = String::from("السلام عليكم");  // Arabic (hello)
let hello = String::from("Dobrý den"); // Czech (good morning)
let hello = String::from("Hello"); // English
let hello = String::from("שלום"); // Hebrew (peace)
let hello = String::from("नमस्ते"); // Hindi (hello)
let hello = String::from("こんにちは"); // Japanese (hello)
let hello = String::from("안녕하세요"); // Korean (hello)
let hello = String::from("你好"); // Chinese simplified (are you ok)
let hello = String::from("Olá"); // Portugese (hello)
let hello = String::from("Здравствуйте"); // Russian (hello)
let hello = String::from("Hola"); // Spanish (hello)
```
#### Updating a String
Like a vector type, the String type can grow in size and its contents change if you push more data into it. In addition, the `+` operator and the `format!` macro can be used to concatenate `String` values.  

**Appending** to a String with `push_str`
```{rust}
// append a string slice
let mut s = String::from("foo");
s.push_str("bar");
```
After these two lines, `s` will contain 'foobar'. The `push_str` method takes a string slice because we don't necessarily want to transfer ownership of the parameter.
```{rust}
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!(s2 is {s2}");
```
Because ownership is not transferred, `s2` is still available in that last line.  

**Appending** to a String with `push`  
The `push` method acts on a single character as a parameter and adds it to the `String`.
#### Concatenation with the `+` operator
Another effective method is the `+` operator, which acts as the `add` method does.  
Signature of the `add` method:
```{rust}
fn add(self, s: &str) -> String {}
```
The `std` library uses the `add` method defined using generics and associated types. The above code has substituted in the concrete type, which occurs when we call this method with `String` values.  
The example provided by the (incomplete) signature along with the next example demonstrate clues to understanding the tricky behavior we encounter with the `+` operator.
```{rust}
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
Use the reference `&s2` in the call to `add` so that the compiler can *coerce* the `&String` argument into a `&str`. **Deref coercion** turns `&s2` into `&s2[..]`, a concept discussed in more detail in chapter 15.

The important points to pay attention to here are:
* we can't `add` to `String` values together
* we can add one `String` to a `&String` or `&str`
* `add` does not take ownership of the `s` parameter
* `add` takes ownership of `self`
* the note regarding the overall complexity of the `String` type was an understatement.



### 8.3 Storing Keys with Associated Values in Hash Maps


---
## Chapter 9: Error Handling

### 9.1 Unrecoverable Errors with panic!

### 9.2 Recoverable Errors with Result

### 9.3 To panic! or Not to panic!


---
## Chapter 10: Generic Types, Traits, and Lifetimes

### 10.1 Generic Data Types

### 10.2 Traits: Defining Shared Behaviors

### 10.3 Validating References with Lifetimes


---
## Chapter 11: Writing Automated Tests

### 11.1 How to Write Tests

### 11.2 Controlling How Tests Are Run

### 11.3 Test Organization