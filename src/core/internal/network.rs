//! Network-related types for Kubernetes health checks and probes
//!
//! This module contains types for network-based health checks used in liveness and readiness probes.

use crate::common::util::IntOrString;
use crate::core::internal::URIScheme;
use serde::{Deserialize, Serialize};

/// TCPSocketAction describes an action based on opening a TCP socket.
///
/// Corresponds to [Kubernetes TCPSocketAction](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2424)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TCPSocketAction {
    /// Port number or name to connect to.
    pub port: IntOrString,
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
}

/// HTTPGetAction describes an action based on HTTP Get requests.
///
/// Corresponds to [Kubernetes HTTPGetAction](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2394)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPGetAction {
    /// Optional: Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Port number or name to access on the container.
    pub port: IntOrString,
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    /// Optional: Scheme to use for connecting to the host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<URIScheme>,
    /// Optional: Custom headers to set in the request.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_headers: Vec<HTTPHeader>,
}

/// HTTPHeader describes a custom header to use in HTTP probes.
///
/// Corresponds to [Kubernetes HTTPHeader](https://github.com/kubernetes/api/blob/master/core/v1/types.go#L2385)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HTTPHeader {
    /// The header field name.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The header field value.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_socket_action_default() {
        let action = TCPSocketAction::default();
        assert_eq!(action.host, "");
    }

    #[test]
    fn test_tcp_socket_action_with_numeric_port() {
        let action = TCPSocketAction {
            port: IntOrString::Int(8080),
            host: "localhost".to_string(),
        };

        assert_eq!(action.port, IntOrString::Int(8080));
        assert_eq!(action.host, "localhost");
    }

    #[test]
    fn test_tcp_socket_action_with_named_port() {
        let action = TCPSocketAction {
            port: IntOrString::String("http".to_string()),
            host: String::new(),
        };

        assert_eq!(action.port, IntOrString::String("http".to_string()));
        assert!(action.host.is_empty());
    }

    #[test]
    fn test_tcp_socket_action_serialize() {
        let action = TCPSocketAction {
            port: IntOrString::Int(443),
            host: String::new(),
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"port\":443"));
        // empty host should be omitted
        assert!(!json.contains("\"host\""));
    }

    #[test]
    fn test_tcp_socket_action_deserialize() {
        let json = r#"{"port":80,"host":"example.com"}"#;
        let action: TCPSocketAction = serde_json::from_str(json).unwrap();

        assert_eq!(action.port, IntOrString::Int(80));
        assert_eq!(action.host, "example.com");
    }

    #[test]
    fn test_tcp_socket_action_round_trip() {
        let original = TCPSocketAction {
            port: IntOrString::String("https".to_string()),
            host: "api.example.com".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TCPSocketAction = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_http_get_action_default() {
        let action = HTTPGetAction::default();
        assert!(action.path.is_empty());
        assert!(action.host.is_empty());
        assert!(action.scheme.is_none());
        assert!(action.http_headers.is_empty());
    }

    #[test]
    fn test_http_get_action_with_fields() {
        let action = HTTPGetAction {
            path: "/healthz".to_string(),
            port: IntOrString::Int(8080),
            host: "localhost".to_string(),
            scheme: Some(URIScheme::Http),
            http_headers: vec![],
        };

        assert_eq!(action.path, "/healthz");
        assert_eq!(action.port, IntOrString::Int(8080));
        assert_eq!(action.host, "localhost");
        assert_eq!(action.scheme, Some(URIScheme::Http));
    }

    #[test]
    fn test_http_get_action_serialize() {
        let action = HTTPGetAction {
            path: "/metrics".to_string(),
            port: IntOrString::Int(9090),
            host: String::new(),
            scheme: None,
            http_headers: vec![],
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"path\":\"/metrics\""));
        assert!(json.contains("\"port\":9090"));
        // empty fields should be omitted
        assert!(!json.contains("\"host\""));
        assert!(!json.contains("\"scheme\""));
        assert!(!json.contains("\"httpHeaders\""));
    }

    #[test]
    fn test_http_get_action_with_headers() {
        let headers = vec![
            HTTPHeader {
                name: "Authorization".to_string(),
                value: "Bearer token".to_string(),
            },
            HTTPHeader {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            },
        ];

        let action = HTTPGetAction {
            path: "/api".to_string(),
            port: IntOrString::String("http".to_string()),
            host: String::new(),
            scheme: Some(URIScheme::Https),
            http_headers: headers,
        };

        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("\"httpHeaders\""));
        assert!(json.contains("\"Authorization\""));
        assert!(json.contains("\"scheme\":\"HTTPS\""));
    }

    #[test]
    fn test_http_get_action_deserialize() {
        let json = r#"{"path":"/health","port":8080,"scheme":"HTTP"}"#;
        let action: HTTPGetAction = serde_json::from_str(json).unwrap();

        assert_eq!(action.path, "/health");
        assert_eq!(action.port, IntOrString::Int(8080));
        assert_eq!(action.scheme, Some(URIScheme::Http));
    }

    #[test]
    fn test_http_get_action_round_trip() {
        let original = HTTPGetAction {
            path: "/status".to_string(),
            port: IntOrString::Int(443),
            host: "example.com".to_string(),
            scheme: Some(URIScheme::Https),
            http_headers: vec![HTTPHeader {
                name: "X-Custom".to_string(),
                value: "value".to_string(),
            }],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: HTTPGetAction = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_http_header_default() {
        let header = HTTPHeader::default();
        assert!(header.name.is_empty());
        assert!(header.value.is_empty());
    }

    #[test]
    fn test_http_header_with_values() {
        let header = HTTPHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        };

        assert_eq!(header.name, "Content-Type");
        assert_eq!(header.value, "application/json");
    }

    #[test]
    fn test_http_header_serialize() {
        let header = HTTPHeader {
            name: "Authorization".to_string(),
            value: "Bearer token123".to_string(),
        };

        let json = serde_json::to_string(&header).unwrap();
        assert!(json.contains("\"name\":\"Authorization\""));
        assert!(json.contains("\"value\":\"Bearer token123\""));
    }

    #[test]
    fn test_http_header_round_trip() {
        let original = HTTPHeader {
            name: "X-API-Key".to_string(),
            value: "secret-key".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: HTTPHeader = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_uri_scheme_serialize() {
        let json_http = serde_json::to_string(&URIScheme::Http).unwrap();
        assert_eq!(json_http, r#""HTTP""#);

        let json_https = serde_json::to_string(&URIScheme::Https).unwrap();
        assert_eq!(json_https, r#""HTTPS""#);
    }

    #[test]
    fn test_uri_scheme_deserialize() {
        let http: URIScheme = serde_json::from_str(r#""HTTP""#).unwrap();
        assert_eq!(http, URIScheme::Http);

        let https: URIScheme = serde_json::from_str(r#""HTTPS""#).unwrap();
        assert_eq!(https, URIScheme::Https);
    }
}
