// errorsn.rs
// This is a bigger error exercise than the previous ones!
// You can do it! :)
//
// Edit the `read_and_validate` function ONLY. Don't create any Errors
// that do not already exist.
//
// So many things could go wrong!
//
// - Reading from stdin could produce an io::Error
// - Parsing the input could produce a num::ParseIntError
// - Validating the input could produce a CreationError (defined below)
//
// How can we lump these errors into one general error? That is, what
// type goes where the question marks are, and how do we return
// that type from the body of read_and_validate?
//
// Execute `rustlings hint errorsn` for hints :)

use std::error;
use std::fmt;
use std::io;

// this is the idiomatic way i think, although Boxing loses the type informations
// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut line = String::new();
    b.read_line(&mut line)?;
    let num = line.trim().parse::<i64>()?;
    let answer = PositiveNonzeroInteger::new(num)?;
    Ok(answer)
}

// Can also convert all errors to strings, which would allow (non idiomatic) and_then calls

// Can also define your own error enum  as below, to support all the error types that could occur. This is
// better as it retains the type information, and allows and_then type calls, although it is a lot of
// code. probably worth it if writing a library but not otherwise.

// This blog post is excellent at describing all the thorny details
// https://blog.burntsushi.net/rust-error-handling/#working-with-multiple-error-types

#[derive(Debug)]
enum AnyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Creation(CreationError)
}

impl fmt::Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            AnyError::Io(ref err) => write!(f, "IO error: {}", err),
            AnyError::Parse(ref err) => write!(f, "Parse error: {}", err),
            AnyError::Creation(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for AnyError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            AnyError::Io(ref err) => err.description(),
            AnyError::Creation(ref err) => "",
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            AnyError::Parse(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            AnyError::Io(ref err) => Some(err),
            AnyError::Parse(ref err) => Some(err),
            AnyError::Creation(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for AnyError {
    fn from(err: io::Error) -> AnyError {
        AnyError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AnyError {
    fn from(err: std::num::ParseIntError) -> AnyError {
        AnyError::Parse(err)
    }
}

impl From<CreationError> for AnyError {
    fn from(err: CreationError) -> AnyError {
        AnyError::Creation(err)
    }
}

// fn read_and_validate(b: &mut dyn io::BufRead) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
//     let mut line = String::new();
//     let bytesCountResult = b.read_line(&mut line).map_err(AnyError::Io);
//     let lineResult = bytesCountResult.map(|_|line);
//     let numResult = lineResult.and_then(|line|line.trim().parse::<i64>().map_err(AnyError::Parse));
//     let answer = numResult.and_then(|num|PositiveNonzeroInteger::new(num).map_err(AnyError::Creation));
//     answer.map_err(|e|From::from(e))
    
// }

//
// Nothing below this needs to be modified
//

// This is a test helper function that turns a &str into a BufReader.
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test]
fn test_success() {
    let x = test_with_str("42\n");
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    assert!(x.is_err());
}

#[test]
fn test_non_positive() {
    let x = test_with_str("-40\n");
    assert!(x.is_err());
}

#[test]
fn test_ioerror() {
    struct Broken;
    impl io::Read for Broken {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
        }
    }
    let mut b = io::BufReader::new(Broken);
    assert!(read_and_validate(&mut b).is_err());
    assert_eq!("uh-oh!", read_and_validate(&mut b).unwrap_err().to_string());
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "Number is negative",
            CreationError::Zero => "Number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
