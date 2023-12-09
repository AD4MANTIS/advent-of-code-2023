use std::time::SystemTime;

pub struct PrintTimer(SystemTime, String);

impl PrintTimer {
    pub fn new(label: &str) -> Self {
        Self(SystemTime::now(), label.to_string())
    }
}

impl Drop for PrintTimer {
    fn drop(&mut self) {
        println!(
            "{}{}Duration: {}s",
            self.1,
            if self.1.is_empty() { "" } else { ": " },
            self.0.elapsed().unwrap_or_default().as_secs_f64()
        );
    }
}
