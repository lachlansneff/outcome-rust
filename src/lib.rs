//! # The outcome crate
//! Type `Outcome` represents a success or failure: Every `Outcome` is either `Success` or `Failure`
//! 
//! ```
//! use outcome::*;
//! 
//! fn do_something() -> Outcome {
//!     Success
//! }
//! 
//! // The return value is an outcome
//! let result = do_something();
//! 
//! // Pattern Match
//! match result {
//!     Success => println!("Well done!"),
//!     Failure => println!("Oh well :("),
//! }
//! ```
//! 
//! # Examples
//! Using `and_then` on an `Outcome`:
//! 
//! ```
//! use outcome::*;
//! 
//! // Returns `Failure`
//! let result = Outcome::from_bool(false);
//! 
//! match result.and_then(|| Success) {
//!     Success => println!("Success! :)"),
//!     Failure => println!("Failure :("),
//! }
//! ```
//! 
//! Using `or_none` on an `Outcome` to transform it into an `Option`:
//! 
//! ```
//! use outcome::*;
//! 
//! let result = Success;
//! 
//! // Encapsulates arg within an option
//! match result.or_none("hello!") {
//!     Some(s) => println!("{}", s),
//!     None => println!("Nothing here!"),
//! }
//! ```

pub use Outcome::{Success, Failure};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Outcome {
    /// Successful
    Success,
    /// Not successful
    Failure
}

impl Outcome {
    /// Returns `Success` if `good` is `true`, otherwise return `Failure`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Outcome::from_bool(true);
    /// 
    /// assert_eq!(result, Success);
    /// ```
    pub fn from_bool(good: bool) -> Outcome {
        match good {
            true => Success,
            false => Failure,
        }
    }

    /// Returns `true` if the outcome is a success
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Success;
    /// 
    /// assert!(result.is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        *self == Success
    }

    /// Returns `true` if the outcome is a failure
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Failure;
    /// 
    /// assert!(result.is_failure());
    /// ```
    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    /// Transforms the `Outcome` into an `Option<T>`, mapping `Success` to `Some(good)` and `Failure` to `None`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Success;
    /// 
    /// assert_eq!(result.or_none(42), Some(42));
    /// ```
    pub fn or_none<T>(self, ok: T) -> Option<T> {
        match self {
            Success => Some(ok),
            Failure => None,
        }
    }

    /// Transforms the `Outcome` into a `Result<T, E>`, mapping `Success` to `Ok(good)` and `Failure` to `Err(err)`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Failure;
    /// 
    /// match result.or_err("good", "bad") {
    ///     Ok(success) => println!("{}", success),
    ///     Err(err) => println!("{}", err),
    /// }
    /// ```
    pub fn or_err<T, E>(self, good: T, err: E) -> Result<T, E> {
        match self {
            Success => Ok(good),
            Failure => Err(err),
        }
    }

    /// Returns `good` if the outcome is `Success`, otherwise panics
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Success;
    /// 
    /// assert_eq!(result.or_panic(42), 42);
    /// ```
    pub fn or_panic<T>(self, good: T) -> T {
        match self {
            Success => good,
            Failure => panic!("Called `Outcome::or_panic(...)` on a `Failure` value"),
        }
    }

    /// Returns `Failure` if the outcome is `Failure`, otherwise returns `outb`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Success;
    /// 
    /// assert_eq!(result.and(Failure), Failure);
    /// ```
    pub fn and(self, outb: Outcome) -> Outcome {
        match self {
            Success => outb,
            Failure => Failure,
        }
    }

    /// Returns `Success` if the outcome is `Success`, otherwise returns `outb`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Failure;
    /// 
    /// assert_eq!(result.or(Success), Success);
    /// ```
    pub fn or(self, outb: Outcome) -> Outcome {
        match self {
            Success => Success,
            Failure => outb,
        }
    }

    /// Returns `Failure` if the outcome is `Failure`, otherwise calls `f` and returns result
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Success;
    /// 
    /// assert_eq!(result.and_then(|| Failure), Failure);
    /// ```
    pub fn and_then<F: FnOnce() -> Outcome>(self, f: F) -> Outcome {
        match self {
            Success => f(),
            Failure => Failure,
        }
    }

    /// Returns `Success` if the outcome is `Success`, otherwise calls `f` and returns result
    /// 
    /// # Examples
    /// 
    /// ```
    /// use outcome::*;
    /// 
    /// let result = Failure;
    /// 
    /// assert_eq!(result.or_then(|| Success), Success);
    /// ```
    pub fn or_then<F: FnOnce() -> Outcome>(self, f: F) -> Outcome {
        match self {
            Success => Success,
            Failure => f(),
        }
    }
}