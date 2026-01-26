//! Probe and Lifecycle validation for Kubernetes core/v1 API
//!
//! This module implements validation for probes (liveness, readiness, startup) and lifecycle hooks.

use crate::common::validation::{
    BadValue, ErrorList, Path, forbidden, invalid, not_supported, required,
};
use crate::core::v1::probe::{
    ExecAction, GRPCAction, HTTPGetAction, Lifecycle, LifecycleHandler, Probe, ProbeHandler,
    SleepAction, TCPSocketAction, uri_scheme,
};
use crate::core::v1::validation::helpers::validate_port_num_or_name;
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

/// Validates a liveness probe.
///
/// Liveness probes have special requirements:
/// - successThreshold must be 1
pub fn validate_liveness_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(p) = probe {
        all_errs.extend(validate_probe(p, grace_period, path));

        // Liveness probe must have successThreshold = 1
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

/// Validates a readiness probe.
///
/// Readiness probes have special requirements:
/// - terminationGracePeriodSeconds must NOT be set
pub fn validate_readiness_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(p) = probe {
        all_errs.extend(validate_probe(p, grace_period, path));

        // Readiness probe must not have terminationGracePeriodSeconds set
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

/// Validates a startup probe.
///
/// Startup probes have special requirements:
/// - successThreshold must be 1
pub fn validate_startup_probe(
    probe: Option<&Probe>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(p) = probe {
        all_errs.extend(validate_probe(p, grace_period, path));

        // Startup probe must have successThreshold = 1
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

/// Validates a probe.
///
/// Validates:
/// - Probe handler (exactly one type must be specified)
/// - initialDelaySeconds >= 0
/// - timeoutSeconds >= 0
/// - periodSeconds >= 0
/// - successThreshold >= 0
/// - failureThreshold >= 0
/// - terminationGracePeriodSeconds > 0 (if set)
fn validate_probe(probe: &Probe, grace_period: &Option<i64>, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate the probe handler
    all_errs.extend(validate_probe_handler(
        &probe.probe_handler,
        grace_period,
        path,
    ));

    // Validate initialDelaySeconds is non-negative
    if let Some(initial_delay) = probe.initial_delay_seconds {
        if initial_delay < 0 {
            all_errs.push(invalid(
                &path.child("initialDelaySeconds"),
                BadValue::Int(initial_delay.into()),
                "must be non-negative",
            ));
        }
    }

    // Validate timeoutSeconds is non-negative
    if let Some(timeout) = probe.timeout_seconds {
        if timeout < 0 {
            all_errs.push(invalid(
                &path.child("timeoutSeconds"),
                BadValue::Int(timeout.into()),
                "must be non-negative",
            ));
        }
    }

    // Validate periodSeconds is non-negative
    if let Some(period) = probe.period_seconds {
        if period < 0 {
            all_errs.push(invalid(
                &path.child("periodSeconds"),
                BadValue::Int(period.into()),
                "must be non-negative",
            ));
        }
    }

    // Validate successThreshold is non-negative
    if let Some(success_threshold) = probe.success_threshold {
        if success_threshold < 0 {
            all_errs.push(invalid(
                &path.child("successThreshold"),
                BadValue::Int(success_threshold.into()),
                "must be non-negative",
            ));
        }
    }

    // Validate failureThreshold is non-negative
    if let Some(failure_threshold) = probe.failure_threshold {
        if failure_threshold < 0 {
            all_errs.push(invalid(
                &path.child("failureThreshold"),
                BadValue::Int(failure_threshold.into()),
                "must be non-negative",
            ));
        }
    }

    // Validate terminationGracePeriodSeconds is positive if set
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

/// Validates a probe handler.
///
/// Exactly one handler type must be specified: exec, httpGet, tcpSocket, or grpc.
/// Note: sleep is not allowed in probe handlers (only in lifecycle handlers).
fn validate_probe_handler(
    handler: &ProbeHandler,
    _grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_handlers = 0;

    // Exec handler
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

    // HTTPGet handler
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

    // TCPSocket handler
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

    // GRPC handler
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

    // Must have exactly one handler
    if num_handlers == 0 {
        all_errs.push(required(path, "must specify a handler type"));
    }

    all_errs
}

// ============================================================================
// Lifecycle Validation Functions
// ============================================================================

/// Validates a lifecycle configuration.
///
/// Validates postStart and preStop handlers if present.
pub fn validate_lifecycle(
    lifecycle: Option<&Lifecycle>,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if let Some(lc) = lifecycle {
        // Validate postStart handler
        if let Some(ref post_start) = lc.post_start {
            all_errs.extend(validate_lifecycle_handler(
                post_start,
                grace_period,
                &path.child("postStart"),
            ));
        }

        // Validate preStop handler
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

/// Validates a lifecycle handler.
///
/// Exactly one handler type must be specified: exec, httpGet, tcpSocket, or sleep.
fn validate_lifecycle_handler(
    handler: &LifecycleHandler,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let mut num_handlers = 0;

    // Exec handler
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

    // HTTPGet handler
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

    // TCPSocket handler
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

    // Sleep handler (lifecycle only)
    if let Some(ref sleep) = handler.sleep {
        if num_handlers > 0 {
            all_errs.push(forbidden(
                &path.child("sleep"),
                "may not specify more than 1 handler type",
            ));
        } else {
            num_handlers += 1;
            all_errs.extend(validate_sleep_action(
                sleep,
                grace_period,
                &path.child("sleep"),
            ));
        }
    }

    // Must have exactly one handler
    if num_handlers == 0 {
        all_errs.push(required(path, "must specify a handler type"));
    }

    all_errs
}

// ============================================================================
// Handler Action Validation Functions
// ============================================================================

/// Validates an ExecAction.
///
/// The command field must not be empty.
fn validate_exec_action(exec: &ExecAction, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if exec.command.is_empty() {
        all_errs.push(required(&path.child("command"), "command is required"));
    }

    all_errs
}

/// Validates an HTTPGetAction.
///
/// Validates:
/// - path is required (non-empty)
/// - port is valid (number or name)
/// - scheme is supported (HTTP or HTTPS)
/// - HTTP headers are valid
fn validate_http_get_action(http: &HTTPGetAction, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Path is required
    if http.path.is_empty() {
        all_errs.push(required(&path.child("path"), "path is required"));
    }

    // Validate port
    all_errs.extend(validate_port_num_or_name(&http.port, &path.child("port")));

    // Validate scheme
    if let Some(ref scheme) = http.scheme {
        if !SUPPORTED_HTTP_SCHEMES.contains(scheme.as_str()) {
            all_errs.push(not_supported(
                &path.child("scheme"),
                BadValue::String(scheme.clone()),
                SUPPORTED_HTTP_SCHEMES_ARRAY,
            ));
        }
    }

    // Validate HTTP headers
    for (i, header) in http.http_headers.iter().enumerate() {
        let header_path = path.child("httpHeaders").index(i);

        // Validate header name using a simple check
        if header.name.is_empty() {
            all_errs.push(required(
                &header_path.child("name"),
                "header name is required",
            ));
        } else if !is_valid_http_header_name(&header.name) {
            all_errs.push(invalid(
                &header_path.child("name"),
                BadValue::String(header.name.clone()),
                "invalid HTTP header name",
            ));
        }
    }

    all_errs
}

/// Validates a TCPSocketAction.
///
/// The port must be valid (number or name).
fn validate_tcp_socket_action(tcp: &TCPSocketAction, path: &Path) -> ErrorList {
    validate_port_num_or_name(&tcp.port, &path.child("port"))
}

/// Validates a GRPCAction.
///
/// The port must be valid.
fn validate_grpc_action(grpc: &GRPCAction, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // Validate port (must be positive)
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

/// Validates a SleepAction.
///
/// The seconds field must be:
/// - Non-negative
/// - Less than terminationGracePeriodSeconds
fn validate_sleep_action(
    sleep: &SleepAction,
    grace_period: &Option<i64>,
    path: &Path,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    // If grace period is set, validate against it
    if let Some(grace) = grace_period {
        if sleep.seconds < 0 || sleep.seconds > *grace {
            all_errs.push(invalid(
                path,
                BadValue::Int(sleep.seconds),
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

/// Validates an HTTP header name.
///
/// A valid HTTP header name:
/// - Must not be empty
/// - Must contain only alphanumeric characters, hyphens, and underscores
/// - Must not start or end with a hyphen
fn is_valid_http_header_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Check for valid characters
    for c in name.chars() {
        if !c.is_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }

    // Must not start or end with hyphen
    if name.starts_with('-') || name.ends_with('-') {
        return false;
    }

    true
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::util::IntOrString;
    use crate::common::validation::ErrorType;

    #[test]
    fn test_validate_liveness_probe_success_threshold() {
        // Liveness probe with successThreshold != 1 should fail
        let probe = Probe {
            probe_handler: ProbeHandler {
                exec: Some(ExecAction {
                    command: vec!["echo".to_string()],
                }),
                ..Default::default()
            },
            success_threshold: Some(2), // Invalid for liveness probe
            ..Default::default()
        };

        let errs = validate_liveness_probe(Some(&probe), &Some(30), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("successThreshold") && e.detail.contains("must be 1"))
        );
    }

    #[test]
    fn test_validate_readiness_probe_termination_grace_period() {
        // Readiness probe with terminationGracePeriodSeconds should fail
        let probe = Probe {
            probe_handler: ProbeHandler {
                exec: Some(ExecAction {
                    command: vec!["echo".to_string()],
                }),
                ..Default::default()
            },
            termination_grace_period_seconds: Some(10), // Invalid for readiness probe
            ..Default::default()
        };

        let errs = validate_readiness_probe(Some(&probe), &Some(30), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("terminationGracePeriodSeconds")
                    && e.detail.contains("must not be set"))
        );
    }

    #[test]
    fn test_validate_probe_handler_multiple_types() {
        // Probe with multiple handlers should fail
        let handler = ProbeHandler {
            exec: Some(ExecAction {
                command: vec!["echo".to_string()],
            }),
            http_get: Some(HTTPGetAction {
                path: "/health".to_string(),
                port: IntOrString::Int(8080),
                ..Default::default()
            }),
            ..Default::default()
        };

        let errs = validate_probe_handler(&handler, &Some(30), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("may not specify more than 1"))
        );
    }

    #[test]
    fn test_validate_probe_handler_no_handler() {
        // Probe with no handler should fail
        let handler = ProbeHandler::default();

        let errs = validate_probe_handler(&handler, &Some(30), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("must specify a handler type"))
        );
    }

    #[test]
    fn test_validate_exec_action_empty_command() {
        let exec = ExecAction {
            command: vec![], // Empty command is invalid
        };

        let errs = validate_exec_action(&exec, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("command is required"))
        );
    }

    #[test]
    fn test_validate_http_get_action_empty_path() {
        let http = HTTPGetAction {
            path: String::new(), // Empty path is invalid
            port: IntOrString::Int(8080),
            ..Default::default()
        };

        let errs = validate_http_get_action(&http, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("path is required"))
        );
    }

    #[test]
    fn test_validate_http_get_action_invalid_scheme() {
        let http = HTTPGetAction {
            path: "/health".to_string(),
            port: IntOrString::Int(8080),
            scheme: Some("FTP".to_string()), // Invalid scheme
            ..Default::default()
        };

        let errs = validate_http_get_action(&http, &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.field.contains("scheme") && e.error_type == ErrorType::NotSupported)
        );
    }

    #[test]
    fn test_validate_grpc_action_invalid_port() {
        let grpc = GRPCAction {
            port: -1, // Invalid port
            service: None,
        };

        let errs = validate_grpc_action(&grpc, &Path::nil());
        assert!(!errs.is_empty());
        assert!(errs.errors.iter().any(|e| e.detail.contains("port")));
    }

    #[test]
    fn test_validate_sleep_action_exceeds_grace_period() {
        let sleep = SleepAction {
            seconds: 100, // Exceeds grace period
        };

        let errs = validate_sleep_action(&sleep, &Some(30), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("terminationGracePeriodSeconds"))
        );
    }

    #[test]
    fn test_validate_lifecycle_multiple_handlers() {
        // Lifecycle handler with multiple types should fail
        let handler = LifecycleHandler {
            exec: Some(ExecAction {
                command: vec!["echo".to_string()],
            }),
            sleep: Some(SleepAction { seconds: 10 }),
            ..Default::default()
        };

        let errs = validate_lifecycle_handler(&handler, &Some(30), &Path::nil());
        assert!(!errs.is_empty());
        assert!(
            errs.errors
                .iter()
                .any(|e| e.detail.contains("may not specify more than 1"))
        );
    }

    #[test]
    fn test_is_valid_http_header_name() {
        assert!(is_valid_http_header_name("Content-Type"));
        assert!(is_valid_http_header_name("X-Custom-Header"));
        assert!(is_valid_http_header_name("Accept"));

        assert!(!is_valid_http_header_name(""));
        assert!(!is_valid_http_header_name("-Invalid"));
        assert!(!is_valid_http_header_name("Invalid-"));
        assert!(!is_valid_http_header_name("Invalid Header")); // Space is not allowed
        assert!(!is_valid_http_header_name("Invalid@Header")); // @ is not allowed
    }

    #[test]
    fn test_validate_probe_negative_values() {
        let probe = Probe {
            probe_handler: ProbeHandler {
                exec: Some(ExecAction {
                    command: vec!["echo".to_string()],
                }),
                ..Default::default()
            },
            initial_delay_seconds: Some(-1),
            timeout_seconds: Some(-1),
            period_seconds: Some(-1),
            success_threshold: Some(-1),
            failure_threshold: Some(-1),
            termination_grace_period_seconds: Some(-1),
        };

        let errs = validate_probe(&probe, &Some(30), &Path::nil());
        assert!(errs.len() >= 6); // Should have errors for all negative fields
    }
}
