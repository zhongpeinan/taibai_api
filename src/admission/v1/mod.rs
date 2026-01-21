//! Kubernetes Admission API v1 types
//!
//! This module contains the admission v1 API types.

use crate::authentication::v1::UserInfo;
use crate::common::{GroupVersionKind, GroupVersionResource, Status, TypeMeta};
use crate::core::internal::ByteString;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// PatchType is the type of patch being used to represent the mutated object.
///
/// Corresponds to [Kubernetes PatchType](https://github.com/kubernetes/api/blob/master/admission/v1/types.go#L155)
pub type PatchType = String;

/// PatchType constants.
pub mod patch_type {
    /// JSONPatch is the only supported patch type
    pub const JSON_PATCH: &str = "JSONPatch";
}

/// Operation is the type of resource operation being checked for admission control.
///
/// Corresponds to [Kubernetes Operation](https://github.com/kubernetes/api/blob/master/admission/v1/types.go#L163)
pub type Operation = String;

/// Operation constants
pub mod operation {
    /// CREATE operation
    pub const CREATE: &str = "CREATE";
    /// UPDATE operation
    pub const UPDATE: &str = "UPDATE";
    /// DELETE operation
    pub const DELETE: &str = "DELETE";
    /// CONNECT operation
    pub const CONNECT: &str = "CONNECT";
}

/// AdmissionReview describes an admission review request/response.
///
/// Corresponds to [Kubernetes AdmissionReview](https://github.com/kubernetes/api/blob/master/admission/v1/types.go#L29)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionReview {
    #[serde(flatten)]
    pub type_meta: TypeMeta,
    /// Request describes the attributes for the admission request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<AdmissionRequest>,
    /// Response describes the attributes for the admission response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<AdmissionResponse>,
}

/// AdmissionRequest describes the admission.Attributes for the admission request.
///
/// Corresponds to [Kubernetes AdmissionRequest](https://github.com/kubernetes/api/blob/master/admission/v1/types.go#L40)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionRequest {
    /// UID is an identifier for the individual request/response.
    pub uid: String,
    /// Kind is the fully-qualified type of object being submitted.
    pub kind: GroupVersionKind,
    /// Resource is the fully-qualified resource being requested.
    pub resource: GroupVersionResource,
    /// SubResource is the subresource being requested, if any.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sub_resource: String,
    /// RequestKind is the fully-qualified type of the original API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_kind: Option<Box<GroupVersionKind>>,
    /// RequestResource is the fully-qualified resource of the original API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_resource: Option<Box<GroupVersionResource>>,
    /// RequestSubResource is the name of the subresource of the original API request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request_sub_resource: String,
    /// Name is the name of the object as presented in the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Namespace is the namespace associated with the request.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    /// Operation is the operation being performed.
    pub operation: Operation,
    /// UserInfo is information about the requesting user.
    #[serde(rename = "userInfo")]
    pub user_info: UserInfo,
    /// Object is the object from the incoming request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<Value>,
    /// OldObject is the existing object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<Value>,
    /// DryRun indicates that modifications will definitely not be persisted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Options is the operation option structure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

/// AdmissionResponse describes an admission response.
///
/// Corresponds to [Kubernetes AdmissionResponse](https://github.com/kubernetes/api/blob/master/admission/v1/types.go#L116)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmissionResponse {
    /// UID is an identifier for the individual request/response.
    pub uid: String,
    /// Allowed indicates whether or not the admission request was permitted.
    pub allowed: bool,
    /// Result contains extra details into why an admission request was denied.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Status>,
    /// Patch is the patch body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<ByteString>,
    /// PatchType is the type of Patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_type: Option<Box<PatchType>>,
    /// AuditAnnotations is an unstructured key value map.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub audit_annotations: BTreeMap<String, String>,
    /// Warnings is a list of warning messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{ListMeta, StatusCause, StatusDetails};

    #[test]
    fn test_patch_type_constants() {
        assert_eq!(patch_type::JSON_PATCH, "JSONPatch");
    }

    #[test]
    fn test_operation_constants() {
        assert_eq!(operation::CREATE, "CREATE");
        assert_eq!(operation::UPDATE, "UPDATE");
        assert_eq!(operation::DELETE, "DELETE");
        assert_eq!(operation::CONNECT, "CONNECT");
    }

    #[test]
    fn test_admission_review_default() {
        let review = AdmissionReview::default();
        assert!(review.request.is_none());
        assert!(review.response.is_none());
    }

    #[test]
    fn test_admission_request_default() {
        let req = AdmissionRequest::default();
        assert!(req.uid.is_empty());
        assert!(req.operation.is_empty());
        assert!(req.name.is_empty());
    }

    #[test]
    fn test_admission_response_default() {
        let resp = AdmissionResponse::default();
        assert!(resp.uid.is_empty());
        assert!(!resp.allowed);
        assert!(resp.warnings.is_empty());
    }

    #[test]
    fn test_admission_request_with_fields() {
        let req = AdmissionRequest {
            uid: "test-uid".to_string(),
            operation: operation::CREATE.to_string(),
            name: "my-pod".to_string(),
            namespace: "default".to_string(),
            ..Default::default()
        };
        assert_eq!(req.uid, "test-uid");
        assert_eq!(req.operation, "CREATE");
    }

    #[test]
    fn test_admission_response_allowed() {
        let resp = AdmissionResponse {
            uid: "req-uid".to_string(),
            allowed: true,
            ..Default::default()
        };
        assert!(resp.allowed);
        assert_eq!(resp.uid, "req-uid");
    }

    #[test]
    fn test_admission_response_denied_with_status() {
        let resp = AdmissionResponse {
            uid: "req-uid".to_string(),
            allowed: false,
            result: Some(Status {
                status: Some("Failure".to_string()),
                message: Some("Access denied".to_string()),
                code: Some(403),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(!resp.allowed);
        assert!(resp.result.is_some());
    }

    #[test]
    fn test_admission_request_serialize() {
        let req = AdmissionRequest {
            uid: "abc-123".to_string(),
            operation: operation::CREATE.to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"uid\":\"abc-123\""));
        assert!(json.contains("\"operation\":\"CREATE\""));
    }

    #[test]
    fn test_admission_response_serialize() {
        let resp = AdmissionResponse {
            uid: "xyz-789".to_string(),
            allowed: true,
            ..Default::default()
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"uid\":\"xyz-789\""));
        assert!(json.contains("\"allowed\":true"));
    }

    #[test]
    fn test_admission_review_serialize() {
        let review = AdmissionReview {
            request: Some(AdmissionRequest {
                uid: "test-uid".to_string(),
                operation: operation::UPDATE.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&review).unwrap();
        assert!(json.contains("\"request\""));
    }

    #[test]
    fn test_admission_response_with_warnings() {
        let resp = AdmissionResponse {
            uid: "req-1".to_string(),
            allowed: true,
            warnings: vec!["Field deprecated".to_string(), "Use new API".to_string()],
            ..Default::default()
        };
        assert_eq!(resp.warnings.len(), 2);
    }

    #[test]
    fn test_admission_response_with_audit_annotations() {
        let mut annotations = BTreeMap::new();
        annotations.insert("key".to_string(), "value".to_string());

        let resp = AdmissionResponse {
            uid: "req-2".to_string(),
            allowed: true,
            audit_annotations: annotations,
            ..Default::default()
        };
        assert_eq!(resp.audit_annotations.len(), 1);
    }

    #[test]
    fn test_admission_response_with_patch() {
        let resp = AdmissionResponse {
            uid: "req-3".to_string(),
            allowed: true,
            patch: Some(ByteString(vec![1, 2, 3])),
            patch_type: Some(Box::new(patch_type::JSON_PATCH.to_string())),
            ..Default::default()
        };
        assert!(resp.patch.is_some());
        assert!(resp.patch_type.is_some());
    }

    #[test]
    fn test_group_version_kind() {
        let gvk = GroupVersionKind {
            group: "apps".to_string(),
            version: "v1".to_string(),
            kind: "Deployment".to_string(),
        };
        let json = serde_json::to_string(&gvk).unwrap();
        assert!(json.contains("\"group\":\"apps\""));
        assert!(json.contains("\"version\":\"v1\""));
        assert!(json.contains("\"kind\":\"Deployment\""));
    }

    #[test]
    fn test_admission_request_with_user_info() {
        let req = AdmissionRequest {
            uid: "test-uid".to_string(),
            operation: operation::CREATE.to_string(),
            user_info: UserInfo {
                username: "admin".to_string(),
                uid: "user-123".to_string(),
                groups: vec!["system:masters".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(req.user_info.username, "admin");
        assert_eq!(req.user_info.groups.len(), 1);
    }

    #[test]
    fn test_admission_request_round_trip() {
        let original = AdmissionRequest {
            uid: "test-uid".to_string(),
            operation: operation::DELETE.to_string(),
            name: "my-resource".to_string(),
            namespace: "default".to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AdmissionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(original.uid, deserialized.uid);
        assert_eq!(original.operation, deserialized.operation);
        assert_eq!(original.name, deserialized.name);
    }

    #[test]
    fn test_empty_fields_omitted() {
        let req = AdmissionRequest {
            uid: "test".to_string(),
            operation: operation::CREATE.to_string(),
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"uid\":\"test\""));
        assert!(json.contains("\"operation\":\"CREATE\""));
        // Empty fields should be omitted
        assert!(!json.contains("\"name\":\"\""));
        assert!(!json.contains("\"namespace\":\"\""));
    }

    /// Test deserialization of actual Kubernetes AdmissionReview JSON from testdata.
    /// This test ensures compatibility with real Kubernetes API responses.
    #[test]
    fn test_admission_review_from_testdata() {
        let json = r#"{
  "kind": "AdmissionReview",
  "apiVersion": "admission.k8s.io/v1",
  "request": {
    "uid": "uidValue",
    "kind": {
      "group": "groupValue",
      "version": "versionValue",
      "kind": "kindValue"
    },
    "resource": {
      "group": "groupValue",
      "version": "versionValue",
      "resource": "resourceValue"
    },
    "subResource": "subResourceValue",
    "requestKind": {
      "group": "groupValue",
      "version": "versionValue",
      "kind": "kindValue"
    },
    "requestResource": {
      "group": "groupValue",
      "version": "versionValue",
      "resource": "resourceValue"
    },
    "requestSubResource": "requestSubResourceValue",
    "name": "nameValue",
    "namespace": "namespaceValue",
    "operation": "operationValue",
    "userInfo": {
      "username": "usernameValue",
      "uid": "uidValue",
      "groups": [
        "groupsValue"
      ],
      "extra": {
        "extraKey": [
          "extraValue"
        ]
      }
    },
    "object": {
      "apiVersion": "example.com/v1",
      "kind": "CustomType",
      "spec": {
        "replicas": 1
      },
      "status": {
        "available": 1
      }
    },
    "oldObject": {
      "apiVersion": "example.com/v1",
      "kind": "CustomType",
      "spec": {
        "replicas": 1
      },
      "status": {
        "available": 1
      }
    },
    "dryRun": true,
    "options": {
      "apiVersion": "example.com/v1",
      "kind": "CustomType",
      "spec": {
        "replicas": 1
      },
      "status": {
        "available": 1
      }
    }
  },
  "response": {
    "uid": "uidValue",
    "allowed": true,
    "status": {
      "metadata": {
        "selfLink": "selfLinkValue",
        "resourceVersion": "resourceVersionValue",
        "continue": "continueValue",
        "remainingItemCount": 4
      },
      "status": "statusValue",
      "message": "messageValue",
      "reason": "reasonValue",
      "details": {
        "name": "nameValue",
        "group": "groupValue",
        "kind": "kindValue",
        "uid": "uidValue",
        "causes": [
          {
            "reason": "reasonValue",
            "message": "messageValue",
            "field": "fieldValue"
          }
        ],
        "retryAfterSeconds": 5
      },
      "code": 6
    },
    "patch": "BA==",
    "patchType": "patchTypeValue",
    "auditAnnotations": {
      "auditAnnotationsKey": "auditAnnotationsValue"
    },
    "warnings": [
      "warningsValue"
    ]
  }
}"#;

        let review: AdmissionReview = serde_json::from_str(json).unwrap();

        // Verify TypeMeta
        assert_eq!(review.type_meta.kind, "AdmissionReview");
        assert_eq!(review.type_meta.api_version, "admission.k8s.io/v1");

        // Verify request
        assert!(review.request.is_some());
        let req = review.request.as_ref().unwrap();
        assert_eq!(req.uid, "uidValue");
        assert_eq!(req.kind.group, "groupValue");
        assert_eq!(req.kind.version, "versionValue");
        assert_eq!(req.kind.kind, "kindValue");
        assert_eq!(req.resource.group, "groupValue");
        assert_eq!(req.resource.version, "versionValue");
        assert_eq!(req.resource.resource, "resourceValue");
        assert_eq!(req.sub_resource, "subResourceValue");
        assert!(req.request_kind.is_some());
        assert_eq!(req.request_kind.as_ref().unwrap().kind, "kindValue");
        assert!(req.request_resource.is_some());
        assert_eq!(
            req.request_resource.as_ref().unwrap().resource,
            "resourceValue"
        );
        assert_eq!(req.request_sub_resource, "requestSubResourceValue");
        assert_eq!(req.name, "nameValue");
        assert_eq!(req.namespace, "namespaceValue");
        assert_eq!(req.operation, "operationValue");
        assert_eq!(req.user_info.username, "usernameValue");
        assert_eq!(req.user_info.uid, "uidValue");
        assert_eq!(req.user_info.groups.len(), 1);
        assert_eq!(req.user_info.groups[0], "groupsValue");
        assert_eq!(req.user_info.extra.len(), 1);
        assert!(req.user_info.extra.contains_key("extraKey"));
        assert!(req.object.is_some());
        assert!(req.old_object.is_some());
        assert_eq!(req.dry_run, Some(true));
        assert!(req.options.is_some());

        // Verify response
        assert!(review.response.is_some());
        let resp = review.response.as_ref().unwrap();
        assert_eq!(resp.uid, "uidValue");
        assert_eq!(resp.allowed, true);
        assert!(resp.result.is_some());
        let status = resp.result.as_ref().unwrap();
        assert!(status.metadata.is_some());
        assert_eq!(status.status, Some("statusValue".to_string()));
        assert_eq!(status.message, Some("messageValue".to_string()));
        assert_eq!(status.reason, Some("reasonValue".to_string()));
        assert!(status.details.is_some());
        let details = status.details.as_ref().unwrap();
        assert_eq!(details.name, "nameValue");
        assert_eq!(details.group, "groupValue");
        assert_eq!(details.kind, "kindValue");
        assert_eq!(details.uid, "uidValue");
        assert_eq!(details.causes.len(), 1);
        assert_eq!(details.causes[0].reason, "reasonValue");
        assert_eq!(details.causes[0].message, "messageValue");
        assert_eq!(details.causes[0].field, "fieldValue");
        assert_eq!(details.retry_after_seconds, Some(5));
        assert_eq!(status.code, Some(6));

        assert!(resp.patch.is_some());
        assert_eq!(resp.patch.as_ref().unwrap().0, vec![4]); // "BA==" base64 decodes to [4]
        assert!(resp.patch_type.is_some());
        assert_eq!(
            *resp.patch_type.as_ref().unwrap(),
            Box::new("patchTypeValue".to_string())
        );
        assert_eq!(resp.audit_annotations.len(), 1);
        assert_eq!(
            resp.audit_annotations.get("auditAnnotationsKey"),
            Some(&"auditAnnotationsValue".to_string())
        );
        assert_eq!(resp.warnings.len(), 1);
        assert_eq!(resp.warnings[0], "warningsValue");
    }

    /// Test serialization of AdmissionReview matches expected JSON format.
    #[test]
    fn test_admission_review_serialization_format() {
        let review = AdmissionReview {
            type_meta: TypeMeta {
                kind: "AdmissionReview".to_string(),
                api_version: "admission.k8s.io/v1".to_string(),
            },
            request: Some(AdmissionRequest {
                uid: "test-uid".to_string(),
                kind: GroupVersionKind {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    kind: "Deployment".to_string(),
                },
                resource: GroupVersionResource {
                    group: "apps".to_string(),
                    version: "v1".to_string(),
                    resource: "deployments".to_string(),
                },
                operation: operation::CREATE.to_string(),
                user_info: UserInfo {
                    username: "admin".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            response: None,
        };

        let json = serde_json::to_string_pretty(&review).unwrap();

        // Verify the JSON structure matches Kubernetes format
        assert!(json.contains("\"kind\": \"AdmissionReview\""));
        assert!(json.contains("\"apiVersion\": \"admission.k8s.io/v1\""));
        assert!(json.contains("\"request\":"));
        assert!(json.contains("\"uid\": \"test-uid\""));
        assert!(json.contains("\"operation\": \"CREATE\""));
        assert!(json.contains("\"userInfo\":"));
        assert!(json.contains("\"username\": \"admin\""));
    }

    /// Test round-trip serialization/deserialization of complete AdmissionReview.
    #[test]
    fn test_admission_review_round_trip() {
        let original = AdmissionReview {
            type_meta: TypeMeta {
                kind: "AdmissionReview".to_string(),
                api_version: "admission.k8s.io/v1".to_string(),
            },
            request: Some(AdmissionRequest {
                uid: "round-trip-uid".to_string(),
                kind: GroupVersionKind {
                    group: "".to_string(),
                    version: "v1".to_string(),
                    kind: "Pod".to_string(),
                },
                resource: GroupVersionResource {
                    group: "".to_string(),
                    version: "v1".to_string(),
                    resource: "pods".to_string(),
                },
                name: "test-pod".to_string(),
                namespace: "default".to_string(),
                operation: operation::CREATE.to_string(),
                user_info: UserInfo {
                    username: "system:admin".to_string(),
                    groups: vec!["system:masters".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            }),
            response: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AdmissionReview = serde_json::from_str(&json).unwrap();

        assert_eq!(original.type_meta.kind, deserialized.type_meta.kind);
        assert_eq!(
            original.type_meta.api_version,
            deserialized.type_meta.api_version
        );

        let orig_req = original.request.as_ref().unwrap();
        let deser_req = deserialized.request.as_ref().unwrap();
        assert_eq!(orig_req.uid, deser_req.uid);
        assert_eq!(orig_req.kind.kind, deser_req.kind.kind);
        assert_eq!(orig_req.operation, deser_req.operation);
    }

    /// Test Status with all fields populated.
    #[test]
    fn test_status_with_all_fields() {
        let status = Status {
            metadata: Some(ListMeta {
                self_link: Some("/api/v1/namespaces/default/pods/test".to_string()),
                resource_version: Some("12345".to_string()),
                ..Default::default()
            }),
            status: Some("Failure".to_string()),
            message: Some("Invalid pod configuration".to_string()),
            reason: Some("Invalid".to_string()),
            details: Some(StatusDetails {
                name: "test-pod".to_string(),
                group: "".to_string(),
                kind: "Pod".to_string(),
                uid: "abc-123".to_string(),
                causes: vec![StatusCause {
                    reason: "FieldValueInvalid".to_string(),
                    message: "Invalid value".to_string(),
                    field: "spec.containers[0].image".to_string(),
                }],
                retry_after_seconds: Some(5),
            }),
            code: Some(400),
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"Failure\""));
        assert!(json.contains("\"message\":\"Invalid pod configuration\""));
        assert!(json.contains("\"reason\":\"Invalid\""));
        assert!(json.contains("\"code\":400"));

        // Round-trip test
        let deserialized: Status = serde_json::from_str(&json).unwrap();
        assert_eq!(status.status, deserialized.status);
        assert_eq!(status.message, deserialized.message);
        assert_eq!(status.code, deserialized.code);
        assert!(deserialized.details.is_some());
    }

    /// Test StatusDetails with causes array.
    #[test]
    fn test_status_details_with_causes() {
        let details = StatusDetails {
            name: "test-resource".to_string(),
            group: "apps".to_string(),
            kind: "Deployment".to_string(),
            uid: "resource-uid".to_string(),
            causes: vec![
                StatusCause {
                    reason: "Reason1".to_string(),
                    message: "Message 1".to_string(),
                    field: "field1".to_string(),
                },
                StatusCause {
                    reason: "Reason2".to_string(),
                    message: "Message 2".to_string(),
                    field: "field2".to_string(),
                },
            ],
            retry_after_seconds: Some(10),
        };

        assert_eq!(details.causes.len(), 2);
        assert_eq!(details.causes[0].reason, "Reason1");
        assert_eq!(details.causes[1].reason, "Reason2");

        // Test serialization
        let json = serde_json::to_string(&details).unwrap();
        assert!(json.contains("\"causes\""));
        assert!(json.contains("\"Reason1\""));
    }

    /// Test AdmissionResponse with complete Status including details.
    #[test]
    fn test_admission_response_with_complete_status() {
        let response = AdmissionResponse {
            uid: "req-uid".to_string(),
            allowed: false,
            result: Some(Status {
                status: Some("Failure".to_string()),
                message: Some("Admission denied".to_string()),
                reason: Some("Forbidden".to_string()),
                details: Some(StatusDetails {
                    name: "my-resource".to_string(),
                    kind: "Pod".to_string(),
                    causes: vec![StatusCause {
                        reason: "Forbidden".to_string(),
                        message: "User cannot create pods".to_string(),
                        field: "".to_string(),
                    }],
                    ..Default::default()
                }),
                code: Some(403),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert!(!response.allowed);
        assert!(response.result.is_some());
        let status = response.result.as_ref().unwrap();
        assert_eq!(status.code, Some(403));
        assert!(status.details.is_some());
        assert_eq!(status.details.as_ref().unwrap().causes.len(), 1);
    }

    /// Test deserialization of response with patch (base64 encoded).
    #[test]
    fn test_admission_response_with_patch_base64() {
        let json = r#"{
            "uid": "test-uid",
            "allowed": true,
            "patch": "W3sib3AiOiJhZGQiLCJwYXRoIjoiL3NwZWMvcmVwbGljYXMiLCJ2YWx1ZSI6M31d",
            "patchType": "JSONPatch"
        }"#;

        let response: AdmissionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.uid, "test-uid");
        assert!(response.patch.is_some());
        assert!(response.patch_type.is_some());

        // Verify base64 decoding works correctly
        let patch_bytes = &response.patch.unwrap().0;
        assert!(!patch_bytes.is_empty());
    }
}
