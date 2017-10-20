[![Build Status](https://travis-ci.org/lachlansneff/outcome-rust.svg?branch=master)](https://travis-ci.org/lachlansneff/outcome-rust)
[![Latest Version](https://img.shields.io/crates/v/outcome.svg)](https://crates.io/crates/outcome)

# The outcome crate
Type `Outcome` represents a success or failure: Every `Outcome` is either `Success` or `Failure`

```
use outcome::*;
 
fn do_something() -> Outcome {
    Success
}
 
// The return value is an outcome
let result = do_something();
 
// Pattern Match
match result {
    Success => println!("Well done!"),
    Failure => println!("Oh well :("),
}
```

# Examples
Using `and_then` on an `Outcome`:

```
/use outcome::*;
 
// Returns `Failure`
let result = Outcome::from_bool(false);
 
match result.and_then(|| Success) {
    Success => println!("Success! :)"),
    Failure => println!("Failure :("),
}
```

Using `or_none` on an `Outcome` to transform it into an `Option`:

```
use outcome::*;
 
let result = Success;
 
// Encapsulates arg within an option
match result.or_none("hello!") {
    Some(s) => println!("{}", s),
    None => println!("Nothing here!"),
}
```
