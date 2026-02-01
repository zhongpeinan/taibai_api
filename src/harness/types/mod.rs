//! Type registration dispatch.

pub mod apps_v1;
pub mod batch_v1;
pub mod core_v1;

use super::registry::Registry;

/// Register all known GVK handlers.
pub fn register_all(registry: &mut Registry) {
    core_v1::register(registry);
    apps_v1::register(registry);
    batch_v1::register(registry);
}
