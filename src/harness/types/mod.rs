//! Type registration dispatch.

pub mod apps_v1;
pub mod batch_v1;
pub mod coordination_v1;
pub mod core_v1;
pub mod discovery_v1;
pub mod networking_v1;
pub mod policy_v1;
pub mod rbac_v1;
pub mod storage_v1;

use super::registry::Registry;

/// Register all known GVK handlers.
pub fn register_all(registry: &mut Registry) {
    core_v1::register(registry);
    apps_v1::register(registry);
    batch_v1::register(registry);
    coordination_v1::register(registry);
    discovery_v1::register(registry);
    networking_v1::register(registry);
    policy_v1::register(registry);
    rbac_v1::register(registry);
    storage_v1::register(registry);
}
