//! GVK-based type registry for the test harness.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::error::HarnessError;
use super::{ConversionResult, DefaultResult, ValidationResult};

/// Handler closures for a single GVK.
pub struct TypeHandler {
    pub apply_defaults: Box<dyn Fn(&str) -> Result<DefaultResult, HarnessError> + Send + Sync>,
    pub convert_roundtrip:
        Option<Box<dyn Fn(&str) -> Result<ConversionResult, HarnessError> + Send + Sync>>,
    pub validate: Option<Box<dyn Fn(&str) -> Result<ValidationResult, HarnessError> + Send + Sync>>,
}

/// Registry maps GVK strings to their handlers.
pub struct Registry {
    handlers: HashMap<String, TypeHandler>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, gvk: &str, handler: TypeHandler) {
        self.handlers.insert(gvk.to_string(), handler);
    }

    pub fn get(&self, gvk: &str) -> Option<&TypeHandler> {
        self.handlers.get(gvk)
    }

    pub fn gvks(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.handlers.keys().cloned().collect();
        keys.sort();
        keys
    }
}

static REGISTRY: OnceLock<Registry> = OnceLock::new();

pub fn global_registry() -> &'static Registry {
    REGISTRY.get_or_init(|| {
        let mut registry = Registry::new();
        super::types::register_all(&mut registry);
        registry
    })
}
