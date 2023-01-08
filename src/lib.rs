//! A macro to enable locks on tokio-based tests.
//!
//! This crate provides a single polite attribute macro
//! `#[test_with_tokio::please]` which allows you to write tests that do some
//! not-async code before running async code within tokio. This is similar to
//! `#[tokio::test]` but with two features: async code can be run prior to the
//! tokio runtime being started, and a single test can be written to generate
//! multiple tests handling multiple cases of the same test.  With a bit of
//! work, this enables you to run most of your tests in parallel, but to have a
//! few that cannot be run concurrently.
//!
//! # Examples
//!
//! At the most basic level, this crate enables you to easily write tests that
//! run non-async code that will be run prior to async code.  FIXME CODE AFTER
//! FIRST ASYNC BLOCK OR AWAIT WILL RUN IN TOKIO RUNTIME
//! ```
//! // The async in `async fn` below is optional and ignored.
//! #[test_with_tokio::please]
//! async fn test_me() {
//!     println!("This code will be run before the tokio runtime is started.");
//!     async_std::println!("This code will be run with a tokio runtime").await;
//! }
//! ```
//! ## Holding a lock
//! The motivating reason for this crate is to enable use of a lock to run tests
//! concurrently:
//! ```
//! static DIRECTORY_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());
//!
//! #[test_with_tokio::please]
//! fn test_run_exclusively() {
//!     let _guard = DIRECTORY_LOCK.write().unwrap();
//!     async_std::println!("This code will be run with exclusive access to the directory.").await;
//! }
//!
//! #[test_with_tokio::please] fn test_run_cooperatively() {
//!     let _guard = DIRECTORY_LOCK.read().unwrap();
//!     async_std::println!("This code will be run concurrently with other cooperative tests..").await;
//! }
//! ```
//! You might wonder, why not take the lock within the `async` block, or perhaps
//! simply within a function marked with `#[tokio::test]`? The answer lies in
//! the lack of an `async` `Drop`.  This means that a test may not be fully
//! cleaned up until *after*  the tokio runtime exits, which is *after* the body
//! of your test function has exited and released the lock, meaning you may
//! still have race conditions in your tests, with a lock taken concurrently.
//!
//! ## Multiple cases
//!
//! If you can write code that generates multiple related tests by assigning a
//! variable to `match CASE { ... }` where each case matches a string literal
//! that is a valid suffix for an identifier.
//! ```
//! #[test_with_tokio::please]
//! fn test_contains() {
//!     let container = match CASE {
//!         "hello" => "hello world",
//!         "this_test" => vec!["this_test"],
//!     };
//!     assert!(container.contains(CASE));
//! }
//! ```
//! This example will create two functions each marked `#[test]`, one named
//! `test_contains_hello` and the other `test_contains_this_test`.  The body of
//! the first function will look like:
//! ```
//! #[test]
//! fn test_contains_hello() {
//!     const CASE: &str = "hello";
//!     let container = "hello world";
//!     assert!(container.contains(CASE));
//! }
//! ```
//!

/// Run a test possibly using tokio, possibly with extra cases.
///
/// Everything before the first `await` or `async` block is run before the tokio
/// runtime is started, and the remainder of the function is run within the
/// tokio runtime.
///
/// In addition, if there is a statement of the form `let ... = match CASE { ... }`
/// then the match must match from string literals that are valid identifier suffixes,
/// and those cases are each used to generate a new test function.  For details, see
/// the example below.
///
/// See module-level documentation for more and better examples.
///
/// # Examples
///
/// The following code will create two tests that safely write to the same file.
/// ```
/// static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
/// #[test_with_tokio::please]
/// fn test_pill() -> std::io::Result<()> {
///     let contents = match CASE {
///         "red" => "red pill",
///         "blue" => "blue pill",
///     };
///     let _guard = LOCK.lock().unwrap();
///     let mut f = tokio::fs::File::create("pill.txt").await?;
///     use tokio::io::AsyncWriteExt;
///     f.write_all(contents.as_bytes()).await?;
///     // do other stuff that needs the file to exist
///     tokio::fs::remove_file("pill.txt").await
/// }
/// ```
/// this will expand to
/// ```
/// static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
///
/// #[test]
/// fn test_pill_red() -> std::io::Result<()> {
///     const CASE: &str = "red";
///     let contents = "red pill";
///     let _guard = LOCK.lock().unwrap();
///     ::tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             let mut f = tokio::fs::File::create("pill.txt").await?;
///             use tokio::io::AsyncWriteExt;
///             f.write_all(contents.as_bytes()).await?;
///             // do other stuff that needs the file to exist
///             tokio::fs::remove_file("pill.txt").await
///         });
/// }
///
/// #[test]
/// fn test_pill_blue() -> std::io::Result<()> {
///     const CASE: &str = "blue";
///     let contents = "blue pill";
///     let _guard = LOCK.lock().unwrap();
///     ::tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .build()
///         .unwrap()
///         .block_on(async {
///             let mut f = tokio::fs::File::create("pill.txt").await?;
///             use tokio::io::AsyncWriteExt;
///             f.write_all(contents.as_bytes()).await?;
///             // do other stuff that needs the file to exist
///             tokio::fs::remove_file("pill.txt").await
///         });
/// }
/// ```
#[doc(inline)]
pub use test_with_tokio_macros::please;
