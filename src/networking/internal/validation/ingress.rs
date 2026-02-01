//! Ingress validation.

use crate::common::validation::*;
use crate::networking::v1::ingress::{Ingress, IngressList};
use std::collections::BTreeSet;

use super::helpers::*;

// Ingress Validation
// ============================================================================

/// Validates an Ingress object.
pub fn validate_ingress(ingress: &Ingress) -> ErrorList {
    validate_ingress_with_path(ingress, &Path::nil())
}

fn validate_ingress_with_path(ingress: &Ingress, base_path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let meta = ingress.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta(
        meta,
        true,
        name_is_dns_subdomain,
        &base_path.child("metadata"),
    ));

    // Validate spec if present
    if let Some(ref spec) = ingress.spec {
        if spec.rules.is_empty() && spec.default_backend.is_none() {
            all_errs.push(invalid(
                &base_path.child("spec"),
                BadValue::String("missing rules and defaultBackend".to_string()),
                "either defaultBackend or rules must be specified",
            ));
        }

        if let Some(ref class_name) = spec.ingress_class_name {
            all_errs.extend(validate_dns1123_subdomain(
                class_name,
                &base_path.child("spec").child("ingressClassName"),
            ));
        }

        // Validate default backend
        if let Some(ref default_backend) = spec.default_backend {
            all_errs.extend(validate_ingress_backend(
                default_backend,
                &base_path.child("spec").child("defaultBackend"),
            ));
        }

        // Validate rules
        for (i, rule) in spec.rules.iter().enumerate() {
            let rule_path = base_path.child("spec").child("rules").index(i);

            // Validate host if present (should be valid DNS subdomain)
            if !rule.host.is_empty() {
                if is_ip_address(&rule.host) {
                    all_errs.push(invalid(
                        &rule_path.child("host"),
                        BadValue::String(rule.host.clone()),
                        "must be a DNS name, not an IP address",
                    ));
                } else if rule.host.contains('*') {
                    all_errs.extend(validate_wildcard_dns1123_subdomain(
                        &rule.host,
                        &rule_path.child("host"),
                    ));
                } else {
                    all_errs.extend(validate_dns1123_subdomain(
                        &rule.host,
                        &rule_path.child("host"),
                    ));
                }
            }

            // Validate HTTP paths
            if let Some(ref http) = rule.http {
                if http.paths.is_empty() {
                    all_errs.push(required(
                        &rule_path.child("http").child("paths"),
                        "paths is required",
                    ));
                }
                for (j, path) in http.paths.iter().enumerate() {
                    let path_path = rule_path.child("http").child("paths").index(j);

                    // Validate path field
                    if path.path.is_empty() {
                        all_errs.push(required(&path_path.child("path"), "path is required"));
                    } else {
                        // Path must start with '/' for Prefix and Exact types
                        use crate::networking::v1::ingress::PathType;
                        match path.path_type {
                            PathType::Prefix | PathType::Exact => {
                                if !path.path.starts_with('/') {
                                    all_errs.push(invalid(
                                        &path_path.child("path"),
                                        BadValue::String(path.path.clone()),
                                        "must be an absolute path",
                                    ));
                                }
                                for sequence in INVALID_PATH_SEQUENCES {
                                    if path.path.contains(sequence) {
                                        all_errs.push(invalid(
                                            &path_path.child("path"),
                                            BadValue::String(path.path.clone()),
                                            &format!("must not contain '{}'", sequence),
                                        ));
                                    }
                                }
                                for suffix in INVALID_PATH_SUFFIXES {
                                    if path.path.ends_with(suffix) {
                                        all_errs.push(invalid(
                                            &path_path.child("path"),
                                            BadValue::String(path.path.clone()),
                                            &format!("cannot end with '{}'", suffix),
                                        ));
                                    }
                                }
                            }
                            PathType::ImplementationSpecific => {
                                if !path.path.is_empty() && !path.path.starts_with('/') {
                                    all_errs.push(invalid(
                                        &path_path.child("path"),
                                        BadValue::String(path.path.clone()),
                                        "must be an absolute path",
                                    ));
                                }
                            }
                        }
                    }

                    // Validate backend
                    all_errs.extend(validate_ingress_backend(
                        &path.backend,
                        &path_path.child("backend"),
                    ));
                }
            }
        }

        // Validate TLS hosts and secret names if present
        for (i, tls) in spec.tls.iter().enumerate() {
            for (j, host) in tls.hosts.iter().enumerate() {
                if host.contains('*') {
                    all_errs.extend(validate_wildcard_dns1123_subdomain(
                        host,
                        &base_path
                            .child("spec")
                            .child("tls")
                            .index(i)
                            .child("hosts")
                            .index(j),
                    ));
                } else {
                    all_errs.extend(validate_dns1123_subdomain(
                        host,
                        &base_path
                            .child("spec")
                            .child("tls")
                            .index(i)
                            .child("hosts")
                            .index(j),
                    ));
                }
            }

            if let Some(secret_name) = tls.secret_name.as_ref()
                && !secret_name.is_empty()
            {
                all_errs.extend(validate_dns1123_subdomain(
                    secret_name,
                    &base_path
                        .child("spec")
                        .child("tls")
                        .index(i)
                        .child("secretName"),
                ));
            }
        }
    }

    all_errs
}

/// Validates an IngressList object.
pub fn validate_ingress_list(list: &IngressList) -> ErrorList {
    let mut all_errs = ErrorList::new();

    for (i, item) in list.items.iter().enumerate() {
        let item_path = Path::new("items").index(i);
        all_errs.extend(validate_ingress_with_path(item, &item_path));
    }

    all_errs
}

/// Validates an Ingress update.
pub fn validate_ingress_update(new_ingress: &Ingress, old_ingress: &Ingress) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_ingress.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_ingress.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    all_errs.extend(validate_ingress_with_path(new_ingress, &Path::nil()));

    all_errs
}

fn validate_ingress_load_balancer_status(
    status: &crate::networking::v1::ingress::IngressLoadBalancerStatus,
    old_status: Option<&crate::networking::v1::ingress::IngressLoadBalancerStatus>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut existing_ips = BTreeSet::new();
    if let Some(old_status) = old_status {
        for ingress in &old_status.ingress {
            if !ingress.ip.is_empty() {
                existing_ips.insert(ingress.ip.clone());
            }
        }
    }

    for (i, ingress) in status.ingress.iter().enumerate() {
        let ingress_path = path.child("ingress").index(i);
        if !ingress.ip.is_empty() && !existing_ips.contains(&ingress.ip) {
            let errs = validate_ip_address_name(&ingress.ip, false);
            for msg in errs {
                all_errs.push(invalid(
                    &ingress_path.child("ip"),
                    BadValue::String(ingress.ip.clone()),
                    &msg,
                ));
            }
        }

        if !ingress.hostname.is_empty() {
            if is_ip_address(&ingress.hostname) {
                all_errs.push(invalid(
                    &ingress_path.child("hostname"),
                    BadValue::String(ingress.hostname.clone()),
                    "must be a DNS name, not an IP address",
                ));
            } else {
                all_errs.extend(validate_dns1123_subdomain(
                    &ingress.hostname,
                    &ingress_path.child("hostname"),
                ));
            }
        }

        for (j, port_status) in ingress.ports.iter().enumerate() {
            let port_path = ingress_path.child("ports").index(j);
            if port_status.protocol.is_empty()
                || !SUPPORTED_PROTOCOLS.contains(&port_status.protocol.as_str())
            {
                all_errs.push(not_supported(
                    &port_path.child("protocol"),
                    BadValue::String(port_status.protocol.clone()),
                    &SUPPORTED_PROTOCOLS,
                ));
            }
        }
    }

    all_errs
}

/// Validates an Ingress status update.
pub fn validate_ingress_status_update(new_ingress: &Ingress, old_ingress: &Ingress) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let default_meta = crate::common::ObjectMeta::default();
    let new_meta = new_ingress.metadata.as_ref().unwrap_or(&default_meta);
    let old_meta = old_ingress.metadata.as_ref().unwrap_or(&default_meta);

    all_errs.extend(validate_object_meta_update(
        new_meta,
        old_meta,
        &Path::new("metadata"),
    ));

    if let Some(ref status) = new_ingress.status {
        let old_status = old_ingress.status.as_ref();
        if let Some(ref load_balancer) = status.load_balancer {
            all_errs.extend(validate_ingress_load_balancer_status(
                load_balancer,
                old_status.and_then(|s| s.load_balancer.as_ref()),
                &Path::new("status").child("loadBalancer"),
            ));
        }
    }

    all_errs
}

// ============================================================================
