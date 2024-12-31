// External imports
use std::error::Error;

// Relative imports of sub modules
use common::setup;
mod common;

#[test]
fn user_context_set_attribute() -> Result<(), Box<dyn Error>> {
    let ctx = setup()?;

    // Create user context without attributes
    let mut user_context = ctx.client.create_user_context("user123");

    // Override attributes on existing user context
    user_context.set_attribute("is_employee", "true");
    user_context.set_attribute("app_version", "1.3.2");

    // Retrieve attributes again
    let attributes = user_context.user_attributes();

    // Attributes should be equal to expected
    assert_eq!(attributes.len(), 2);
    assert!(attributes
        .iter()
        .any(|attribute| attribute.id() == "23328260042"));

    Ok(())
}

#[test]
#[cfg(feature = "online")]
fn user_context_track_event() -> Result<(), Box<dyn Error>> {
    let ctx = setup()?;

    // Create user context with given attributes
    let user_context = ctx.client.create_user_context("user123");

    // Send a conversion event
    user_context.track_event("purchase");

    // Assert that exactly one event is dispatched
    assert_eq!(ctx.conversions.len(), 1);

    Ok(())
}
