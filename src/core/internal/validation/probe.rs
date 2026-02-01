//! Probe validation wrappers for internal core API types.

use crate::common::validation::{ErrorList, Path};
use crate::core::v1::probe::{Lifecycle, Probe};
use crate::core::v1::validation::probe as v1_probe_validation;

pub fn validate_liveness_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    v1_probe_validation::validate_liveness_probe(probe, grace_period, path)
}

pub fn validate_readiness_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    v1_probe_validation::validate_readiness_probe(probe, grace_period, path)
}

pub fn validate_startup_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    v1_probe_validation::validate_startup_probe(probe, grace_period, path)
}

pub fn validate_lifecycle(
    lifecycle: Option<&Lifecycle>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    v1_probe_validation::validate_lifecycle(lifecycle, grace_period, path)
}
