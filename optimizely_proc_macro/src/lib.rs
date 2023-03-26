use proc_macro::TokenStream;
use quote::quote;
use optimizely::datafile::{Json, Datafile};

#[proc_macro]
pub fn compile_datafile(_: TokenStream) -> TokenStream {
    // Retrieve latest datafile and parse it at compile time
    let sdk_key = "KVpGWnzPGKvvQ8yeEWmJZ";
    let url = format!("https://cdn.optimizely.com/datafiles/{}.json", sdk_key);
    let response = ureq::get(&url).call().unwrap();
    let content = response.into_string().unwrap();
    let mut json = Json::build(&content).unwrap();
    let shadow_datafile = Datafile::build(&mut json).unwrap();

    // Retrieve information from datafile
    let account_id = shadow_datafile.account_id();
    let revision = shadow_datafile.revision();

    // Generate code
    let q_init = quote! (
        let datafile = optimizely::datafile::Datafile::new(#account_id, #revision);
    );

    // TODO: add features
    // TODO: add rollouts
    // TODO: add experiments
    // TODO: add traffic allocation
    // TODO: add variations
    // TODO: add events
    
    // Wrap all code into a statement that returns the datafile
    quote! (
        {
            #q_init
            datafile
        }
    ).into()
}