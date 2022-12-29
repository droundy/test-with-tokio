#[test_with_tokio::with(let name = "")]
fn empty_string(name: &str) {
    assert_eq!(name, "");
    assert!(name.is_empty());
}

static LOCK: std::sync::RwLock<u64> = std::sync::RwLock::new(0);

#[test_with_tokio::with(let guard = LOCK.write().unwrap())]
fn with_write_lock(mut guard: std::sync::RwLockWriteGuard<'static, u64>) {
    *guard = 2;
}

#[test_with_tokio::with(let  guard = { std::thread::sleep(std::time::Duration::from_secs(1)); LOCK.read().unwrap()})]
fn with_read_lock(guard: std::sync::RwLockReadGuard<'static, u64>) {
    assert_eq!(*guard, 2);
}
