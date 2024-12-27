// Imports from Optimizely crate
use optimizely::client::UninitializedClient;
use optimizely_proc_macro::compile_datafile;

#[test]
fn test_optimizely() {
    let datafile = compile_datafile!(sdk_key);

    let client = UninitializedClient::new(datafile).initialize();

    assert!(client.datafile().revision() >= 141);
    assert_eq!(client.datafile().account_id(), String::from("21537940595"));
}
