use crate::common::ObjectMeta;
use crate::common::validation::{
    BadValue, ErrorList, Path, invalid, not_supported, required, validate_object_meta,
    validate_object_meta_update,
};
use crate::rbac::internal::ClusterRoleBinding;
use crate::rbac::v1::api_group;

use super::{validate_rbac_name, validate_role_binding_subject};

pub fn validate_cluster_role_binding(role_binding: &ClusterRoleBinding) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = ObjectMeta::default();
    let meta = role_binding.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        false,
        validate_rbac_name,
        &Path::new("metadata"),
    ));

    if role_binding.role_ref.api_group != api_group::RBAC {
        all_errs.push(not_supported(
            &Path::new("roleRef").child("apiGroup"),
            BadValue::String(role_binding.role_ref.api_group.clone()),
            &[api_group::RBAC],
        ));
    }

    match role_binding.role_ref.kind.as_str() {
        "ClusterRole" => {}
        _ => {
            all_errs.push(not_supported(
                &Path::new("roleRef").child("kind"),
                BadValue::String(role_binding.role_ref.kind.clone()),
                &["ClusterRole"],
            ));
        }
    }

    if role_binding.role_ref.name.is_empty() {
        all_errs.push(required(&Path::new("roleRef").child("name"), ""));
    } else {
        for msg in validate_rbac_name(&role_binding.role_ref.name, false) {
            all_errs.push(invalid(
                &Path::new("roleRef").child("name"),
                BadValue::String(role_binding.role_ref.name.clone()),
                &msg,
            ));
        }
    }

    let subjects_path = Path::new("subjects");
    for (i, subject) in role_binding.subjects.iter().enumerate() {
        all_errs.extend(validate_role_binding_subject(
            subject,
            false,
            &subjects_path.index(i),
        ));
    }

    all_errs
}

pub fn validate_cluster_role_binding_update(
    role_binding: &ClusterRoleBinding,
    old_role_binding: &ClusterRoleBinding,
) -> ErrorList {
    let mut all_errs = validate_cluster_role_binding(role_binding);

    if let (Some(new_meta), Some(old_meta)) = (
        role_binding.metadata.as_ref(),
        old_role_binding.metadata.as_ref(),
    ) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    if old_role_binding.role_ref != role_binding.role_ref {
        all_errs.push(invalid(
            &Path::new("roleRef"),
            BadValue::String(format!("{:?}", role_binding.role_ref)),
            "cannot change roleRef",
        ));
    }

    all_errs
}
