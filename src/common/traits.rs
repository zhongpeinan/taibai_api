//! Kubernetes API Machinery core traits
//!
//! 映射 Kubernetes API 核心语义：静态身份、外部传输态、内部业务态

use crate::common::{ObjectMeta, TypeMeta};

// ============================================================================
// 1. 静态身份 (Resource Schema)
// ============================================================================

/// 定义资源的 Group-Version-Kind-Resource 元信息基础 trait。
///
/// `Meta` 类型参数支持：
/// - `()`: 编译时静态已知的资源（内置资源）
/// - 动态类型: CRD 等运行时确定的资源
pub trait GVKRMeta: Send + Sync {
    /// 元信息类型，用于支持静态或动态资源定义
    type MetaType: Clone + Send + Sync + Default;

    /// API Group (e.g., "" for core, "apps" for apps/v1)
    fn group(meta: &Self::MetaType) -> &str;

    /// API Version (e.g., "v1", "v1beta1")
    fn version(meta: &Self::MetaType) -> &str;

    /// Kind (e.g., "Pod", "Deployment")
    fn kind(meta: &Self::MetaType) -> &str;

    /// Resource 名称 (e.g., "pods", "deployments")
    fn resource(meta: &Self::MetaType) -> &str;
}

/// 定义资源的 Group-Version-Kind-Resource 元信息。
///
/// `Meta` 类型参数支持：
/// - `()`: 编译时静态已知的资源（内置资源）
/// - 动态类型: CRD 等运行时确定的资源
pub trait ResourceSchema: Send + Sync {
    /// 元信息类型，用于支持静态或动态资源定义
    type Meta: Clone + Send + Sync + Default;

    /// API Group (e.g., "" for core, "apps" for apps/v1)
    fn group(meta: &Self::Meta) -> &str;

    /// API Version (e.g., "v1", "v1beta1")
    fn version(meta: &Self::Meta) -> &str;

    /// Kind (e.g., "Pod", "Deployment")
    fn kind(meta: &Self::Meta) -> &str;

    /// Resource 名称 (e.g., "pods", "deployments")
    fn resource(meta: &Self::Meta) -> &str;

    /// 便捷方法：使用默认 Meta 获取 group
    ///
    /// 默认实现要求返回静态字符串。对于动态资源，需要覆盖此方法。
    fn group_static() -> &'static str
    where
        Self::Meta: Default,
    {
        // 注意：这需要实现者返回静态字符串
        // 对于静态资源（Meta = ()），这应该是常量
        panic!(
            "group_static() must be implemented for {} to return static string",
            std::any::type_name::<Self>()
        )
    }

    /// 便捷方法：使用默认 Meta 获取 version
    ///
    /// 默认实现要求返回静态字符串。对于动态资源，需要覆盖此方法。
    fn version_static() -> &'static str
    where
        Self::Meta: Default,
    {
        panic!(
            "version_static() must be implemented for {} to return static string",
            std::any::type_name::<Self>()
        )
    }

    /// 便捷方法：使用默认 Meta 获取 kind
    ///
    /// 默认实现要求返回静态字符串。对于动态资源，需要覆盖此方法。
    fn kind_static() -> &'static str
    where
        Self::Meta: Default,
    {
        panic!(
            "kind_static() must be implemented for {} to return static string",
            std::any::type_name::<Self>()
        )
    }

    /// 便捷方法：使用默认 Meta 获取 resource
    ///
    /// 默认实现要求返回静态字符串。对于动态资源，需要覆盖此方法。
    fn resource_static() -> &'static str
    where
        Self::Meta: Default,
    {
        panic!(
            "resource_static() must be implemented for {} to return static string",
            std::any::type_name::<Self>()
        )
    }
}

/// 为所有 `ResourceSchema` 实现者自动实现 `GVKRMeta`。
impl<T> GVKRMeta for T
where
    T: ResourceSchema,
{
    type MetaType = <T as ResourceSchema>::Meta;

    fn group(meta: &Self::MetaType) -> &str {
        <T as ResourceSchema>::group(meta)
    }

    fn version(meta: &Self::MetaType) -> &str {
        <T as ResourceSchema>::version(meta)
    }

    fn kind(meta: &Self::MetaType) -> &str {
        <T as ResourceSchema>::kind(meta)
    }

    fn resource(meta: &Self::MetaType) -> &str {
        <T as ResourceSchema>::resource(meta)
    }
}

// ============================================================================
// 2. 外部版本 (Versioned Object)
// ============================================================================

/// 只读访问 TypeMeta 字段。
pub trait HasTypeMetaReadOnly {
    /// 获取 TypeMeta 引用
    fn type_meta(&self) -> &TypeMeta;
}

/// 访问 TypeMeta 字段。
pub trait HasTypeMeta {
    /// 获取 TypeMeta 引用
    fn type_meta(&self) -> &TypeMeta;

    /// 获取 TypeMeta 可变引用
    fn type_meta_mut(&mut self) -> &mut TypeMeta;
}

/// 为所有 `HasTypeMeta` 实现者自动实现 `HasTypeMetaReadOnly`。
impl<T> HasTypeMetaReadOnly for T
where
    T: HasTypeMeta,
{
    fn type_meta(&self) -> &TypeMeta {
        <T as HasTypeMeta>::type_meta(self)
    }
}

/// 访问 ObjectMeta 字段。
///
/// 提供 Kubernetes 对象元数据的访问接口。
pub trait HasObjectMeta: Send + Sync {
    /// 获取 ObjectMeta 引用
    ///
    /// 如果 metadata 为 None，返回默认 ObjectMeta 的引用
    fn meta(&self) -> &ObjectMeta;

    /// 获取 ObjectMeta 可变引用
    ///
    /// 如果 metadata 为 None，自动插入默认 ObjectMeta
    fn meta_mut(&mut self) -> &mut ObjectMeta;
}

/// 定义带版本的 Kubernetes 对象（外部 API 版本）。
///
/// 外部版本用于 API 序列化/反序列化，是用户面对的版本。
///
/// **重要**：虽然资源的 `metadata` 字段是 `Option<ObjectMeta>`，
/// 但 Trait 实现应处理 None 情况，提供 Go 风格的零值访问。
pub trait VersionedObject: Send + Sync {
    /// 获取 ObjectMeta 引用
    ///
    /// 如果 metadata 为 None，返回默认 ObjectMeta 的引用
    fn metadata(&self) -> &ObjectMeta;

    /// 获取 ObjectMeta 可变引用
    ///
    /// 如果 metadata 为 None，自动插入默认 ObjectMeta
    fn metadata_mut(&mut self) -> &mut ObjectMeta;
}

/// 为所有 `VersionedObject` 实现者自动实现 `HasObjectMeta`。
impl<T> HasObjectMeta for T
where
    T: VersionedObject,
{
    fn meta(&self) -> &ObjectMeta {
        <T as VersionedObject>::metadata(self)
    }

    fn meta_mut(&mut self) -> &mut ObjectMeta {
        <T as VersionedObject>::metadata_mut(self)
    }
}

/// 填充资源默认值。
///
/// 包括：
/// - TypeMeta (apiVersion, kind)
/// - Spec 字段默认值
/// - 计算字段
pub trait ApplyDefault {
    /// 填充默认值
    fn apply_default(&mut self);
}

// ============================================================================
// 3. 内部版本 (Internal Object)
// ============================================================================

/// 定义无版本的 Kubernetes 对象（内部版本）。
///
/// 内部版本用于业务逻辑处理，不关心具体 API 版本。
///
/// 继承 `HasObjectMeta`，提供对 ObjectMeta 的访问。
pub trait InternalObject: HasObjectMeta {
    /// 获取 ObjectMeta 引用
    ///
    /// 默认实现委托给 `HasObjectMeta`。
    fn metadata(&self) -> &ObjectMeta {
        <Self as HasObjectMeta>::meta(self)
    }

    /// 获取 ObjectMeta 可变引用
    ///
    /// 默认实现委托给 `HasObjectMeta`。
    fn metadata_mut(&mut self) -> &mut ObjectMeta {
        <Self as HasObjectMeta>::meta_mut(self)
    }
}

// ============================================================================
// 4. 版本转换 (Version Conversion)
// ============================================================================

/// 外部版本转内部版本（归一化）。
///
/// 消费外部版本实例，丢弃版本信息，生成内部版本。
pub trait ToInternal<I: InternalObject> {
    /// 转换为内部版本
    fn to_internal(self) -> I;
}

/// 内部版本转外部版本（版本化）。
///
/// 从内部版本构造外部版本，需要调用 `apply_defaults()` 补全 TypeMeta。
pub trait FromInternal<I: InternalObject>: Sized {
    /// 从内部版本构造
    fn from_internal(internal: I) -> Self;
}

// ============================================================================
// 5. 占位实现 (Placeholder Implementations)
// ============================================================================

/// 标记 Trait：表示资源尚未实现版本转换功能。
///
/// 为所有标记为 `UnimplementedConversion` 的类型自动提供：
/// - `ToInternal<I>` 实现（使用 `todo!()`）
/// - `FromInternal<I>` 实现（使用 `todo!()`）
///
/// # 使用方式
///
/// ```ignore
/// // 对于尚未实现转换逻辑的资源，只需一行代码：
/// impl UnimplementedConversion for Pod {}
/// impl UnimplementedConversion for PodList {}
///
/// // 自动获得：
/// // - ToInternal<internal::Pod> for Pod
/// // - FromInternal<internal::Pod> for Pod
/// ```
///
/// # 迁移到真实实现
///
/// 当需要实现真实的转换逻辑时：
/// 1. 移除 `impl UnimplementedConversion for XXX {}`
/// 2. 手动实现 `ToInternal` 和 `FromInternal`
pub trait UnimplementedConversion: Sized {}

/// 为所有实现了 `UnimplementedConversion` 的类型，自动实现 `ToInternal`（使用 `todo!()`）
impl<T, I> ToInternal<I> for T
where
    T: UnimplementedConversion,
    I: InternalObject,
{
    fn to_internal(self) -> I {
        todo!(
            "Version conversion not implemented: {} -> {}",
            std::any::type_name::<Self>(),
            std::any::type_name::<I>()
        )
    }
}

/// 为所有实现了 `UnimplementedConversion` 的类型，自动实现 `FromInternal`（使用 `todo!()`）
impl<T, I> FromInternal<I> for T
where
    T: UnimplementedConversion,
    I: InternalObject,
{
    fn from_internal(_internal: I) -> Self {
        todo!(
            "Version conversion not implemented: {} -> {}",
            std::any::type_name::<I>(),
            std::any::type_name::<Self>()
        )
    }
}

// ============================================================================
// 6. Protobuf 序列化占位实现
// ============================================================================

/// 为类型自动实现 `prost::Message`，所有方法使用 `todo!()`。
///
/// 由于 Rust 的孤儿规则，我们无法使用 blanket implementation 为类型参数实现外部 trait。
/// 因此提供此宏，为每个具体类型生成 `prost::Message` 的占位实现。
///
/// # 使用方式
///
/// ```ignore
/// use crate::common::impl_unimplemented_prost_message;
///
/// // 为 Pod 和 PodList 自动实现 prost::Message（使用 todo!()）
/// impl_unimplemented_prost_message!(Pod);
/// impl_unimplemented_prost_message!(PodList);
/// ```
///
/// # 生成的代码
///
/// 宏会为类型生成如下实现：
/// ```ignore
/// impl prost::Message for Pod {
///     fn encode_raw<B>(&self, _buf: &mut B) where B: BufMut {
///         todo!("Protobuf encoding not implemented for Pod")
///     }
///     // ... 其他方法也都是 todo!()
/// }
/// ```
///
/// # 迁移到真实实现
///
/// 当需要实现真实的 Protobuf 序列化时：
/// 1. 移除宏调用 `impl_unimplemented_prost_message!(XXX);`
/// 2. 使用 `#[derive(prost::Message)]` 或手动实现 `prost::Message`
///
/// # 注意
///
/// 当前项目主要使用 serde 进行 JSON 序列化。
/// Protobuf 支持是可选的，仅在需要时实现。
#[macro_export]
macro_rules! impl_unimplemented_prost_message {
    ($type:ty) => {
        impl prost::Message for $type {
            fn encode_raw<B>(&self, _buf: &mut B)
            where
                B: prost::bytes::BufMut,
            {
                todo!(
                    "Protobuf encoding not implemented for {}",
                    stringify!($type)
                )
            }

            fn merge_field<B>(
                &mut self,
                _tag: u32,
                _wire_type: prost::encoding::WireType,
                _buf: &mut B,
                _ctx: prost::encoding::DecodeContext,
            ) -> Result<(), prost::DecodeError>
            where
                B: prost::bytes::Buf,
            {
                todo!(
                    "Protobuf decoding not implemented for {}",
                    stringify!($type)
                )
            }

            fn encoded_len(&self) -> usize {
                todo!(
                    "Protobuf encoded_len not implemented for {}",
                    stringify!($type)
                )
            }

            fn clear(&mut self) {
                todo!("Protobuf clear not implemented for {}", stringify!($type))
            }
        }
    };
}

// ============================================================================
// Trait Implementation Macros
// ============================================================================

/// 为外部版本实现 `VersionedObject` trait。
///
/// 外部版本的 `metadata` 字段是 `Option<ObjectMeta>`，此宏自动处理 None 情况。
///
/// # 使用方式
///
/// ```ignore
/// use crate::impl_versioned_object;
///
/// impl_versioned_object!(Pod);
/// impl_versioned_object!(PodList);
/// ```
#[macro_export]
macro_rules! impl_versioned_object {
    ($type:ty) => {
        impl $crate::common::traits::VersionedObject for $type {
            fn metadata(&self) -> &$crate::common::ObjectMeta {
                use std::sync::OnceLock;
                self.metadata.as_ref().unwrap_or_else(|| {
                    static DEFAULT: OnceLock<$crate::common::ObjectMeta> = OnceLock::new();
                    DEFAULT.get_or_init($crate::common::ObjectMeta::default)
                })
            }

            fn metadata_mut(&mut self) -> &mut $crate::common::ObjectMeta {
                self.metadata.get_or_insert_with($crate::common::ObjectMeta::default)
            }
        }
    };
}

/// 为内部版本实现 `HasObjectMeta` trait。
///
/// 内部版本的 `metadata` 字段是 `ObjectMeta`（��可选），直接返回引用。
///
/// # 使用方式
///
/// ```ignore
/// use crate::impl_has_object_meta;
///
/// impl_has_object_meta!(Pod);
/// impl_has_object_meta!(PodList);
/// ```
#[macro_export]
macro_rules! impl_has_object_meta {
    ($type:ty) => {
        impl $crate::common::traits::HasObjectMeta for $type {
            fn meta(&self) -> &$crate::common::ObjectMeta {
                &self.metadata
            }

            fn meta_mut(&mut self) -> &mut $crate::common::ObjectMeta {
                &mut self.metadata
            }
        }
    };
}
