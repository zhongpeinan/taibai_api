use super::{
    ConditionStatus, ServerStorageVersion, StorageVersion, StorageVersionCondition,
    StorageVersionConditionType, StorageVersionList, StorageVersionStatus,
};
use crate::apiserverinternal::internal;
use crate::common::{
    ApplyDefault, FromInternal, ListMeta, ObjectMeta, Timestamp, ToInternal, TypeMeta,
};

#[test]
fn storage_version_apply_default_sets_type_meta() {
    let mut value = StorageVersion::default();
    value.apply_default();
    assert_eq!(
        value.type_meta.api_version,
        "apiserverinternal.k8s.io/v1alpha1"
    );
    assert_eq!(value.type_meta.kind, "StorageVersion");
}

#[test]
fn storage_version_apply_default_preserves_existing_type_meta() {
    let mut value = StorageVersion {
        type_meta: TypeMeta {
            api_version: "custom/v1".to_string(),
            kind: "CustomKind".to_string(),
        },
        ..StorageVersion::default()
    };
    value.apply_default();
    assert_eq!(value.type_meta.api_version, "custom/v1");
    assert_eq!(value.type_meta.kind, "CustomKind");
}

#[test]
fn storage_version_list_apply_default_sets_type_meta() {
    let mut value = StorageVersionList::default();
    value.apply_default();
    assert_eq!(
        value.type_meta.api_version,
        "apiserverinternal.k8s.io/v1alpha1"
    );
    assert_eq!(value.type_meta.kind, "StorageVersionList");
}

#[test]
fn storage_version_round_trip_conversion() {
    let mut value = StorageVersion {
        metadata: Some(ObjectMeta {
            name: Some("apps.deployments".to_string()),
            ..ObjectMeta::default()
        }),
        status: StorageVersionStatus {
            storage_versions: vec![ServerStorageVersion {
                api_server_id: "server-1".to_string(),
                encoding_version: "v1".to_string(),
                decodable_versions: vec!["v1".to_string(), "v1beta1".to_string()],
                served_versions: vec!["v1".to_string()],
            }],
            common_encoding_version: Some("v1".to_string()),
            conditions: vec![StorageVersionCondition {
                type_: StorageVersionConditionType::from(
                    StorageVersionConditionType::ALL_ENCODING_VERSIONS_EQUAL,
                ),
                status: ConditionStatus::from(ConditionStatus::TRUE),
                observed_generation: 1,
                last_transition_time: Timestamp::zero(),
                reason: "AllEqual".to_string(),
                message: "All servers agree".to_string(),
            }],
        },
        ..StorageVersion::default()
    };
    value.apply_default();

    let internal = value.clone().to_internal();
    let round_trip = StorageVersion::from_internal(internal);

    assert_eq!(round_trip, value);
}

#[test]
fn storage_version_list_round_trip_conversion() {
    let mut value = StorageVersionList {
        metadata: Some(ListMeta {
            resource_version: Some("10".to_string()),
            ..ListMeta::default()
        }),
        items: vec![{
            let mut item = StorageVersion {
                metadata: Some(ObjectMeta {
                    name: Some("apps.deployments".to_string()),
                    ..ObjectMeta::default()
                }),
                ..StorageVersion::default()
            };
            item.apply_default();
            item
        }],
        ..StorageVersionList::default()
    };
    value.apply_default();

    let internal: internal::StorageVersionList = value.clone().into();
    let round_trip: StorageVersionList = internal.into();

    assert_eq!(round_trip, value);
}
