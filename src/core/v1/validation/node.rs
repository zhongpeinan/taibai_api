//! Node validation

use crate::common::validation::ErrorList;
use crate::core::v1::Node;

/// Validates a Node
pub fn validate_node(_node: &Node) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}

/// Validates Node update
pub fn validate_node_update(_new: &Node, _old: &Node) -> ErrorList {
    ErrorList::new()
    // TODO: Implement in Phase 6
}
