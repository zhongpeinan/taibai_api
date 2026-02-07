//! Security context validation for Kubernetes core internal API.
//!
//! Ported from k8s.io/kubernetes/pkg/apis/core/validation/validation.go

use crate::common::validation::{BadValue, ErrorList, Path, invalid, not_supported, required};
use crate::core::internal::security::{PodSecurityContext, Sysctl};
use crate::core::v1::security::{
    self, AppArmorProfile, SeccompProfile, SecurityContext, WindowsSecurityContextOptions,
};

const IS_NEGATIVE_ERROR_MSG: &str = "must be greater than or equal to 0";

/// Maximum length for a localhost profile path.
const MAX_LOCALHOST_PROFILE_LENGTH: usize = 4095;

/// Maximum size of GMSA credential spec (64 KiB).
const MAX_GMSA_CREDENTIAL_SPEC_LENGTH: usize = 64 * 1024;

/// Valid proc mount types.
const VALID_PROC_MOUNT_TYPES: &[&str] = &[
    security::proc_mount_type::DEFAULT,
    security::proc_mount_type::UNMASKED,
];

/// Valid seccomp profile types.
const VALID_SECCOMP_PROFILE_TYPES: &[&str] = &[
    security::seccomp_profile_type::UNCONFINED,
    security::seccomp_profile_type::RUNTIME_DEFAULT,
    security::seccomp_profile_type::LOCALHOST,
];

/// Valid AppArmor profile types.
const VALID_APP_ARMOR_PROFILE_TYPES: &[&str] = &[
    security::app_armor_profile_type::UNCONFINED,
    security::app_armor_profile_type::RUNTIME_DEFAULT,
    security::app_armor_profile_type::LOCALHOST,
];

/// Validates PodSecurityContext.
pub fn validate_pod_security_context(sec_ctx: &PodSecurityContext, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(value) = sec_ctx.run_as_user {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsUser")));
    }

    if let Some(value) = sec_ctx.run_as_group {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsGroup")));
    }

    if let Some(value) = sec_ctx.fs_group {
        all_errs.extend(validate_nonnegative_field(value, &path.child("fsGroup")));
    }

    for (i, group) in sec_ctx.supplemental_groups.iter().enumerate() {
        all_errs.extend(validate_nonnegative_field(
            *group,
            &path.child("supplementalGroups").index(i),
        ));
    }

    all_errs.extend(validate_sysctls(&sec_ctx.sysctls, &path.child("sysctls")));

    all_errs
}

/// Validates Sysctl values.
pub fn validate_sysctls(sysctls: &[Sysctl], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, sysctl) in sysctls.iter().enumerate() {
        let idx_path = path.index(i);
        if sysctl.name.is_empty() {
            all_errs.push(required(&idx_path.child("name"), "name is required"));
        }
        if sysctl.value.is_empty() {
            all_errs.push(required(&idx_path.child("value"), "value is required"));
        }
    }

    all_errs
}

/// Validates a container-level SecurityContext.
///
/// Corresponds to [upstream ValidateSecurityContext](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/validation/validation.go)
pub fn validate_security_context(sc: &SecurityContext, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate runAsUser is non-negative
    if let Some(value) = sc.run_as_user {
        all_errs.extend(validate_nonnegative_field(value, &path.child("runAsUser")));
    }

    // Validate runAsGroup is non-negative
    if let Some(value) = sc.run_as_group {
        all_errs.extend(validate_nonnegative_field(
            value,
            &path.child("runAsGroup"),
        ));
    }

    // Validate procMount
    if let Some(ref proc_mount) = sc.proc_mount {
        all_errs.extend(validate_proc_mount_type(
            proc_mount,
            &path.child("procMount"),
        ));
    }

    // Validate allowPrivilegeEscalation conflicts with privileged
    if let Some(false) = sc.allow_privilege_escalation {
        if let Some(true) = sc.privileged {
            all_errs.push(invalid(
                &path.child("allowPrivilegeEscalation"),
                BadValue::String("false".to_string()),
                "cannot set allowPrivilegeEscalation to false and privileged to true",
            ));
        }
    }

    // Validate seccomp profile
    if let Some(ref seccomp) = sc.seccomp_profile {
        all_errs.extend(validate_seccomp_profile_field(
            seccomp,
            &path.child("seccompProfile"),
        ));
    }

    // Validate appArmor profile
    if let Some(ref app_armor) = sc.app_armor_profile {
        all_errs.extend(validate_app_armor_profile_field(
            app_armor,
            &path.child("appArmorProfile"),
        ));
    }

    // Validate windows options
    if let Some(ref windows) = sc.windows_options {
        all_errs.extend(validate_windows_security_context_options(
            windows,
            &path.child("windowsOptions"),
        ));
    }

    all_errs
}

/// Validates ProcMountType.
///
/// Only Default and Unmasked are accepted.
fn validate_proc_mount_type(proc_mount: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !VALID_PROC_MOUNT_TYPES.contains(&proc_mount) {
        all_errs.push(not_supported(
            path,
            BadValue::String(proc_mount.to_string()),
            VALID_PROC_MOUNT_TYPES,
        ));
    }
    all_errs
}

/// Validates a SeccompProfile.
///
/// - Type must be a valid SeccompProfileType
/// - If Localhost: localhostProfile is required
/// - Otherwise: localhostProfile must not be set
fn validate_seccomp_profile_field(profile: &SeccompProfile, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let type_str = profile.type_.as_str();

    if !VALID_SECCOMP_PROFILE_TYPES.contains(&type_str) {
        all_errs.push(not_supported(
            &path.child("type"),
            BadValue::String(type_str.to_string()),
            VALID_SECCOMP_PROFILE_TYPES,
        ));
        return all_errs;
    }

    if type_str == security::seccomp_profile_type::LOCALHOST {
        if let Some(ref localhost_profile) = profile.localhost_profile {
            if localhost_profile.is_empty() {
                all_errs.push(required(
                    &path.child("localhostProfile"),
                    "must be set when type is Localhost",
                ));
            } else if localhost_profile.contains("..") {
                all_errs.push(invalid(
                    &path.child("localhostProfile"),
                    BadValue::String(localhost_profile.clone()),
                    "must not contain '..'",
                ));
            }
        } else {
            all_errs.push(required(
                &path.child("localhostProfile"),
                "must be set when type is Localhost",
            ));
        }
    } else if profile.localhost_profile.is_some() {
        all_errs.push(invalid(
            &path.child("localhostProfile"),
            BadValue::String(profile.localhost_profile.clone().unwrap_or_default()),
            "must not be set when type is not Localhost",
        ));
    }

    all_errs
}

/// Validates an AppArmorProfile.
///
/// - If Localhost: localhostProfile is required and must be <= 4095 chars
/// - Otherwise: localhostProfile must not be set
fn validate_app_armor_profile_field(profile: &AppArmorProfile, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    let type_str = profile.type_.as_str();

    if !VALID_APP_ARMOR_PROFILE_TYPES.contains(&type_str) {
        all_errs.push(not_supported(
            &path.child("type"),
            BadValue::String(type_str.to_string()),
            VALID_APP_ARMOR_PROFILE_TYPES,
        ));
        return all_errs;
    }

    if type_str == security::app_armor_profile_type::LOCALHOST {
        if let Some(ref localhost_profile) = profile.localhost_profile {
            let trimmed = localhost_profile.trim();
            if trimmed.is_empty() {
                all_errs.push(required(
                    &path.child("localhostProfile"),
                    "must be set when type is Localhost",
                ));
            } else if localhost_profile.len() > MAX_LOCALHOST_PROFILE_LENGTH {
                all_errs.push(invalid(
                    &path.child("localhostProfile"),
                    BadValue::String(format!("length {}", localhost_profile.len())),
                    &format!(
                        "must be less than or equal to {} characters",
                        MAX_LOCALHOST_PROFILE_LENGTH
                    ),
                ));
            }
        } else {
            all_errs.push(required(
                &path.child("localhostProfile"),
                "must be set when type is Localhost",
            ));
        }
    } else if profile.localhost_profile.is_some() {
        all_errs.push(invalid(
            &path.child("localhostProfile"),
            BadValue::String(profile.localhost_profile.clone().unwrap_or_default()),
            "must not be set when type is not Localhost",
        ));
    }

    all_errs
}

/// Validates WindowsSecurityContextOptions.
///
/// Corresponds to [upstream validateWindowsSecurityContextOptions](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/core/validation/validation.go)
fn validate_windows_security_context_options(
    opts: &WindowsSecurityContextOptions,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate GMSACredentialSpecName
    if let Some(ref name) = opts.gmsa_credential_spec_name {
        if name.is_empty() {
            all_errs.push(invalid(
                &path.child("gmsaCredentialSpecName"),
                BadValue::String(name.clone()),
                "must not be empty when specified",
            ));
        }
    }

    // Validate GMSACredentialSpec (not empty, max 64 KiB)
    if let Some(ref spec) = opts.gmsa_credential_spec {
        if spec.is_empty() {
            all_errs.push(invalid(
                &path.child("gmsaCredentialSpec"),
                BadValue::String(String::new()),
                "must not be empty when specified",
            ));
        } else if spec.len() > MAX_GMSA_CREDENTIAL_SPEC_LENGTH {
            all_errs.push(invalid(
                &path.child("gmsaCredentialSpec"),
                BadValue::String(format!("length {}", spec.len())),
                &format!(
                    "must not be longer than {} bytes",
                    MAX_GMSA_CREDENTIAL_SPEC_LENGTH
                ),
            ));
        }
    }

    // Validate RunAsUserName
    if let Some(ref user_name) = opts.run_as_user_name {
        if user_name.is_empty() {
            all_errs.push(invalid(
                &path.child("runAsUserName"),
                BadValue::String(user_name.clone()),
                "must not be empty when specified",
            ));
        }
    }

    all_errs
}

fn validate_nonnegative_field(value: i64, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if value < 0 {
        all_errs.push(invalid(path, BadValue::Int(value), IS_NEGATIVE_ERROR_MSG));
    }
    all_errs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::validation::Path;
    use crate::core::v1::security;

    #[test]
    fn test_validate_security_context_valid() {
        let sc = SecurityContext {
            run_as_user: Some(1000),
            run_as_group: Some(1000),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_security_context_negative_run_as_user() {
        let sc = SecurityContext {
            run_as_user: Some(-1),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for negative runAsUser");
        assert!(errs
            .errors
            .iter()
            .any(|e| e.field.contains("runAsUser")));
    }

    #[test]
    fn test_validate_security_context_negative_run_as_group() {
        let sc = SecurityContext {
            run_as_group: Some(-5),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for negative runAsGroup");
        assert!(errs
            .errors
            .iter()
            .any(|e| e.field.contains("runAsGroup")));
    }

    #[test]
    fn test_validate_security_context_invalid_proc_mount() {
        let sc = SecurityContext {
            proc_mount: Some("BadType".to_string()),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for invalid proc mount type"
        );
        assert!(errs
            .errors
            .iter()
            .any(|e| e.field.contains("procMount")));
    }

    #[test]
    fn test_validate_security_context_valid_proc_mount() {
        let sc = SecurityContext {
            proc_mount: Some(security::proc_mount_type::UNMASKED.to_string()),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_security_context_escalation_conflict() {
        let sc = SecurityContext {
            privileged: Some(true),
            allow_privilege_escalation: Some(false),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for escalation conflict"
        );
        assert!(errs
            .errors
            .iter()
            .any(|e| e.detail.contains("cannot set allowPrivilegeEscalation")));
    }

    #[test]
    fn test_validate_security_context_escalation_no_conflict() {
        let sc = SecurityContext {
            privileged: Some(true),
            allow_privilege_escalation: Some(true),
            ..Default::default()
        };
        let errs = validate_security_context(&sc, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_seccomp_profile_localhost_valid() {
        let profile = SeccompProfile {
            type_: security::seccomp_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("profiles/my-profile.json".to_string()),
        };
        let errs = validate_seccomp_profile_field(&profile, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_seccomp_profile_localhost_missing() {
        let profile = SeccompProfile {
            type_: security::seccomp_profile_type::LOCALHOST.to_string(),
            localhost_profile: None,
        };
        let errs = validate_seccomp_profile_field(&profile, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for missing localhost profile"
        );
    }

    #[test]
    fn test_validate_seccomp_profile_localhost_with_dotdot() {
        let profile = SeccompProfile {
            type_: security::seccomp_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("../evil-profile.json".to_string()),
        };
        let errs = validate_seccomp_profile_field(&profile, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for path traversal");
        assert!(errs
            .errors
            .iter()
            .any(|e| e.detail.contains("must not contain '..'")));
    }

    #[test]
    fn test_validate_seccomp_profile_unconfined_with_profile() {
        let profile = SeccompProfile {
            type_: security::seccomp_profile_type::UNCONFINED.to_string(),
            localhost_profile: Some("should-not-be-here".to_string()),
        };
        let errs = validate_seccomp_profile_field(&profile, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for profile set on non-Localhost type"
        );
    }

    #[test]
    fn test_validate_seccomp_profile_runtime_default_valid() {
        let profile = SeccompProfile {
            type_: security::seccomp_profile_type::RUNTIME_DEFAULT.to_string(),
            localhost_profile: None,
        };
        let errs = validate_seccomp_profile_field(&profile, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_seccomp_profile_invalid_type() {
        let profile = SeccompProfile {
            type_: "InvalidType".to_string(),
            localhost_profile: None,
        };
        let errs = validate_seccomp_profile_field(&profile, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for invalid type");
    }

    #[test]
    fn test_validate_apparmor_profile_localhost_valid() {
        let profile = AppArmorProfile {
            type_: security::app_armor_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("my-profile".to_string()),
        };
        let errs = validate_app_armor_profile_field(&profile, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_apparmor_profile_localhost_missing() {
        let profile = AppArmorProfile {
            type_: security::app_armor_profile_type::LOCALHOST.to_string(),
            localhost_profile: None,
        };
        let errs = validate_app_armor_profile_field(&profile, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for missing localhost profile"
        );
    }

    #[test]
    fn test_validate_apparmor_profile_localhost_too_long() {
        let profile = AppArmorProfile {
            type_: security::app_armor_profile_type::LOCALHOST.to_string(),
            localhost_profile: Some("a".repeat(MAX_LOCALHOST_PROFILE_LENGTH + 1)),
        };
        let errs = validate_app_armor_profile_field(&profile, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for too-long profile");
    }

    #[test]
    fn test_validate_apparmor_profile_unconfined_with_profile() {
        let profile = AppArmorProfile {
            type_: security::app_armor_profile_type::UNCONFINED.to_string(),
            localhost_profile: Some("should-not-be-here".to_string()),
        };
        let errs = validate_app_armor_profile_field(&profile, &Path::nil());
        assert!(
            !errs.is_empty(),
            "Expected error for profile set on non-Localhost type"
        );
    }

    #[test]
    fn test_validate_apparmor_profile_invalid_type() {
        let profile = AppArmorProfile {
            type_: "InvalidType".to_string(),
            localhost_profile: None,
        };
        let errs = validate_app_armor_profile_field(&profile, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for invalid type");
    }

    #[test]
    fn test_validate_windows_options_valid() {
        let opts = WindowsSecurityContextOptions {
            gmsa_credential_spec_name: Some("my-cred".to_string()),
            gmsa_credential_spec: Some("{\"spec\": true}".to_string()),
            run_as_user_name: Some("NT AUTHORITY\\SYSTEM".to_string()),
            host_process: None,
        };
        let errs = validate_windows_security_context_options(&opts, &Path::nil());
        assert!(errs.is_empty(), "Expected no errors, got: {:?}", errs);
    }

    #[test]
    fn test_validate_windows_options_empty_fields() {
        let opts = WindowsSecurityContextOptions {
            gmsa_credential_spec_name: Some(String::new()),
            gmsa_credential_spec: Some(String::new()),
            run_as_user_name: Some(String::new()),
            host_process: None,
        };
        let errs = validate_windows_security_context_options(&opts, &Path::nil());
        assert_eq!(
            errs.errors.len(),
            3,
            "Expected 3 errors for empty fields, got: {:?}",
            errs
        );
    }

    #[test]
    fn test_validate_windows_options_gmsa_spec_too_large() {
        let opts = WindowsSecurityContextOptions {
            gmsa_credential_spec: Some("x".repeat(MAX_GMSA_CREDENTIAL_SPEC_LENGTH + 1)),
            ..Default::default()
        };
        let errs = validate_windows_security_context_options(&opts, &Path::nil());
        assert!(!errs.is_empty(), "Expected error for oversized spec");
    }
}
