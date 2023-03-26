// Imports from Optimizely crate
use optimizely_proc_macro::make_answer;

// Run macro
make_answer!();

#[test]
fn test_answer() {
    assert_eq!(42, answer());
}