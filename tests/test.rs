#[test_with_tokio::please]
fn empty_string() {
    let name = "";
    async {
        assert_eq!(name, "");
        assert!(name.is_empty());
    }
}

static LOCK: std::sync::RwLock<u64> = std::sync::RwLock::new(0);

#[test_with_tokio::please]
fn with_write_lock() {
    let mut guard = LOCK.write().unwrap();
    async {
        *guard = 2;
    }
}

#[test_with_tokio::please]
fn with_read_lock() {
    let guard = {
        std::thread::sleep(std::time::Duration::from_secs(1));
        LOCK.read().unwrap()
    };
    async {
        assert_eq!(*guard, 2);
    }
}

#[test_with_tokio::please]
fn with_color() {
    let color = match CASE {
        "red" => 1,
        "green" => 2,
    };
    async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        if CASE == "red" {
            assert_eq!(color, 1);
        } else {
            assert_eq!(color, 2);
        }
    }
}
