//! Discovery v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- discovery/v1/EndpointSlice ----
    register_type::<
        crate::discovery::v1::EndpointSlice,
        crate::discovery::internal::EndpointSlice,
        _,
    >(
        registry,
        "discovery/v1/EndpointSlice",
        crate::discovery::v1::validation::validate_endpoint_slice,
    );
}
