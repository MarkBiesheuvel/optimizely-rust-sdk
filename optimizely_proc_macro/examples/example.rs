// Imports from Optimizely crate
use optimizely::client::UninitializedClient;
use optimizely_proc_macro::compile_datafile;

fn main() {
    let datafile = compile_datafile!(sdk_key);

    let client = UninitializedClient::new(datafile).initialize();

    println!("{} @ v{}", client.datafile().account_id(), client.datafile().revision());
}
