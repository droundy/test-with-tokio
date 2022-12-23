use test_with_tokio::test_with;

#[test_with("")]
fn empty_string() {
    assert_eq!(_guard, "");
    assert!(_guard.is_empty());
}

static LOCK: std::sync::RwLock<u64> = std::sync::RwLock::new(0);

#[test_with(LOCK.write().unwrap())]
fn with_write_lock() {
    *_guard = 2;
}

#[test_with({ std::thread::sleep(std::time::Duration::from_secs(1)); LOCK.read().unwrap()})]
fn with_read_lock() {
    assert_eq!(*_guard, 2);
}
