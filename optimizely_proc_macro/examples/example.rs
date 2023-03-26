// Imports from Optimizely crate
use optimizely::client::ClientBuilder;
use optimizely_proc_macro::compile_datafile;

fn main() {
    let datafile = compile_datafile!(sdk_key);

    let client = ClientBuilder::new()
        .with_native_datafile(datafile)
        .build();

    println!("{} @ v{}", client.account_id(), client.revision());
}