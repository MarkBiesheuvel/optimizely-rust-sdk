// Imports from Optimizely crate
use optimizely::{client::ClientError, datafile::DatafileError, Client};

// Relative imports of sub modules
use common::{ACCOUNT_ID, FILE_PATH, REVISION};
mod common;

#[test]
fn with_invalid_json() {
    // Empty datafile is invalid
    let json = "";

    // Get error report
    let report = Client::from_string(json).err().unwrap();

    // Verify the client error type
    let client_error = report.downcast_ref::<ClientError>().unwrap();
    assert!(
        matches!(client_error, ClientError::InvalidDatafile),
        "Report did not include ClientError::InvalidDatafile"
    );

    // Verify the json error type
    let datafile_error = report.downcast_ref::<DatafileError>().unwrap();
    assert!(
        matches!(datafile_error, DatafileError::InvalidJson),
        "Report did not include DatafileError::InvalidJson"
    );
}

#[test]
fn with_missing_properties() {
    // Valid JSON, but missing properties
    let json = r#"
    {
        "accountId": "21537940595"
    }"#;

    // Get error report
    let report = Client::from_string(json).err().unwrap();

    // Verify the client error type
    let client_error = report.downcast_ref::<ClientError>().unwrap();
    assert!(
        matches!(client_error, ClientError::InvalidDatafile),
        "Report did not include ClientError::InvalidDatafile"
    );

    // Verify the datafile error type
    let datafile_error = report.downcast_ref::<DatafileError>().unwrap();
    assert!(
        matches!(datafile_error, DatafileError::InvalidJson),
        "Report did not include DatafileError::KeyNotFound"
    );
}

#[test]
fn with_invalid_array_properties() {
    // Valid JSON, but rollouts, experiments, and featureFlags should be an array
    let json = r#"
    {
        "accountId": "21537940595",
        "revision": "73",
        "rollouts": null,
        "experiments": null,
        "featureFlags": null,
        "events": null
    }"#;

    // Get error report
    let report = Client::from_string(json.into()).err().unwrap();

    // Verify the client error type
    let client_error = report.downcast_ref::<ClientError>().unwrap();
    assert!(
        matches!(client_error, ClientError::InvalidDatafile),
        "Report did not include ClientError::InvalidDatafile"
    );

    // Verify the datafile error type
    let datafile_error = report.downcast_ref::<DatafileError>().unwrap();
    assert!(
        matches!(datafile_error, DatafileError::InvalidJson),
        "Report did not include DatafileError::InvalidType"
    );
}

#[test]
#[cfg(feature = "online")]
fn with_sdk_key() {
    let client = Client::from_sdk_key(common::SDK_KEY)
        .expect("sdk key should work")
        .initialize();

    // Check account id property on client
    assert_eq!(client.datafile().account_id(), ACCOUNT_ID);

    // Check revision property on client
    // NOTE: the online datafile might have been updated
    assert!(client.datafile().revision() >= REVISION);
}

#[test]
fn with_fixed_datafile() {
    let client = Client::from_local_datafile(FILE_PATH)
        .expect("local datafile should work")
        .initialize();

    // Check account id property on client
    assert_eq!(client.datafile().account_id(), ACCOUNT_ID);

    // Check revision property on client
    assert_eq!(client.datafile().revision(), REVISION);
}
