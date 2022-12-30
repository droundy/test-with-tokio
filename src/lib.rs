//! A macro to enable locks on tokio-based tests.
//!
//! This crate provides a single polite attribute macro
//! `#[test_with_tokio::please]` which allows you to write tests that do some
//! not-async code before running async code within tokio, so this is similar to
//! `#[tokio::test]` but with different bells and whistles (apart from the
//! guard). This enables you to run most of your tests in parallel, but to have a
//! few that cannot be run concurrently.
//!
//! # Examples
//! ```
//! #[test_with_tokio::please]
//! fn test_me() {
//!     println!("This code will be run before the tokio runtime is started.");
//!     async {
//!         println!("This code will be run under tokio");
//!     }
//! }
//! ```
//! ## Holding a lock
//! ```
//! static DIRECTORY_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());
//!
//! #[test_with_tokio::please]
//! fn test_run_exclusively() {
//!     let _guard = DIRECTORY_LOCK.write().unwrap();
//!     async {
//!         println!("This code will be run with exclusive access to the directory.");
//!     }
//! }
//!
//! #[test_with_tokio::please] fn test_run_cooperatively() {
//!     let _guard = DIRECTORY_LOCK.read().unwrap();
//!     async {
//!         println!("This code may be run concurrently with other cooperative tests..");
//!     }
//! }
//! ```
//!
//! ## Multiple cases
//!
//! If you can write code that generates multiple related tests by assigning a variable
//! to `match CASE { ... }` where each case matches a string literal.
//! ```
//! #[test_with_tokio::please] fn test_contains() {
//!     let container = match CASE {
//!         "hello" => "hello world",
//!         "this_test" => vec!["this_test"],
//!     };
//!     async {
//!         assert!(container.contains(CASE));
//!     }
//! }
//! ```
//! You might wonder, why not take the lock within a function marked with
//! `#[tokio::test]`? The answer lies in the lack of an `async` `Drop`.  This
//! means that the evil test isn't fully cleaned up until after the tokio
//! wrapper exits, which is *after* the body of your test function has exited
//! and released the lock.

/// Run a test using tokio, possibly with extra cases and possibly running extra
/// code synchronously.
///
/// # Examples
/// ```
/// #[test_with_tokio::please]
/// fn test_me() {
///     println!("This code will be run before the tokio runtime is started.");
///     async {
///         println!("This code will be run under tokio");
///     }
/// }
/// ```
/// ## Holding a lock
/// ```
/// static DIRECTORY_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());
///
/// #[test_with_tokio::please]
/// fn test_run_exclusively() {
///     let _guard = DIRECTORY_LOCK.write().unwrap();
///     async {
///         println!("This code will be run with exclusive access to the directory.");
///     }
/// }
///
/// #[test_with_tokio::please] fn test_run_cooperatively() {
///     let _guard = DIRECTORY_LOCK.read().unwrap();
///     async {
///         println!("This code may be run concurrently with other cooperative tests..");
///     }
/// }
/// ```
///
/// ## Multiple cases
///
/// If you can write code that generates multiple related tests by assigning a variable
/// to `match CASE { ... }` where each case matches a string literal.
/// ```
/// #[test_with_tokio::please] fn test_contains() {
///     let container = match CASE {
///         "hello" => "hello world",
///         "this_test" => vec!["this_test"],
///     };
///     async {
///         assert!(container.contains(CASE));
///     }
/// }
/// ```
#[doc(inline)]
pub use test_with_tokio_macros::please;
