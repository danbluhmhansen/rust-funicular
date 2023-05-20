use pgrx::prelude::*;

mod char_aggr_sync;
mod refresh_char_aggr;
mod refresh_char_aggr_trigger;

pgrx::pg_module_magic!();

#[cfg(any(debug_assertions, test))]
extension_sql_file!("../static/up.sql");

#[pg_extern]
fn hello_funicular() -> &'static str {
    "Hello, funicular"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_funicular() {
        assert_eq!("Hello, funicular", crate::hello_funicular());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
