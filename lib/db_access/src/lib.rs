struct DBAccess {
    connection_str: String,
}

impl DBAccess {
    /// DB access constructor
    fn new(hostname: &str) -> DBAccess {
        DBAccess {
            connection_str: format!("host={} user=postgres", hostname),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_connection_string() {
        let result = DBAccess::new("localhost");
        assert_eq!(result.connection_str, "host=localhost user=postgres");
    }
}
