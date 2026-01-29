# Packages, Crate, Modules

## Crates
A crate is the smallest amount of code that the Rust compiler considers at a time. Even a single source code file is a crate.

A crate can come in one of two forms: **a binary crate** or a **library crate**. 
Binary crates are programs you can compile to an executable, such as a command line program or a server. Every binary crate must have a function called main that defines what happens when the executable runs.

Library crates do not have a main function, and do not compile to an executable. Instead, library crates define functionality intended to be shared with multiple projects. Most of the time when Rustaceans say “crate,” they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library.”

The **crate root** is a source file that the Rust compiler starts from and makes up the root module of your crate


## Package
A package is a bundle of one or more crates which provide a set of functionality. 

A package contains a **Cargo.toml** file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command line tool you use to build your code. 

The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.

A package can contain as many binary crates as you like, but at most only one library crate. 

A package must contain at least one crate, whether that’s a library or binary crate.

Cargo follows a convention that src/main.rs is the crate root of a binary crate. 

If the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.

 If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library
 
> A package can have multiple binary crates by placing files in the src/bin directory: Each file will be a separate binary crate.

## Modules

[cheatsheet to modules](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)

Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`

 To make items within a module public, use `pub` before their declarations.
 
 The `use` keyword creates shortcuts to items to reduce repetition of long paths within a scope.
 > In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;`, and then you only need write `Asparagus` to use the type in the scope.
