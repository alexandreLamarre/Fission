use std::path;
use typed_builder::TypedBuilder;

// Format enum to specify the file structure / assumptions of the
// file to read
#[derive(Debug, PartialEq)]
pub enum ReaderFormat {
    Csv,
    Json,
    Yaml,
    Toml,
    None, // For unstructured format
}

#[derive(TypedBuilder)]
struct Reader {
    format: ReaderFormat,
    path: String,
}

impl Reader {
    fn compile() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let reader = Reader::builder()
            .format(ReaderFormat::Json)
            .path("/path/to/file".to_string())
            .build();
        assert_eq!(reader.format, ReaderFormat::Json);
        assert_eq!(reader.path, "/path/to/file");
    }
}
