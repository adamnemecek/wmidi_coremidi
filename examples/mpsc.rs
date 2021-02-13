pub struct Event {
    timestamp: std::time::Instant,
}

impl Event {
    pub fn now() -> Self {
        Self {
            timestamp: std::time::Instant::now(),
        }
    }
}
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || loop {
        let z = tx.send(std::time::Instant::now());
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    loop {
        let e = rx.recv().unwrap();
        let now = std::time::Instant::now();
        println!("diff {:?}", now - e);
    }
}
