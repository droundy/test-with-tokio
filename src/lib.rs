#![doc = include_str!("../README.md")]

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
