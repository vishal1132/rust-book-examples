use core::panic;
use std::{fs::File, io::{Error, ErrorKind, self, Read}};

// 2 major types of errors- recoverable and unrecoverable
/*
By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.
However, this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up. 
Memory that the program was using will then need to be cleaned up by the operating system. 
If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
*/
fn main() {
    // RUST_BACKTRACE=1 to see the complete backtrace of the panic.
    // let v = vec![1, 2, 3];
    // v[99];

    let f:Result<File, Error>=File::open("abcd.txt"); // the type of f is enum Result<T,E>{Ok(T),Err(E)}
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    let f = File::open("hello.txt").unwrap(); // unwrap method is implemented on Result<T,E> and when the variant is Ok, it returns the type T, when the variant is Error, it panics.
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // like unwrap but let's us define our intent.
}

fn read_username_from_file() -> Result<String, io::Error> {
    // try to read a file and put it's contents into a String, if not return error, rather than panicking.
    // First thing the return type is Result<T,E>, where T is String, and E is io::Error
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // we don't need to explicitly use return here, since this is the last expression of the file.
        Err(e) => Err(e),
    }
}


fn read_username_from_file_shorter() -> Result<String, io::Error> {
    // The ? placed after a Result value is defined to work in almost the same way as the match expression, if ok continue, if err return it.
    /*
        There is a difference between what the match expression above does and what the ? operator does:
        error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. 
        When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. 
        This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. 
        As long as there’s an impl From<OtherError> for ReturnedError to define the conversion in the trait’s from function, the ? operator takes care of calling the from function automatically.
    */
    let mut f = File::open("hello.txt")?; 
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter_chaining_methods() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    use std::fs;
    fs::read_to_string("hello.txt") // reading files to strings is very common that std::fs provides a method directly.
}

/*
    The ? operator can only be used in functions whose return type is compatible with the value the ? is used on
    We’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.

    Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.
    The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.
*/