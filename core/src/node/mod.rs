#![warn(dead_code)]

mod job;

use crate::metrics::Biz as MockMetrics;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use typed_builder::TypedBuilder;

#[derive(Debug, Default)]
pub struct MockConfig; //FIXME

#[derive(Debug, Default)]
pub struct MockInfo; //FIXME

#[derive(Debug, Default)]
pub struct MockSource; //FIXME

/// Static hook for returning a closure that initiates Node shutdown
fn init_shutdown_hook() -> fn() -> () {
    unimplemented!("init_shutdown_hook");
    || println!("[Unimplemented] Shutting down...")
}

static NUM_WORKERS: usize = 4;

/// Defines where and how a Node should be treated
#[derive(Debug)]
pub enum ClusteringType {
    Local,
    Remote,
    // In a managed cluster setting, i.e. Kubernetes
    Distributed,
}

// TODO : Metrics polling
// TODO : Kill Mark Cleanup
// TODO : Running tasks
// TODO : Shutdown
// TODO : Heartbeat

/// Execution context for a worker
/// this can be :
/// - a local context (local host / manual)
/// - a remote context (ssh/ ssh tunnel/ web api)
/// - a distributed context (kubernetes)
#[derive(Debug, TypedBuilder)]
pub struct Node {
    id: String,
    #[builder(default = "local".to_string())]
    hostname: String,
    /// Local, Remote, Distributed
    #[builder(default = ClusteringType::Local)]
    clustering_type: ClusteringType,
    // TODO : add Node struct information
    #[builder(default)]
    resources: MockInfo,

    /// Files and archives to track for the node's execution context
    cur_files: HashMap<String, String>,
    cur_archives: HashMap<String, String>,
    // temporary computations / streaming
    BYTE_BUFFER: Vec<u8>,

    #[builder(default)]
    config: MockConfig,
    /// Reserved for computational tasks
    #[builder(default = ThreadPool::new(NUM_WORKERS))]
    worker_pool: ThreadPool,
    ///  Reserved for killing and interrupting running tasks
    #[builder(default = None)]
    reaper_pool: Option<ThreadPool>,
    #[builder(default)]
    source: MockSource,
    /// Measure runtime metrics
    #[builder(default = None)]
    metrics: Option<MockMetrics>,
    /// TODO: implement
    #[builder(default = None)]
    self_serializer: Option<String>,
    #[builder(default = 10000)]
    // TTL in milliseconds, default = 10 seconds
    TTL: u64,
}

impl Node {
    fn new(clustering_type: ClusteringType, id: &str) -> Self {
        let new_node = Node::builder()
            .clustering_type(clustering_type)
            .id(id.to_string())
            .hostname("localhost".to_string())
            .BYTE_BUFFER(Vec::new())
            .cur_files(HashMap::new())
            .cur_archives(HashMap::new())
            .build();

        match &new_node.clustering_type {
            Local => {
                println!("local selected")
            }
            Remote => {
                // validate remote hostname / connection
                println!("remote selected")
            }
            Distributed => {
                // validate cluster hostname
                println!("distribured selected")
            }
        }
        new_node
    }

    /// Read from a valid Node configuration file
    fn from_config(node_config_file: &str) -> Result<Self, String> {
        unimplemented!("from_config");
    }

    fn num_running_tasks(&self) -> usize {
        unimplemented!("num_running_tasks");
    }

    fn decommision(&self) -> Result<(), String> {
        unimplemented!("decommision");
    }

    fn launch_task(&self, task: &str) -> Result<(), String> {
        unimplemented!("launch_task");
    }

    fn kill_task(&self, task: &str) -> Result<(), String> {
        unimplemented!("kill_task");
    }

    fn kill_all_tasks(&self) -> Result<(), String> {
        unimplemented!("kill_all_tasks");
    }

    fn stop(&self) -> Result<(), String> {
        unimplemented!("stop");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod crate_integration {
        use super::*;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::{Arc, Barrier};
        #[test]
        fn spawnexec_independent_workers() {
            let n_workers = 4;
            let n_jobs = 8;
            let pool = ThreadPool::new(n_workers);

            let (tx, rx) = channel();

            for _ in 0..n_jobs {
                let tx = tx.clone();
                pool.execute(move || {
                    tx.send(1).expect("channel will be there");
                });
            }
            assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), n_jobs);
        }

        #[test]
        fn spawnexec_barrier_workers() {
            let n_workers = 42;
            let n_jobs = 23;
            let pool = ThreadPool::new(n_workers);
            let an_atomic = Arc::new(AtomicUsize::new(0));

            assert!(n_jobs <= n_workers, "too many jobs will deadlock");

            // create a barrier that waits for all jobs plus the starter thread
            let barrier = Arc::new(Barrier::new(n_jobs + 1));
            for _ in 0..n_jobs {
                let barrier = barrier.clone();
                let an_atomic = an_atomic.clone();

                pool.execute(move || {
                    // heavy work
                    an_atomic.fetch_add(1, Ordering::Relaxed);

                    barrier.wait();
                })
            }
            barrier.wait();
            assert_eq!(an_atomic.load(Ordering::SeqCst), /*n_jobs*/ 23);
        }
    }
}
