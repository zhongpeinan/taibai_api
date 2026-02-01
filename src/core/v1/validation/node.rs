//! Node validation for Kubernetes core/v1 API.

use crate::common::ToInternal;
use crate::common::validation::ErrorList;
use crate::core::internal::validation::node as internal_node_validation;
use crate::core::v1::node::Node;

/// Validates a Node.
pub fn validate_node(node: &Node) -> ErrorList {
    let internal_node = node.clone().to_internal();
    internal_node_validation::validate_node(&internal_node)
}

/// Validates Node update.
pub fn validate_node_update(new: &Node, old: &Node) -> ErrorList {
    let internal_new = new.clone().to_internal();
    let internal_old = old.clone().to_internal();
    internal_node_validation::validate_node_update(&internal_new, &internal_old)
}
