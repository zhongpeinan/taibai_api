//! TestAPIGroup API types
//!
//! This module contains the testapigroup API types used in upstream Kubernetes tests.

pub mod internal;
pub mod v1;

pub use v1::{
    Carp, CarpCondition, CarpConditionType, CarpInfo, CarpList, CarpPhase, CarpSpec, CarpStatus,
    ConditionStatus, RestartPolicy,
};
