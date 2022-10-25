# Miette Helper

A library providing helper macros for Miette as well as some general error-handling helper macros

I recommend to use it in conjuntion with [unwrap or](www.docs.rs/unwrap_or)

## Provided Functions
### wrap_err
Converts given Result into Diagnostic and returns it. Made for applications not using a custom error enum/struct.
```rust
let a: Result<u32, std::io::Error> = Ok(42);
let b: u32 = wrap_err!(a, "Failed to calulate number")? + 27;
```

### or_wrap_err
Converts given Option into Diagnostic with the given error message. 
```
let b: Option<&str> = None;
let c: &str = or_wrap_err!(b, "No path specified")?
```