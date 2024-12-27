// Imports from Optimizely crate
use optimizely::user_attributes;

// Relative imports of sub modules
use common::setup;
mod common;

#[test]
fn user_context_set_attribute() {
    let ctx = setup();

    // Create user context without attributes
    let mut user_context = ctx.client.create_user_context("user123");

    // Override attributes on existing user context
    user_context.set_attribute("is_employee", "true");
    user_context.set_attribute("app_version", "1.3.2");

    // Attributes should be equal to expected
    assert_eq!(user_context.attributes().get("is_employee").unwrap(), "true");
    assert_eq!(user_context.attributes().get("app_version").unwrap(), "1.3.2");
}

#[test]
fn user_context_with_attributes() {
    let ctx = setup();

    // Create user context with given attributes
    let user_context = ctx.client.create_user_context_with_attributes(
        "user123",
        user_attributes! {
            "is_employee" => "true",
            "app_version" => "1.3.2",
        },
    );

    // Attributes should be equal to expected
    assert_eq!(user_context.attributes().get("is_employee").unwrap(), "true");
    assert_eq!(user_context.attributes().get("app_version").unwrap(), "1.3.2");
}

#[test]
#[cfg(feature = "online")]
fn user_context_track_event() {
    let ctx = setup();

    // Create user context with given attributes
    let user_context = ctx.client.create_user_context("user123");

    // Send a conversion event
    user_context.track_event("purchase");

    // Assert that exactly one event is dispatched
    assert_eq!(ctx.conversions.len(), 1);
}
