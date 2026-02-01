use crate::common::ObjectMeta;
use crate::common::validation::{
    ErrorList, Path, validate_object_meta, validate_object_meta_update,
};
use crate::rbac::internal::Role;

use super::{validate_policy_rule, validate_rbac_name};

pub fn validate_role(role: &Role) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = ObjectMeta::default();
    let meta = role.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        validate_rbac_name,
        &Path::new("metadata"),
    ));

    for (i, rule) in role.rules.iter().enumerate() {
        all_errs.extend(validate_policy_rule(
            rule,
            true,
            &Path::new("rules").index(i),
        ));
    }

    all_errs
}

pub fn validate_role_update(role: &Role, old_role: &Role) -> ErrorList {
    let mut all_errs = validate_role(role);

    if let (Some(new_meta), Some(old_meta)) = (role.metadata.as_ref(), old_role.metadata.as_ref()) {
        all_errs.extend(validate_object_meta_update(
            new_meta,
            old_meta,
            &Path::new("metadata"),
        ));
    }

    all_errs
}
