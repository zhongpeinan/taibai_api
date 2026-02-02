//! PersistentVolume and PersistentVolumeClaim conversions
//!
//! Includes: PersistentVolume, PersistentVolumeList, PersistentVolumeSpec/Status,
//! PersistentVolumeClaim, PersistentVolumeClaimList, and supporting types.

use crate::common::{ApplyDefault, FromInternal, ToInternal};
use crate::core::internal;
use crate::core::internal::persistent_volume as internal_pv;
use crate::core::v1::affinity;
use crate::core::v1::persistent_volume as v1_pv;
use crate::core::v1::{LocalVolumeSource, PodCondition, TypedLocalObjectReference};
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::helpers::*;

// ============================================================================
// PersistentVolume and PersistentVolumeList
// ============================================================================

impl ToInternal<internal_pv::PersistentVolume> for v1_pv::PersistentVolume {
    fn to_internal(self) -> internal_pv::PersistentVolume {
        internal_pv::PersistentVolume {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()),
            status: self.status.map(|s| s.to_internal()),
        }
    }
}

impl FromInternal<internal_pv::PersistentVolume> for v1_pv::PersistentVolume {
    fn from_internal(value: internal_pv::PersistentVolume) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value.spec.map(v1_pv::PersistentVolumeSpec::from_internal),
            status: value
                .status
                .map(v1_pv::PersistentVolumeStatus::from_internal),
        };

        result
    }
}

impl ToInternal<internal_pv::PersistentVolumeList> for v1_pv::PersistentVolumeList {
    fn to_internal(self) -> internal_pv::PersistentVolumeList {
        internal_pv::PersistentVolumeList {
            metadata: self.metadata,
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeList> for v1_pv::PersistentVolumeList {
    fn from_internal(value: internal_pv::PersistentVolumeList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(v1_pv::PersistentVolume::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// PersistentVolumeSpec and Status
// ============================================================================

impl ToInternal<internal_pv::PersistentVolumeSpec> for v1_pv::PersistentVolumeSpec {
    fn to_internal(self) -> internal_pv::PersistentVolumeSpec {
        internal_pv::PersistentVolumeSpec {
            persistent_volume_source: self
                .persistent_volume_source
                .map(persistent_volume_source_to_internal)
                .unwrap_or_default(),
            capacity: self.capacity,
            source: None,
            access_modes: self.access_modes,
            claim_ref: self.claim_ref,
            persistent_volume_reclaim_policy: self
                .persistent_volume_reclaim_policy
                .unwrap_or_default(),
            storage_class_name: self.storage_class_name.unwrap_or_default(),
            mount_options: self.mount_options,
            volume_mode: self.volume_mode,
            node_affinity: self.node_affinity.map(|n| n.to_internal()),
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeSpec> for v1_pv::PersistentVolumeSpec {
    fn from_internal(value: internal_pv::PersistentVolumeSpec) -> Self {
        let persistent_volume_source =
            persistent_volume_source_from_internal(value.persistent_volume_source);
        Self {
            capacity: value.capacity,
            persistent_volume_source: if is_empty_persistent_volume_source(
                &persistent_volume_source,
            ) {
                None
            } else {
                Some(persistent_volume_source)
            },
            access_modes: value.access_modes,
            claim_ref: value.claim_ref,
            persistent_volume_reclaim_policy: if value.persistent_volume_reclaim_policy.is_empty() {
                None
            } else {
                Some(value.persistent_volume_reclaim_policy)
            },
            storage_class_name: if value.storage_class_name.is_empty() {
                None
            } else {
                Some(value.storage_class_name)
            },
            mount_options: value.mount_options,
            volume_mode: value.volume_mode,
            node_affinity: value
                .node_affinity
                .map(v1_pv::VolumeNodeAffinity::from_internal),
            volume_attributes_class_name: None,
        }
    }
}

impl ToInternal<internal_pv::PersistentVolumeStatus> for v1_pv::PersistentVolumeStatus {
    fn to_internal(self) -> internal_pv::PersistentVolumeStatus {
        internal_pv::PersistentVolumeStatus {
            phase: self.phase.unwrap_or_default(),
            reason: self.reason.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
            last_phase_transition_time: self.last_phase_transition_time,
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeStatus> for v1_pv::PersistentVolumeStatus {
    fn from_internal(value: internal_pv::PersistentVolumeStatus) -> Self {
        Self {
            phase: if value.phase.is_empty() {
                None
            } else {
                Some(value.phase)
            },
            message: if value.message.is_empty() {
                None
            } else {
                Some(value.message)
            },
            reason: if value.reason.is_empty() {
                None
            } else {
                Some(value.reason)
            },
            last_phase_transition_time: value.last_phase_transition_time,
        }
    }
}

impl ToInternal<internal_pv::VolumeNodeAffinity> for v1_pv::VolumeNodeAffinity {
    fn to_internal(self) -> internal_pv::VolumeNodeAffinity {
        internal_pv::VolumeNodeAffinity {
            required: self.required.map(|r| r.to_internal()),
        }
    }
}

impl FromInternal<internal_pv::VolumeNodeAffinity> for v1_pv::VolumeNodeAffinity {
    fn from_internal(value: internal_pv::VolumeNodeAffinity) -> Self {
        Self {
            required: value.required.map(affinity::NodeSelector::from_internal),
        }
    }
}

// ============================================================================
// PersistentVolumeClaim and list
// ============================================================================

impl ToInternal<internal_pv::PersistentVolumeClaim> for v1_pv::PersistentVolumeClaim {
    fn to_internal(self) -> internal_pv::PersistentVolumeClaim {
        internal_pv::PersistentVolumeClaim {
            metadata: option_object_meta_to_meta(self.metadata),
            spec: self.spec.map(|s| s.to_internal()),
            status: self.status.map(|s| s.to_internal()),
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeClaim> for v1_pv::PersistentVolumeClaim {
    fn from_internal(value: internal_pv::PersistentVolumeClaim) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: meta_to_option_object_meta(value.metadata),
            spec: value
                .spec
                .map(v1_pv::PersistentVolumeClaimSpec::from_internal),
            status: value
                .status
                .map(v1_pv::PersistentVolumeClaimStatus::from_internal),
        };

        result
    }
}

impl ToInternal<internal_pv::PersistentVolumeClaimList> for v1_pv::PersistentVolumeClaimList {
    fn to_internal(self) -> internal_pv::PersistentVolumeClaimList {
        internal_pv::PersistentVolumeClaimList {
            metadata: self.metadata,
            items: self.items.into_iter().map(|i| i.to_internal()).collect(),
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeClaimList> for v1_pv::PersistentVolumeClaimList {
    fn from_internal(value: internal_pv::PersistentVolumeClaimList) -> Self {
        let mut result = Self {
            type_meta: crate::common::TypeMeta::default(),
            metadata: value.metadata,
            items: value
                .items
                .into_iter()
                .map(v1_pv::PersistentVolumeClaim::from_internal)
                .collect(),
        };

        result
    }
}

// ============================================================================
// PersistentVolumeClaimSpec and Status
// ============================================================================

impl ToInternal<internal_pv::PersistentVolumeClaimSpec> for v1_pv::PersistentVolumeClaimSpec {
    fn to_internal(self) -> internal_pv::PersistentVolumeClaimSpec {
        internal_pv::PersistentVolumeClaimSpec {
            access_modes: self.access_modes,
            selector: self.selector.map(label_selector_to_internal),
            resources: self.resources.map(volume_resources_to_internal),
            volume_name: self.volume_name.unwrap_or_default(),
            storage_class_name: self.storage_class_name,
            volume_mode: self.volume_mode,
            data_source: self.data_source,
            data_source_ref: self.data_source_ref.map(typed_object_reference_to_local),
            volume_attributes_class_name: self.volume_attributes_class_name,
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeClaimSpec> for v1_pv::PersistentVolumeClaimSpec {
    fn from_internal(value: internal_pv::PersistentVolumeClaimSpec) -> Self {
        Self {
            access_modes: value.access_modes,
            selector: value.selector.map(label_selector_from_internal),
            resources: value.resources.map(volume_resources_from_internal),
            volume_name: if value.volume_name.is_empty() {
                None
            } else {
                Some(value.volume_name)
            },
            storage_class_name: value.storage_class_name,
            volume_mode: value.volume_mode,
            data_source: value.data_source,
            data_source_ref: value.data_source_ref.map(typed_local_to_object_reference),
            volume_attributes_class_name: value.volume_attributes_class_name,
        }
    }
}

impl ToInternal<internal_pv::PersistentVolumeClaimStatus> for v1_pv::PersistentVolumeClaimStatus {
    fn to_internal(self) -> internal_pv::PersistentVolumeClaimStatus {
        internal_pv::PersistentVolumeClaimStatus {
            phase: self.phase.unwrap_or_default(),
            access_modes: self.access_modes,
            capacity: self.capacity,
            conditions: self
                .conditions
                .into_iter()
                .map(pvc_condition_to_pod_condition)
                .collect(),
        }
    }
}

impl FromInternal<internal_pv::PersistentVolumeClaimStatus> for v1_pv::PersistentVolumeClaimStatus {
    fn from_internal(value: internal_pv::PersistentVolumeClaimStatus) -> Self {
        Self {
            phase: if value.phase.is_empty() {
                None
            } else {
                Some(value.phase)
            },
            access_modes: value.access_modes,
            capacity: value.capacity,
            conditions: value
                .conditions
                .into_iter()
                .map(pod_condition_to_pvc_condition)
                .collect(),
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn volume_resources_to_internal(
    value: v1_pv::VolumeResourceRequirements,
) -> internal::ResourceRequirements {
    internal::ResourceRequirements {
        limits: value.limits,
        requests: value.requests,
    }
}

fn volume_resources_from_internal(
    value: internal::ResourceRequirements,
) -> v1_pv::VolumeResourceRequirements {
    v1_pv::VolumeResourceRequirements {
        limits: value.limits,
        requests: value.requests,
    }
}

fn pvc_condition_to_pod_condition(value: v1_pv::PersistentVolumeClaimCondition) -> PodCondition {
    PodCondition {
        type_: value.type_,
        status: value.status,
        last_probe_time: value.last_probe_time,
        last_transition_time: value.last_transition_time,
        reason: value.reason,
        message: value.message,
        observed_generation: None,
    }
}

fn pod_condition_to_pvc_condition(value: PodCondition) -> v1_pv::PersistentVolumeClaimCondition {
    v1_pv::PersistentVolumeClaimCondition {
        type_: value.type_,
        status: value.status,
        last_probe_time: value.last_probe_time,
        last_transition_time: value.last_transition_time,
        reason: value.reason,
        message: value.message,
    }
}

fn label_selector_to_internal(
    value: crate::common::meta::LabelSelector,
) -> internal::selector::LabelSelector {
    internal::selector::LabelSelector {
        match_labels: value.match_labels,
        match_expressions: value
            .match_expressions
            .into_iter()
            .map(|expr| internal::selector::LabelSelectorRequirement {
                key: expr.key,
                operator: expr.operator,
                values: expr.values,
            })
            .collect(),
    }
}

fn label_selector_from_internal(
    value: internal::selector::LabelSelector,
) -> crate::common::meta::LabelSelector {
    crate::common::meta::LabelSelector {
        match_labels: value.match_labels,
        match_expressions: value
            .match_expressions
            .into_iter()
            .map(|expr| crate::common::meta::LabelSelectorRequirement {
                key: expr.key,
                operator: expr.operator,
                values: expr.values,
            })
            .collect(),
    }
}

fn typed_object_reference_to_local(
    value: v1_pv::TypedObjectReference,
) -> TypedLocalObjectReference {
    TypedLocalObjectReference {
        api_group: value.api_group,
        kind: value.kind,
        name: value.name,
    }
}

fn typed_local_to_object_reference(
    value: TypedLocalObjectReference,
) -> v1_pv::TypedObjectReference {
    v1_pv::TypedObjectReference {
        api_group: value.api_group,
        kind: value.kind,
        name: value.name,
        namespace: None,
    }
}

fn is_empty_persistent_volume_source(value: &v1_pv::PersistentVolumeSource) -> bool {
    value.gce_persistent_disk.is_none()
        && value.aws_elastic_block_store.is_none()
        && value.host_path.is_none()
        && value.glusterfs.is_none()
        && value.nfs.is_none()
        && value.rbd.is_none()
        && value.iscsi.is_none()
        && value.cinder.is_none()
        && value.ceph_fs.is_none()
        && value.fc.is_none()
        && value.flocker.is_none()
        && value.flex_volume.is_none()
        && value.azure_file.is_none()
        && value.vsphere_volume.is_none()
        && value.quobyte.is_none()
        && value.azure_disk.is_none()
        && value.photon_persistent_disk.is_none()
        && value.portworx_volume.is_none()
        && value.scale_io.is_none()
        && value.local.is_none()
        && value.storage_os.is_none()
        && value.csi.is_none()
}

fn from_json_value<T: DeserializeOwned>(value: serde_json::Value) -> Option<T> {
    serde_json::from_value(value).ok()
}

fn to_json_value<T: Serialize>(value: &T) -> Option<serde_json::Value> {
    serde_json::to_value(value).ok()
}

fn persistent_volume_source_to_internal(
    value: v1_pv::PersistentVolumeSource,
) -> internal_pv::PersistentVolumeSource {
    internal_pv::PersistentVolumeSource {
        host_path: value
            .host_path
            .and_then(from_json_value::<internal_pv::HostPathVolumeSource>),
        gce_persistent_disk: value
            .gce_persistent_disk
            .and_then(from_json_value::<internal_pv::GCEPersistentDiskVolumeSource>),
        aws_elastic_block_store: value
            .aws_elastic_block_store
            .and_then(from_json_value::<internal_pv::AWSElasticBlockStoreVolumeSource>),
        nfs: value
            .nfs
            .and_then(from_json_value::<internal_pv::NFSVolumeSource>),
        iscsi: value
            .iscsi
            .and_then(from_json_value::<internal_pv::ISCSIPersistentVolumeSource>),
        glusterfs: value
            .glusterfs
            .and_then(from_json_value::<internal_pv::GlusterfsPersistentVolumeSource>),
        rbd: value
            .rbd
            .and_then(from_json_value::<internal_pv::RBDPersistentVolumeSource>),
        flex_volume: value
            .flex_volume
            .and_then(from_json_value::<internal_pv::FlexPersistentVolumeSource>),
        cinder: value
            .cinder
            .and_then(from_json_value::<internal_pv::CinderPersistentVolumeSource>),
        ceph_fs: value
            .ceph_fs
            .and_then(from_json_value::<internal_pv::CephFSPersistentVolumeSource>),
        flocker: value
            .flocker
            .and_then(from_json_value::<internal_pv::FlockerVolumeSource>),
        fc: value
            .fc
            .and_then(from_json_value::<internal_pv::FCVolumeSource>),
        azure_file: value
            .azure_file
            .and_then(from_json_value::<internal_pv::AzureFilePersistentVolumeSource>),
        vsphere_volume: value
            .vsphere_volume
            .and_then(from_json_value::<internal_pv::VsphereVirtualDiskVolumeSource>),
        quobyte: value
            .quobyte
            .and_then(from_json_value::<internal_pv::QuobyteVolumeSource>),
        azure_disk: value
            .azure_disk
            .and_then(from_json_value::<internal_pv::AzureDiskVolumeSource>),
        photon_persistent_disk: value
            .photon_persistent_disk
            .and_then(from_json_value::<internal_pv::PhotonPersistentDiskVolumeSource>),
        portworx_volume: value
            .portworx_volume
            .and_then(from_json_value::<internal_pv::PortworxVolumeSource>),
        scale_io: value
            .scale_io
            .and_then(from_json_value::<internal_pv::ScaleIOPersistentVolumeSource>),
        local: value.local.map(|l| internal_pv::LocalVolumeSource {
            path: l.path,
            fs_type: l.fs_type,
        }),
        storage_os: value
            .storage_os
            .and_then(from_json_value::<internal_pv::StorageOSPersistentVolumeSource>),
        csi: value
            .csi
            .and_then(from_json_value::<internal_pv::CSIPersistentVolumeSource>),
    }
}

fn persistent_volume_source_from_internal(
    value: internal_pv::PersistentVolumeSource,
) -> v1_pv::PersistentVolumeSource {
    v1_pv::PersistentVolumeSource {
        host_path: value.host_path.and_then(|v| to_json_value(&v)),
        gce_persistent_disk: value.gce_persistent_disk.and_then(|v| to_json_value(&v)),
        aws_elastic_block_store: value
            .aws_elastic_block_store
            .and_then(|v| to_json_value(&v)),
        nfs: value.nfs.and_then(|v| to_json_value(&v)),
        iscsi: value.iscsi.and_then(|v| to_json_value(&v)),
        glusterfs: value.glusterfs.and_then(|v| to_json_value(&v)),
        rbd: value.rbd.and_then(|v| to_json_value(&v)),
        flex_volume: value.flex_volume.and_then(|v| to_json_value(&v)),
        cinder: value.cinder.and_then(|v| to_json_value(&v)),
        ceph_fs: value.ceph_fs.and_then(|v| to_json_value(&v)),
        flocker: value.flocker.and_then(|v| to_json_value(&v)),
        fc: value.fc.and_then(|v| to_json_value(&v)),
        azure_file: value.azure_file.and_then(|v| to_json_value(&v)),
        vsphere_volume: value.vsphere_volume.and_then(|v| to_json_value(&v)),
        quobyte: value.quobyte.and_then(|v| to_json_value(&v)),
        azure_disk: value.azure_disk.and_then(|v| to_json_value(&v)),
        photon_persistent_disk: value.photon_persistent_disk.and_then(|v| to_json_value(&v)),
        portworx_volume: value.portworx_volume.and_then(|v| to_json_value(&v)),
        scale_io: value.scale_io.and_then(|v| to_json_value(&v)),
        local: value.local.map(|l| LocalVolumeSource {
            path: l.path,
            fs_type: l.fs_type,
        }),
        storage_os: value.storage_os.and_then(|v| to_json_value(&v)),
        csi: value.csi.and_then(|v| to_json_value(&v)),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pv_roundtrip_local_source() {
        let v1_pv = v1_pv::PersistentVolume {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("pv".to_string()),
                ..Default::default()
            }),
            spec: Some(v1_pv::PersistentVolumeSpec {
                capacity: std::collections::BTreeMap::new(),
                persistent_volume_source: Some(v1_pv::PersistentVolumeSource {
                    local: Some(LocalVolumeSource {
                        path: "/data".to_string(),
                        fs_type: Some("ext4".to_string()),
                    }),
                    ..Default::default()
                }),
                storage_class_name: Some("fast".to_string()),
                ..Default::default()
            }),
            status: None,
        };

        let internal = v1_pv.clone().to_internal();
        assert!(
            internal
                .spec
                .as_ref()
                .unwrap()
                .persistent_volume_source
                .local
                .is_some()
        );

        let mut roundtrip = v1_pv::PersistentVolume::from_internal(internal);
        roundtrip.apply_default();
        assert_eq!(
            roundtrip.spec.as_ref().unwrap().storage_class_name,
            Some("fast".to_string())
        );
        assert!(
            roundtrip
                .spec
                .as_ref()
                .unwrap()
                .persistent_volume_source
                .as_ref()
                .unwrap()
                .local
                .is_some()
        );
    }

    #[test]
    fn test_pv_volume_source_json_roundtrip() {
        let mut source = v1_pv::PersistentVolumeSource::default();
        source.nfs = Some(json!({
            "server": "nfs.example.com",
            "path": "/exports",
            "readOnly": true
        }));

        let internal = persistent_volume_source_to_internal(source);
        assert!(internal.nfs.is_some());

        let v1_back = persistent_volume_source_from_internal(internal);
        assert!(v1_back.nfs.is_some());
    }

    #[test]
    fn test_pvc_roundtrip_resources() {
        let v1_pvc = v1_pv::PersistentVolumeClaim {
            type_meta: crate::common::TypeMeta::default(),
            metadata: Some(crate::common::ObjectMeta {
                name: Some("pvc".to_string()),
                ..Default::default()
            }),
            spec: Some(v1_pv::PersistentVolumeClaimSpec {
                resources: Some(v1_pv::VolumeResourceRequirements {
                    limits: std::collections::BTreeMap::from([(
                        "storage".to_string(),
                        crate::common::Quantity("1Gi".to_string()),
                    )]),
                    requests: std::collections::BTreeMap::new(),
                }),
                ..Default::default()
            }),
            status: None,
        };

        let internal = v1_pvc.clone().to_internal();
        assert!(internal.spec.as_ref().unwrap().resources.is_some());

        let mut roundtrip = v1_pv::PersistentVolumeClaim::from_internal(internal);
        roundtrip.apply_default();
        assert!(roundtrip.spec.as_ref().unwrap().resources.is_some());
    }
}
