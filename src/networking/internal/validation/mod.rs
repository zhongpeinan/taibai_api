//! Validation for Kubernetes Networking API types (internal implementation)
//!
//! Ported from k8s.io/kubernetes/pkg/apis/networking/validation/validation.go

mod helpers;
mod ingress;
mod ingress_class;
mod ip_address;
mod network_policy;
mod service_cidr;

pub use ingress::{
    validate_ingress, validate_ingress_list, validate_ingress_status_update,
    validate_ingress_update,
};
pub use ingress_class::{
    validate_ingress_class, validate_ingress_class_list, validate_ingress_class_update,
};
pub use ip_address::{validate_ip_address, validate_ip_address_list, validate_ip_address_update};
pub use network_policy::{
    validate_network_policy, validate_network_policy_list, validate_network_policy_update,
};
pub use service_cidr::{
    validate_service_cidr, validate_service_cidr_list, validate_service_cidr_status_update,
    validate_service_cidr_update,
};

#[cfg(test)]
mod tests;
