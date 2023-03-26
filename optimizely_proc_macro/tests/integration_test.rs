// Imports from Optimizely crate
use optimizely::client::ClientBuilder;
use optimizely_proc_macro::compile_datafile;

#[test]
fn test_optimizely() {
    let datafile = compile_datafile!(sdk_key);

    let client = ClientBuilder::new()
        .with_native_datafile(datafile)
        .build();

    assert!(client.revision() >= 141);
    assert_eq!(client.account_id(), "21537940595".to_owned());
}