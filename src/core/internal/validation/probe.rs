//! Probe and Lifecycle validation for Kubernetes core internal API.

use crate::common::util::IntOrString;
use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, not_supported, required,
};
use crate::core::internal::uri_scheme;
use crate::core::internal::{
    ExecAction, GRPCAction, HTTPGetAction, HTTPHeader, Lifecycle, LifecycleHandler, Probe,
    ProbeHandler, TCPSocketAction,
};
use std::collections::HashSet;
use std::sync::LazyLock;

/// Supported HTTP schemes for HTTPGetAction
static SUPPORTED_HTTP_SCHEMES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from([uri_scheme::HTTP, uri_scheme::HTTPS]));

/// Static array of supported HTTP schemes for error messages
const SUPPORTED_HTTP_SCHEMES_ARRAY: &[&str] = &[uri_scheme::HTTP, uri_scheme::HTTPS];

// ============================================================================
// Probe Validation Functions
// ============================================================================

pub fn validate_liveness_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(p) = probe {
        all_errs.extend(validate_probe(p, grace_period, path));

        if let Some(success_threshold) = p.success_threshold {
            if success_threshold != 1 {
                all_errs.push(invalid(
                    &path.child("successThreshold"),
                    BadValue::Int(success_threshold.into()),
                    "must be 1",
                ));
            }
        }
    }

    all_errs
}

pub fn validate_readiness_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(p) = probe {
        all_errs.extend(validate_probe(p, grace_period, path));

        if p.termination_grace_period_seconds.is_some() {
            all_errs.push(invalid(
                &path.child("terminationGracePeriodSeconds"),
                BadValue::Int(p.termination_grace_period_seconds.unwrap()),
                "must not be set for readinessProbes",
            ));
        }
    }

    all_errs
}

pub fn validate_startup_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(p) = probe {
        all_errs.extend(validate_probe(p, grace_period, path));

        if let Some(success_threshold) = p.success_threshold {
            if success_threshold != 1 {
                all_errs.push(invalid(
                    &path.child("successThreshold"),
                    BadValue::Int(success_threshold.into()),
                    "must be 1",
                ));
            }
        }
    }

    all_errs
}

fn validate_probe(probe: &Probe, grace_period: &Option<i64>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    all_errs.extend(validate_probe_handler(
        &probe.probe_handler,
        grace_period,
        path,
    ));

    if let Some(initial_delay) = probe.initial_delay_seconds {
        if initial_delay < 0 {
            all_errs.push(invalid(
                &path.child("initialDelaySeconds"),
                BadValue::Int(initial_delay.into()),
                "must be non-negative",
            ));
        }
    }

    if let Some(timeout) = probe.timeout_seconds {
        if timeout < 0 {
            all_errs.push(invalid(
                &path.child("timeoutSeconds"),
                BadValue::Int(timeout.into()),
                "must be non-negative",
            ));
        }
    }

    if let Some(period) = probe.period_seconds {
        if period < 0 {
            all_errs.push(invalid(
                &path.child("periodSeconds"),
                BadValue::Int(period.into()),
                "must be non-negative",
            ));
        }
    }

    if let Some(success_threshold) = probe.success_threshold {
        if success_threshold < 0 {
            all_errs.push(invalid(
                &path.child("successThreshold"),
                BadValue::Int(success_threshold.into()),
                "must be non-negative",
            ));
        }
    }

    if let Some(failure_threshold) = probe.failure_threshold {
        if failure_threshold < 0 {
            all_errs.push(invalid(
                &path.child("failureThreshold"),
                BadValue::Int(failure_threshold.into()),
                "must be non-negative",
            ));
        }
    }

    if let Some(term_grace) = probe.termination_grace_period_seconds {
        if term_grace <= 0 {
            all_errs.push(invalid(
                &path.child("terminationGracePeriodSeconds"),
                BadValue::Int(term_grace),
                "must be greater than 0",
            ));
        }
    }

    all_errs
}

fn validate_probe_handler(
    handler: &ProbeHandler,
    _grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_handlers = 0;

    if let Some(ref exec) = handler.exec {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("exec"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_exec_action(exec, &path.child("exec")));
        }
    }

    if let Some(ref http_get) = handler.http_get {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("httpGet"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_http_get_action(http_get, &path.child("httpGet")));
        }
    }

    if let Some(ref tcp_socket) = handler.tcp_socket {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("tcpSocket"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_tcp_socket_action(
                tcp_socket,
                &path.child("tcpSocket"),
            ));
        }
    }

    if let Some(ref grpc) = handler.grpc {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("grpc"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_grpc_action(grpc, &path.child("grpc")));
        }
    }

    if num_handlers == 0 {
        all_errs.push(required(path, "must specify a handler type"));
    }

    all_errs
}

// ============================================================================
// Lifecycle Validation Functions
// ============================================================================

pub fn validate_lifecycle(
    lifecycle: Option<&Lifecycle>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(lc) = lifecycle {
        if let Some(ref post_start) = lc.post_start {
            all_errs.extend(validate_lifecycle_handler(
                post_start,
                grace_period,
                &path.child("postStart"),
            ));
        }

        if let Some(ref pre_stop) = lc.pre_stop {
            all_errs.extend(validate_lifecycle_handler(
                pre_stop,
                grace_period,
                &path.child("preStop"),
            ));
        }
    }

    all_errs
}

fn validate_lifecycle_handler(
    handler: &LifecycleHandler,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_handlers = 0;

    if let Some(ref exec) = handler.exec {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("exec"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_exec_action(exec, &path.child("exec")));
        }
    }

    if let Some(ref http_get) = handler.http_get {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("httpGet"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_http_get_action(http_get, &path.child("httpGet")));
        }
    }

    if let Some(ref tcp_socket) = handler.tcp_socket {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("tcpSocket"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_tcp_socket_action(
                tcp_socket,
                &path.child("tcpSocket"),
            ));
        }
    }

    if let Some(ref sleep_seconds) = handler.sleep {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("sleep"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_sleep_action(
                *sleep_seconds,
                grace_period,
                &path.child("sleep"),
            ));
        }
    }

    if num_handlers == 0 {
        all_errs.push(required(path, "must specify a handler type"));
    }

    all_errs
}

// ============================================================================
// Handler Action Validation Functions
// ============================================================================

fn validate_exec_action(exec: &ExecAction, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if exec.command.is_empty() {
        all_errs.push(required(&path.child("command"), "command is required"));
    }

    all_errs
}

fn validate_http_get_action(http: &HTTPGetAction, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if http.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    }

    all_errs.extend(validate_port_num_or_name(&http.port, &path.child("port")));

    if let Some(ref scheme) = http.scheme {
        if !SUPPORTED_HTTP_SCHEMES.contains(scheme.as_str()) {
            all_errs.push(not_supported(
                &path.child("scheme"),
                BadValue::String(scheme.clone()),
                SUPPORTED_HTTP_SCHEMES_ARRAY,
            ));
        }
    }

    for (i, header) in http.http_headers.iter().enumerate() {
        let header_path = path.child("httpHeaders").index(i);
        all_errs.extend(validate_http_header(header, &header_path));
    }

    all_errs
}

fn validate_http_header(header: &HTTPHeader, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if header.name.is_empty() {
        all_errs.push(required(&path.child("name"), "header name is required"));
    } else if !is_valid_http_header_name(&header.name) {
        all_errs.push(invalid(
            &path.child("name"),
            BadValue::String(header.name.clone()),
            "invalid HTTP header name",
        ));
    }

    all_errs
}

fn validate_tcp_socket_action(tcp: &TCPSocketAction, path: &Path) -> ErrorList {
    validate_port_num_or_name(&tcp.port, &path.child("port"))
}

fn validate_grpc_action(grpc: &GRPCAction, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if grpc.port <= 0 {
        all_errs.push(invalid(
            &path.child("port"),
            BadValue::Int(grpc.port.into()),
            "port must be positive",
        ));
    } else if grpc.port > 65535 {
        all_errs.push(invalid(
            &path.child("port"),
            BadValue::Int(grpc.port.into()),
            "port must be between 1 and 65535",
        ));
    }

    all_errs
}

fn validate_sleep_action(seconds: i64, grace_period: &Option<i64>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(grace) = grace_period {
        if seconds < 0 || seconds > *grace {
            all_errs.push(invalid(
                path,
                BadValue::Int(seconds),
                &format!(
                    "must be non-negative and less than terminationGracePeriodSeconds ({})",
                    grace
                ),
            ));
        }
    }

    all_errs
}

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_port_num_or_name(port: &IntOrString, path: &Path) -> ErrorList {
    match port {
        IntOrString::Int(num) => validate_port_number(*num, path),
        IntOrString::String(name) => validate_port_name(name, path),
    }
}

fn validate_port_number(port: i32, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if !(1..=65535).contains(&port) {
        all_errs.push(invalid(
            path,
            BadValue::Int(port as i64),
            "must be in the range 1-65535",
        ));
    }
    all_errs
}

fn validate_port_name(name: &str, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if name.is_empty() {
        all_errs.push(required(path, "port name must not be empty"));
        return all_errs;
    }

    let is_valid = name.len() <= 15
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && name.chars().any(|c| c.is_ascii_lowercase())
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--");

    if !is_valid {
        all_errs.push(invalid(
            path,
            BadValue::String(name.to_string()),
            "must be a valid port name (lowercase alphanumeric or '-', 1-15 chars, must contain at least one letter, no consecutive hyphens)",
        ));
    }
    all_errs
}

fn is_valid_http_header_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    for c in name.chars() {
        if !c.is_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }

    if name.starts_with('-') || name.ends_with('-') {
        return false;
    }

    true
}
