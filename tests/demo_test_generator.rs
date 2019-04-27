#[cfg(test)]
extern crate test_generator;

#[cfg(test)]
test_generator::test_expand_paths! {
     file_tests; "data/*/*.idl"
}

fn file_tests(file_path: &str) {
    // use 'file_name' as input for your test
    assert!(
        std::path::Path::new(file_path).exists()
    );
}
