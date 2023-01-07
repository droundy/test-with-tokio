[![Workflow Status](https://github.com/droundy/test-with-tokio/workflows/main/badge.svg)](https://github.com/droundy/test-with-tokio/actions?query=workflow%3A%22main%22)

# test-with-tokio

A macro to enable locks on tokio-based tests.

This crate provides a single polite attribute macro
`#[test_with_tokio::please]` which allows you to write tests that do some
not-async code before running async code within tokio, so this is similar to
`#[tokio::test]` but with different bells and whistles. With a bit of work,
this enables you to run most of your tests in parallel, but to have a few
that cannot be run concurrently.

## Examples

At the most basic level, this crate enables you to easily write tests that
run non-async code that will be run prior to async code.
```rust
// The async in `async fn` below is optional and ignored.
#[test_with_tokio::please]
async fn test_me() {
    println!("This code will be run before the tokio runtime is started.");
    async_std::println!("This code will be run with a tokio runtime").await;
}
```
### Holding a lock
The motivating reason for this crate is to enable use of a lock to run tests
concurrently:
```rust
static DIRECTORY_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());

#[test_with_tokio::please]
fn test_run_exclusively() {
    let _guard = DIRECTORY_LOCK.write().unwrap();
    async_std::println!("This code will be run with exclusive access to the directory.").await;
}

#[test_with_tokio::please] fn test_run_cooperatively() {
    let _guard = DIRECTORY_LOCK.read().unwrap();
    async_std::println!("This code will be run concurrently with other cooperative tests..").await;
}
```
You might wonder, why not take the lock within the `async` block, or perhaps
simply within a function marked with `#[tokio::test]`? The answer lies in
the lack of an `async` `Drop`.  This means that a test may not be fully
cleaned up until *after*  the tokio runtime exits, which is *after* the body
of your test function has exited and released the lock, meaning you may
still have race conditions in your tests, with a lock taken concurrently.

### Multiple cases

If you can write code that generates multiple related tests by assigning a
variable to `match CASE { ... }` where each case matches a string literal
that is a valid suffix for an identifier.
```rust
#[test_with_tokio::please]
fn test_contains() {
    let container = match CASE {
        "hello" => "hello world",
        "this_test" => vec!["this_test"],
    };
    assert!(container.contains(CASE));
}
```
This example will create two functions each marked `#[test]`, one named
`test_contains_hello` and the other `test_contains_this_test`.  The body of
the first function will look like:
```rust
#[test]
fn test_contains_hello() {
    const CASE: &str = "hello";
    let container = "hello world";
    assert!(container.contains(CASE));
}
```


License: MIT
