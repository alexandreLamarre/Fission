#![warn(dead_code)] //FIXME: remove after initial development

// Dataset trait implements transformations on the object
// that implements it.
//
// Each Dataset implementation must also specify the following types:
// - Item: the type of the items in the dataset
// - Key : the type of the keys (or indexing mechanism) in the dataset
// - Value : the type of the mapped values should hold
//
// More concretely, in the example of RDDs, they are uniquely
// created and modified via these transformations.
trait Dataset {
    type Item;
    type Key;
    type Value;
    fn map(&self, f: dyn Fn(&Self::Item)) -> Self;
    fn filter(&self, f: &Fn(&Self::Item) -> bool) -> Self;
    fn sample(&self, n: usize) -> Self;
    fn group_by_key(&self, f: &Fn(&Self::Item) -> &Self::Key) -> Self;
    fn reduce_by_key(&self, f: &Fn(&Self::Item, &Self::Item) -> Self::Item) -> Self;
    fn sort_by_key(&self, f: &Fn(&Self::Item) -> &Self::Key) -> Self;
    fn flat_map(&self, f: &Fn(&Self::Item) -> Self) -> Self;
    fn union(&self, other: &Self) -> Self;
    fn join(&self, other: &Self, f: &Fn(&Self::Item) -> &Self::Key) -> Self;
    fn cogroup(&self, other: &Self, f: &Fn(&Self::Item) -> &Self::Key) -> Self;
    fn cross(&self, other: &Self) -> Self;
    fn map_values(&self, f: &Fn(&Self::Item) -> Self::Value) -> Self;
}

// Actions on Datasets that must be implemented
//
// More specfically Actions specify aggregation API/methods on
// Objects that should implement that Dataset trait.
trait DatasetAction {
    type Item;
    type Key;
    fn collect(&self) -> Vec<Self::Item>;
    fn reduce(&self, f: &Fn(&Self::Item, &Self::Item) -> Self::Item) -> Self::Item;
    fn count(&self) -> usize;
    fn save(&self, path: &str);
    fn lookup_key(&self, key: &Self::Key) -> Option<&Self::Item>;
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

// Resilient Distributed Dataset (RDD)
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

#[cfg(tests)]
mod tests {
    use super::*;
}
