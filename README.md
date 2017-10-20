# The outcome crate
Type `Outcome` represents a success or failure: Every `Outcome` is either `Success` or `Failure`

```
fn do_something() -> Outcome {
    // ...
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
// Returns `Failure`
let result = Outcome::from_bool(false);

match result.and_then(|| Success) {
    Success => println!("Success! :)"),
    Failure => println!("Failure :("),
}
```

Using `or_none` on an `Outcome` to transform it into an `Option`:

```
let result = Success;

// Encapsulates arg within an option
match result.or_none("hello!") {
    Some(s) => println!("{}", s),
    None => println!("Nothing here!"),
}
```
