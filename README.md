## Rust

* Statically typed language

### Start
* Files are ending with extension ```.rs```
* Compile class by ```rustc``` command
* ```main()``` function is entry of each program
    * can have arguments
* lines/execution parts are ending with parenthesis ```;```
* functions with ```!``` are __macros__

### Cargo
* Helps to create new project
* Create ```src``` and ```toml``` files to manage project
* ```cargo build```
  * It's default build with ```target/debug``` build
* ```cargo run``` - compile and run program
* ```cargo check``` - compile and check errors
* ```cargo build --released``` - compile optimized version
* ```cargo update``` - updating dependencies

### Dependencies
* ```https://crates.io/```
* ```cargo doc --open``` - build docs dependencies of your program

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


### Cargo.lock file
```cargo update```: ignore lock file and trying to find out the newest version of packages

### Variables 
> https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

### Main concepts
* variables by defaults are **immutable!**
* but you can make variables mutable if you want! `mut`
  * keyword `mut` don't allow to assign another type value to variable
* constants are immutable also `const`
* Shadowing variables - declares with the same name
  * Allow to redeclare with other value or type

### Data types [#doc](https://doc.rust-lang.org/book/ch03-02-data-types.html)

Data types subsets:
* Scalar - represents a single value
  * integers 
      ```
      example: u32 or i32
    
      u -> unsigned (not allows negative values)
      i -> signed (allow negative values)
    
      i8 -> from -(2**7) to 2**7-1 (from -128 to 128)
      but:
      u8 -> from 0 to 2**8-1 (from 0 to 255)
      ``` 
  * floating-points,
  * numbers
  * Boolean
  * characters
  
* Compound



