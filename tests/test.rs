#[test_with_tokio::please]
fn empty_string() {
    let name = "";
    async {
        assert_eq!(name, "");
        assert!(name.is_empty());
    }
    .await
}

static LOCK: std::sync::RwLock<u64> = std::sync::RwLock::new(0);

#[test_with_tokio::please]
fn with_write_lock() {
    let mut guard = LOCK.write().unwrap();
    async {
        *guard = 2;
    }
    .await
}

#[test_with_tokio::please]
fn with_read_lock() {
    std::thread::sleep(std::time::Duration::from_secs(1));
    let guard = LOCK.read().unwrap();
    async {
        assert_eq!(*guard, 2);
    }
    .await
}

#[test_with_tokio::please]
fn with_color() {
    let color = match CASE {
        "red" => 1,
        "green" => 2,
    };
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    if CASE == "red" {
        assert_eq!(color, 1);
    } else {
        assert_eq!(color, 2);
    }
}

static MYLOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
#[test_with_tokio::please]
fn test_pill() -> std::io::Result<()> {
    let contents = match CASE {
        "red" => "red pill",
        "blue" => "blue pill",
    };
    let _guard = MYLOCK.lock().unwrap();
    let mut f = tokio::fs::File::create("pill.txt").await?;
    use tokio::io::AsyncWriteExt;
    f.write_all(contents.as_bytes()).await?;
    // do other stuff that needs the file to exist
    tokio::fs::remove_file("pill.txt").await
}
