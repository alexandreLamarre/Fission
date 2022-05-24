#![warn(dead_code)]

use super::ClusteringType;
use crate::metrics::Biz as MockMetrics;
use typed_builder::TypedBuilder;

/// Jobs scheduled by a Node
#[derive(Debug, TypedBuilder)]
struct Job {
    backend: ClusteringType,
    #[builder(default)]
    name: String,
    #[builder(default = -1)]
    thread_id: i32,
}

/// Supervises the killing / cancellation of a task by sending the interrupted flag,
///  optionally sending a Thread.interrupt(), and monitoring the task until it finishes.
#[derive(Debug, TypedBuilder)]
struct JobReaper {
    interrupt: bool,
    #[builder(default = "".to_string())]
    reason: String,
}

impl Job {
    fn new() -> Self {
        Job::builder().backend(ClusteringType::Local).build()
    }

    fn is_finished() -> bool {
        unimplemented!("is_finished");
    }

    fn is_fatal_err() -> bool {
        unimplemented!("is_fatal_err");
    }

    fn set_task() {
        unimplemented!("set_task");
    }

    fn run() {
        unimplemented!("run");
    }
}

impl JobReaper {
    fn new() -> Self {
        JobReaper::builder()
            .interrupt(false)
            .reason("".to_string())
            .build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_it_works() {
        assert_eq!(2 * 2, 4);
    }
}
