//! Coordination v1 type registrations for the test harness.

use crate::harness::helpers::register_type;
use crate::harness::registry::Registry;

pub fn register(registry: &mut Registry) {
    // ---- coordination/v1/Lease ----
    register_type::<crate::coordination::v1::Lease, crate::coordination::internal::Lease, _>(
        registry,
        "coordination/v1/Lease",
        crate::coordination::v1::validation::validate_lease,
    );
}
