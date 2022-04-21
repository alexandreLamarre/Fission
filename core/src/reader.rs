use std::path;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
struct Reader {
    format: String,
    load: String,
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
            .format("json".to_string())
            .load("/path/to/file".to_string())
            .build();
        assert_eq!(reader.format, "json");
        assert_eq!(reader.load, "/path/to/file");
    }
}
