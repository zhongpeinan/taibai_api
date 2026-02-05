//! Compatibility workarounds for downstream API constraints.
//!
//! This module contains implementations that exist solely for
//! compatibility with downstream code. They are NOT part of the
//! canonical Kubernetes API semantics.
//!
//! # Design Principle
//!
//! Workarounds in this module are:
//! - **Isolated**: Kept separate from core trait definitions
//! - **Documented**: Each workaround references its motivating issue
//! - **Removable**: Can be deleted when downstream constraints are fixed
//!
//! # Contents
//!
//! - [`options_object`]: `HasObjectMeta` impl for Options types (issue #147)

pub mod options_object;

pub use options_object::{EMPTY_OBJECT_META, OptionsObject};
