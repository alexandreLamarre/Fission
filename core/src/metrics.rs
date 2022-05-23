#![warn(dead_code)] //FIXME: remove after initial development

/// Module that uses `metered` and polling helpers to report
/// system & application metrics to a variety of formats (data sinks).
use metered::{metered, HitCount, Throughput};
use serde;
use std::sync::Arc;
use std::thread;

#[derive(Default, Debug, serde::Serialize)]
pub struct Biz {
    metrics: BizMetrics,
}

#[metered(registry = BizMetrics)]
impl Biz {
    #[measure([HitCount, Throughput])]
    pub fn biz(&self) {
        let delay = std::time::Duration::from_millis(rand::random::<u64>() % 200);
        std::thread::sleep(delay);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_biz() {
        let biz = Arc::new(Biz::default());
        let mut threads = Vec::new();
        for _ in 0..5 {
            let biz = Arc::clone(&biz);
            let t = thread::spawn(move || {
                for _ in 0..200 {
                    biz.biz();
                }
            });
            threads.push(t);
        }
        for t in threads {
            t.join().unwrap();
        }
        // Print the results!
        let serialized = serde_yaml::to_string(&*biz).unwrap();
        println!("{}", serialized);
    }
}
