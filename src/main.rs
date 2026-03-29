use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(elapsed) => {
            let seconds_since_epoch: u64 = elapsed.as_secs();
            let entry = Entry::new("test".to_string(), Some(seconds_since_epoch));

            println!(
                "value: {}, is_expired: {}",
                entry.value,
                entry.is_expired(seconds_since_epoch)
            );
            ()
        }
        Err(e) => {
            // This case handles situations where the system time is set before the Unix epoch.
            eprintln!("SystemTime before UNIX EPOCH! {:?}", e);
        }
    }
}

struct Entry {
    value: String,
    expires_at: Option<u64>,
}

impl Entry {
    fn new(value: String, expires_at: Option<u64>) -> Entry {
        Entry { value, expires_at }
    }

    fn is_expired(&self, now: u64) -> bool {
        match self.expires_at {
            Some(exp) => now >= exp,
            None => false,
        }
    }
}
