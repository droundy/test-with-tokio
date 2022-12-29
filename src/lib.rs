//! A macro to enable locks on tokio-based tests.
//!
//! This crate provides a single attribute macro `#[test_with(GUARDEXPRESSION)]`
//! which allows you to provide a guard expression, and is otherwise like
//! `#[tokio::test]` with fewer bells and whistles (apart from the guard).  It
//! enables you to run most of your tests in parallel, but to have a few that
//! cannot be run concurrently.
//! ```notest
//! #[test]
//! fn this_test_can_run_with_anything() { ... }
//!
//! static DIRECTORY_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());
//!
//! /// This test cannot be run alongside any tests that use the directory.
//! #[test_with(DIRECTORY_LOCK.write().unwrap())]
//! fn evil_test() { ... }
//!
//! /// This test can be run with other tests using the directory, but not with the evil test.
//! #[test_with({ DIRECTORY_LOCK.read().unwrap()})]
//! fn nicer_test() { ... }
//! ```
//! You might wonder, why not take the lock within a function marked with
//! `#[tokio::test]`? The answer lies in the lack of an `async` `Drop`.  This
//! means that the evil test isn't fully cleaned up until after the tokio
//! wrapper exits, which is *after* the body of your test function has exited
//! and released the lock.

/// Run a test with a given expression held
///
/// ```notest
/// #[test_with_tokio::with(std::fs::open(""))]
/// ```
#[doc(inline)]
pub use test_with_tokio_macros::please;
