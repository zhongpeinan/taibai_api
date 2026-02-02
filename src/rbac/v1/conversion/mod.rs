//! RBAC v1 conversion implementations
//!
//! Note: RBAC internal types are re-exports of v1 types, so conversion
//! is essentially identity with type_meta reset.

mod cluster_role;
mod cluster_role_binding;
mod role;
mod role_binding;
