# *Unofficial* Optimizely Rust SDK

This repository houses an *unofficial* Rust SDK for use with Optimizely Feature Experimentation.

Optimizely Feature Experimentation is an A/B testing and feature management tool for product development teams that enables you to experiment at every step. Using Optimizely Feature Experimentation allows for every feature on your roadmap to be an opportunity to discover hidden insights. Learn more at [optimizely.com](https://www.optimizely.com/products/feature-experimentation/), or see the [developer documentation](https://docs.developers.optimizely.com/feature-experimentation/docs/introduction).

## What does *unofficial* SDK mean?

This is a hobby project of a single employee at Optimizely. In other words, there is not a dedicated development team working on the SDK.

The SDK does not include all of the features that is included by the official SDKs, such as the JavaScript SDK or Python SDK. A list of supported features is shown below.

The Optimizely support team does not have experience with Rust and will not be able to help with any issues. If you run into a problem, you can create an issue via Github. There is no SLA for Github issues.

## Code example

This example shows how to initiate an SDK client and bucket a single user for a single feature flag.

```rust
use optimizely::{event_api::BatchedEventDispatcher, Client};

// Initialize Optimizely client using local datafile
let file_path = "../datafiles/sandbox.json";
let optimizely_client = Client::from_local_datafile(file_path)?
    .with_event_dispatcher(BatchedEventDispatcher::default())
    .initialize();

// Create user context for current user
let user_id = "123abc789xyz";
let user_context = optimizely_client.create_user_context(user_id);

// Get decision for the Buy Button feature flag
let feature_flag = "buy_button";
let decision = user_context.decide(feature_flag);

// Return Ok for doc-tests
Ok::<(), Box<dyn std::error::Error>>(())
```

## Supported features

All *checked* features are currently supported in the Rust SDK.
All *unchecked* features are not supported in the Rust SDK, but are supported by all official SDKs.

- [x] Initialize client from local datafile
- [x] Initialize client from SDK key
- [ ] Periodically poll latest datafile
- [x] Event dispatcher (synchronous)
- [x] Event dispatcher (batched)
- [ ] Logger
- [ ] Notification listeners
- [X] Decide option (DisableDecisionEvent)
- [ ] Decide options (others)
- [X] Creating an user context
- [X] Decide method consistent with other SDKs
- [ ] Evaluating audience conditions
- [ ] Variation variables
- [ ] Forced decision methods
- [ ] Mutual exclusion groups
