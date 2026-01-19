// External imports
use std::error::Error;

// Imports from Optimizely crate
use optimizely::{
    error::{ClientError, DatafileError},
    Client,
};

// Relative imports of sub modules
use common::{ACCOUNT_ID, FILE_PATH, REVISION};
mod common;

#[test]
fn with_invalid_json() -> Result<(), Box<dyn Error>> {
    // Empty datafile is invalid
    let json = "";

    // Get error report
    let report = Client::from_string(json)
        .err()
        .ok_or("Unexpected Result::Ok")?;

    // Verify the client error type
    let client_error = report
        .downcast_ref::<ClientError>()
        .ok_or("ClientError not found")?;
    assert_eq!(client_error, &ClientError::InvalidDatafile);

    // Verify the json error type
    let datafile_error = report
        .downcast_ref::<DatafileError>()
        .ok_or("DatafileError not found")?;
    assert_eq!(datafile_error, &DatafileError::InvalidJson);

    Ok(())
}

#[test]
fn with_missing_properties() -> Result<(), Box<dyn Error>> {
    // Valid JSON, but missing properties
    let json = r#"
    {
        "accountId": "21537940595"
    }"#;

    // Get error report
    let report = Client::from_string(json)
        .err()
        .ok_or("Unexpected Result::Ok")?;

    // Verify the client error type
    let client_error = report
        .downcast_ref::<ClientError>()
        .ok_or("ClientError not found")?;
    assert_eq!(client_error, &ClientError::InvalidDatafile);

    // Verify the json error type
    let datafile_error = report
        .downcast_ref::<DatafileError>()
        .ok_or("DatafileError not found")?;
    assert_eq!(datafile_error, &DatafileError::InvalidJson);

    Ok(())
}

#[test]
fn with_invalid_array_properties() -> Result<(), Box<dyn Error>> {
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
    let report = Client::from_string(json)
        .err()
        .ok_or("Unexpected Result::Ok")?;

    // Verify the client error type
    let client_error = report
        .downcast_ref::<ClientError>()
        .ok_or("ClientError not found")?;
    assert_eq!(client_error, &ClientError::InvalidDatafile);

    // Verify the json error type
    let datafile_error = report
        .downcast_ref::<DatafileError>()
        .ok_or("DatafileError not found")?;
    assert_eq!(datafile_error, &DatafileError::InvalidJson);

    Ok(())
}

#[test]
#[cfg(feature = "online")]
fn with_sdk_key() -> Result<(), Box<dyn Error>> {
    let client = Client::from_sdk_key(common::SDK_KEY)?.initialize();

    // Check account id property on client
    assert_eq!(client.datafile().account_id(), ACCOUNT_ID);

    // Check revision property on client
    // NOTE: the online datafile might have been updated
    assert!(client.datafile().revision() >= REVISION);

    Ok(())
}

#[test]
fn with_fixed_datafile() -> Result<(), Box<dyn Error>> {
    let client = Client::from_local_datafile(FILE_PATH)?.initialize();

    // Check account id property on client
    assert_eq!(client.datafile().account_id(), ACCOUNT_ID);

    // Check revision property on client
    assert!(client.datafile().revision() >= REVISION);

    Ok(())
}
