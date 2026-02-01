//! Environment validation wrappers for internal core API types.

use crate::common::validation::{ErrorList, Path};
use crate::core::v1::env::{EnvFromSource, EnvVar};
use crate::core::v1::validation::env as v1_env_validation;

pub fn validate_env(vars: &[EnvVar], path: &Path) -> ErrorList {
    v1_env_validation::validate_env(vars, path)
}

pub fn validate_env_from(vars: &[EnvFromSource], path: &Path) -> ErrorList {
    v1_env_validation::validate_env_from(vars, path)
}
