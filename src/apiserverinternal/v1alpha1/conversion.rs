use crate::apiserverinternal::internal;
use crate::common::{ListMeta, ObjectMeta, TypeMeta};

use super::{
    ConditionStatus, ServerStorageVersion, StorageVersion, StorageVersionCondition,
    StorageVersionConditionType, StorageVersionList, StorageVersionSpec, StorageVersionStatus,
};

fn is_empty_object_meta(meta: &ObjectMeta) -> bool {
    meta.name.is_none()
        && meta.generate_name.is_none()
        && meta.namespace.is_none()
        && meta.uid.is_none()
        && meta.resource_version.is_none()
        && meta.generation.is_none()
        && meta.self_link.is_none()
        && meta.labels.is_empty()
        && meta.annotations.is_empty()
        && meta.owner_references.is_empty()
        && meta.finalizers.is_empty()
        && meta.managed_fields.is_empty()
        && meta.creation_timestamp.is_none()
        && meta.deletion_timestamp.is_none()
        && meta.deletion_grace_period_seconds.is_none()
}

fn is_empty_list_meta(meta: &ListMeta) -> bool {
    meta.continue_.is_none()
        && meta.remaining_item_count.is_none()
        && meta.resource_version.is_none()
        && meta.self_link.is_none()
}

fn object_meta_to_option(meta: ObjectMeta) -> Option<ObjectMeta> {
    if is_empty_object_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

fn list_meta_to_option(meta: ListMeta) -> Option<ListMeta> {
    if is_empty_list_meta(&meta) {
        None
    } else {
        Some(meta)
    }
}

impl From<StorageVersion> for internal::StorageVersion {
    fn from(value: StorageVersion) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata.unwrap_or_default(),
            spec: value.spec.into(),
            status: value.status.into(),
        }
    }
}

impl From<internal::StorageVersion> for StorageVersion {
    fn from(value: internal::StorageVersion) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: object_meta_to_option(value.metadata),
            spec: value.spec.into(),
            status: value.status.into(),
        }
    }
}

impl From<StorageVersionList> for internal::StorageVersionList {
    fn from(value: StorageVersionList) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: value.metadata.unwrap_or_default(),
            items: value.items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<internal::StorageVersionList> for StorageVersionList {
    fn from(value: internal::StorageVersionList) -> Self {
        Self {
            type_meta: TypeMeta::default(),
            metadata: list_meta_to_option(value.metadata),
            items: value.items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<StorageVersionSpec> for internal::StorageVersionSpec {
    fn from(_: StorageVersionSpec) -> Self {
        internal::StorageVersionSpec {}
    }
}

impl From<internal::StorageVersionSpec> for StorageVersionSpec {
    fn from(_: internal::StorageVersionSpec) -> Self {
        StorageVersionSpec {}
    }
}

impl From<StorageVersionStatus> for internal::StorageVersionStatus {
    fn from(value: StorageVersionStatus) -> Self {
        Self {
            storage_versions: value.storage_versions.into_iter().map(Into::into).collect(),
            common_encoding_version: value.common_encoding_version,
            conditions: value.conditions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<internal::StorageVersionStatus> for StorageVersionStatus {
    fn from(value: internal::StorageVersionStatus) -> Self {
        Self {
            storage_versions: value.storage_versions.into_iter().map(Into::into).collect(),
            common_encoding_version: value.common_encoding_version,
            conditions: value.conditions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ServerStorageVersion> for internal::ServerStorageVersion {
    fn from(value: ServerStorageVersion) -> Self {
        Self {
            api_server_id: value.api_server_id,
            encoding_version: value.encoding_version,
            decodable_versions: value.decodable_versions,
            served_versions: value.served_versions,
        }
    }
}

impl From<internal::ServerStorageVersion> for ServerStorageVersion {
    fn from(value: internal::ServerStorageVersion) -> Self {
        Self {
            api_server_id: value.api_server_id,
            encoding_version: value.encoding_version,
            decodable_versions: value.decodable_versions,
            served_versions: value.served_versions,
        }
    }
}

impl From<StorageVersionConditionType> for internal::StorageVersionConditionType {
    fn from(value: StorageVersionConditionType) -> Self {
        internal::StorageVersionConditionType(value.0)
    }
}

impl From<internal::StorageVersionConditionType> for StorageVersionConditionType {
    fn from(value: internal::StorageVersionConditionType) -> Self {
        StorageVersionConditionType(value.0)
    }
}

impl From<ConditionStatus> for internal::ConditionStatus {
    fn from(value: ConditionStatus) -> Self {
        internal::ConditionStatus(value.0)
    }
}

impl From<internal::ConditionStatus> for ConditionStatus {
    fn from(value: internal::ConditionStatus) -> Self {
        ConditionStatus(value.0)
    }
}

impl From<StorageVersionCondition> for internal::StorageVersionCondition {
    fn from(value: StorageVersionCondition) -> Self {
        Self {
            type_: value.type_.into(),
            status: value.status.into(),
            observed_generation: value.observed_generation,
            last_transition_time: value.last_transition_time,
            reason: value.reason,
            message: value.message,
        }
    }
}

impl From<internal::StorageVersionCondition> for StorageVersionCondition {
    fn from(value: internal::StorageVersionCondition) -> Self {
        Self {
            type_: value.type_.into(),
            status: value.status.into(),
            observed_generation: value.observed_generation,
            last_transition_time: value.last_transition_time,
            reason: value.reason,
            message: value.message,
        }
    }
}
