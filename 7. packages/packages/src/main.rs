// A package can contain multiple binary crates and optionally one library crate.
/*
    Packages: A Cargo feature that lets you build, test, and share crates
    Crates: A tree of modules that produces a library or executable
    Modules and use: Let you control the organization, scope, and privacy of paths
    Paths: A way of naming an item, such as a struct, function, or module
*/
// A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

/*
* A crate can be a binary crate or a library crate.
* Library crates don’t have a main function, and they don’t compile to an executable.
* The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.
* A package can contain at most one library crate. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).
*/

/*
 * Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate).
 *
 * Declaring modules: In the crate root file, you can declare a new module named, say, “garden”, with mod garden;. The compiler will look for the code inside the module in these places:
 * Inline, directly following mod garden, within curly brackets instead of the semicolon
 * In the file src/garden.rs
 * In the file src/garden/mod.rs
 *
 * Declaring submodules: In any file other than the crate root that’s being compiled as part of the crate (for example, src/garden.rs), you can declare submodules (for example, mod vegetables;). The compiler will look for the code inside submodules in these places within a directory named for the parent module:
 * Inline, directly following mod vegetables, within curly brackets instead of the semicolon
 * In the file src/garden/vegetables.rs
 * In the file src/garden/vegetables/mod.rs
 *
 *
 * */
 
use crate::garden::vegetables::Asparagus; // it's like bringing this type in the namespace of this file
pub mod garden; // tells the compiler to include the code from garden module
// The module at the crate's root is called crate, and every other module is relative to this root's module (create), therefore create::garden::vegetables::Asparagus
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
