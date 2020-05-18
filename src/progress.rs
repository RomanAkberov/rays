use std::sync::atomic::{AtomicU32, Ordering};

pub trait Progress: Sync {
    fn increment(&self, total: u32);
}

#[derive(Default)]
pub struct NoProgress;

impl Progress for NoProgress {
    fn increment(&self, _total: u32) {}
}

#[derive(Default)]
pub struct ConsoleProgress(AtomicU32);

impl Progress for ConsoleProgress {
    fn increment(&self, total: u32) {
        let progress = self.0.fetch_add(1, Ordering::SeqCst) + 1;
        println!("{:.5}% ({}/{})", 100.0 * progress as f64 / total as f64, progress, total);
    }
}
