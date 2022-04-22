#![warn(dead_code)] //FIXME: remove after initial development

// Dataframe trait specifies the API
// each dataframe must implement.
trait DF {
    fn get_column(&self, name: &str) -> Vec<&str>;
}

// Internally, each RDD is characterized by five main properties:
//
//  - A list of partitions
//  - A function for computing each split
//  - A list of dependencies on other RDDs
//  - Optionally, a Partitioner for key-value RDDs (e.g. to say that the RDD is hash-partitioned)
//  - Optionally, a list of preferred locations to compute each split on (e.g. block locations for
//    an HDFS file)
struct RDD<T> {
    partitions: Vec<Partition>,
    compute: fn(partition: &Partition) -> T,
    dependencies: Vec<RDD<T>>,
    partitioner: Option<Partitioner>,
    preferred_locations: Vec<String>,
}

// Stores metadata for a partition of a Dataframe
enum Partition {
    Plain,
    Block,
    Empty,
}

// Defines the strategy for partitioning a Dataframe
enum Partitioner {
    Hash,
    Empty,
}

#[cfg(tests)]
mod tests {
    use super::*;
}
