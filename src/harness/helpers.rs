//! Common helpers and generic registration functions for the test harness.

use crate::common::validation::{ErrorList, Path};
use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::harness::error::{FieldError, HarnessError};
use crate::harness::registry::{Registry, TypeHandler};
use crate::harness::{ConversionResult, DefaultResult, ValidationResult};
use serde::{Serialize, de::DeserializeOwned};

// ============================================================================
// 公共辅助函数
// ============================================================================

pub fn parse_json<T: DeserializeOwned>(json: &str) -> Result<T, HarnessError> {
    serde_json::from_str(json).map_err(|e| HarnessError::JsonParse {
        message: e.to_string(),
    })
}

pub fn to_value<T: Serialize>(obj: &T) -> Result<serde_json::Value, HarnessError> {
    serde_json::to_value(obj).map_err(|e| HarnessError::Internal {
        message: e.to_string(),
    })
}

pub fn errors_to_field_errors(errors: &ErrorList) -> Vec<FieldError> {
    errors.errors.iter().map(FieldError::from).collect()
}

// ============================================================================
// 泛型 Handler 生成函数
// ============================================================================

/// 创建 apply_defaults 处理器
pub fn make_defaults_handler<T>(
    gvk: &'static str,
) -> Box<dyn Fn(&str) -> Result<DefaultResult, HarnessError> + Send + Sync>
where
    T: DeserializeOwned + Serialize + ApplyDefault + 'static,
{
    Box::new(move |json| {
        let mut obj: T = parse_json(json)?;
        obj.apply_default();
        Ok(DefaultResult {
            gvk: gvk.to_string(),
            result: to_value(&obj)?,
            defaults_applied: true,
        })
    })
}

/// 创建 convert_roundtrip 处理器
pub fn make_conversion_handler<V1, I>(
    gvk: &'static str,
) -> Box<dyn Fn(&str) -> Result<ConversionResult, HarnessError> + Send + Sync>
where
    V1: DeserializeOwned + Serialize + ToInternal<I> + FromInternal<I> + 'static,
    I: 'static,
{
    Box::new(move |json| {
        let obj: V1 = parse_json(json)?;
        let original = to_value(&obj)?;
        let internal: I = obj.to_internal();
        let roundtrip_obj = V1::from_internal(internal);
        let roundtrip = to_value(&roundtrip_obj)?;
        Ok(ConversionResult {
            gvk: gvk.to_string(),
            original,
            roundtrip,
            success: true,
        })
    })
}

/// 创建 validate 处理器 (无 path 参数)
pub fn make_validate_handler<T, F>(
    gvk: &'static str,
    validate_fn: F,
) -> Box<dyn Fn(&str) -> Result<ValidationResult, HarnessError> + Send + Sync>
where
    T: DeserializeOwned + 'static,
    F: Fn(&T) -> ErrorList + Send + Sync + 'static,
{
    Box::new(move |json| {
        let obj: T = parse_json(json)?;
        let errors = validate_fn(&obj);
        Ok(ValidationResult {
            gvk: gvk.to_string(),
            valid: errors.is_empty(),
            errors: errors_to_field_errors(&errors),
        })
    })
}

/// 创建 validate 处理器 (带 path 参数)
pub fn make_validate_handler_with_path<T, F>(
    gvk: &'static str,
    validate_fn: F,
) -> Box<dyn Fn(&str) -> Result<ValidationResult, HarnessError> + Send + Sync>
where
    T: DeserializeOwned + 'static,
    F: Fn(&T, &Path) -> ErrorList + Send + Sync + 'static,
{
    Box::new(move |json| {
        let obj: T = parse_json(json)?;
        let root = Path::new("");
        let errors = validate_fn(&obj, &root);
        Ok(ValidationResult {
            gvk: gvk.to_string(),
            valid: errors.is_empty(),
            errors: errors_to_field_errors(&errors),
        })
    })
}

// ============================================================================
// 便捷注册函数
// ============================================================================

/// 注册类型 - 带验证(无path)
pub fn register_type<V1, I, F>(registry: &mut Registry, gvk: &'static str, validate_fn: F)
where
    V1: DeserializeOwned + Serialize + ApplyDefault + ToInternal<I> + FromInternal<I> + 'static,
    I: 'static,
    F: Fn(&V1) -> ErrorList + Send + Sync + 'static,
{
    registry.register(
        gvk,
        TypeHandler {
            apply_defaults: make_defaults_handler::<V1>(gvk),
            convert_roundtrip: Some(make_conversion_handler::<V1, I>(gvk)),
            validate: Some(make_validate_handler(gvk, validate_fn)),
        },
    );
}

/// 注册类型 - 带验证(带path)
pub fn register_type_with_path<V1, I, F>(registry: &mut Registry, gvk: &'static str, validate_fn: F)
where
    V1: DeserializeOwned + Serialize + ApplyDefault + ToInternal<I> + FromInternal<I> + 'static,
    I: 'static,
    F: Fn(&V1, &Path) -> ErrorList + Send + Sync + 'static,
{
    registry.register(
        gvk,
        TypeHandler {
            apply_defaults: make_defaults_handler::<V1>(gvk),
            convert_roundtrip: Some(make_conversion_handler::<V1, I>(gvk)),
            validate: Some(make_validate_handler_with_path(gvk, validate_fn)),
        },
    );
}

/// 注册类型 - 无验证
pub fn register_type_no_validate<V1, I>(registry: &mut Registry, gvk: &'static str)
where
    V1: DeserializeOwned + Serialize + ApplyDefault + ToInternal<I> + FromInternal<I> + 'static,
    I: 'static,
{
    registry.register(
        gvk,
        TypeHandler {
            apply_defaults: make_defaults_handler::<V1>(gvk),
            convert_roundtrip: Some(make_conversion_handler::<V1, I>(gvk)),
            validate: None,
        },
    );
}
