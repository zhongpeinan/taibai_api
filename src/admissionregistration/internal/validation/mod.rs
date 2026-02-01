//! Validation for Kubernetes AdmissionRegistration internal API types
//!
//! Ported from k8s.io/kubernetes/pkg/apis/admissionregistration/validation/validation.go

mod mutating_webhook_configuration;
mod validating_webhook_configuration;

use std::collections::BTreeSet;

use crate::admissionregistration::internal::{MutatingWebhook, ValidatingWebhook};
use crate::admissionregistration::v1::{
    FailurePolicyType, MatchCondition, MatchPolicyType, OperationType, Rule, RuleWithOperations,
    ServiceReference, SideEffectClass, WebhookClientConfig,
};
use crate::common::validation::{
    BadValue, ErrorList, Path, duplicate, invalid, is_dns1035_label, is_dns1123_subdomain,
    not_supported, required, validate_qualified_name,
};

pub use mutating_webhook_configuration::validate_mutating_webhook_configuration;
pub use validating_webhook_configuration::validate_validating_webhook_configuration;

fn validate_webhook_configuration<T>(
    metadata: &crate::common::ObjectMeta,
    webhooks: &[T],
    webhooks_path: &Path,
    validating: bool,
) -> ErrorList
where
    T: WebhookAdapter,
{
    let mut all_errs = crate::common::validation::validate_object_meta(
        metadata,
        false,
        name_is_dns_subdomain,
        &Path::new("metadata"),
    );

    let mut names = BTreeSet::new();
    for (i, webhook) in webhooks.iter().enumerate() {
        let path = webhooks_path.index(i);
        all_errs.extend(validate_webhook(webhook, &path, validating));
        all_errs.extend(validate_admission_review_versions(
            webhook.admission_review_versions(),
            &path.child("admissionReviewVersions"),
        ));
        if !webhook.name().is_empty() && !names.insert(webhook.name().to_string()) {
            all_errs.push(duplicate(
                &path.child("name"),
                BadValue::String(webhook.name().to_string()),
            ));
        }
    }

    all_errs
}

fn validate_webhook(webhook: &dyn WebhookAdapter, path: &Path, _validating: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if webhook.name().is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else {
        all_errs.extend(validate_qualified_name(webhook.name(), &path.child("name")));
    }

    for (i, rule) in webhook.rules().iter().enumerate() {
        all_errs.extend(validate_rule_with_operations(
            rule,
            &path.child("rules").index(i),
            true,
        ));
    }

    if let Some(policy) = webhook.failure_policy() {
        if !matches!(policy, FailurePolicyType::Ignore | FailurePolicyType::Fail) {
            all_errs.push(not_supported(
                &path.child("failurePolicy"),
                BadValue::String(format!("{:?}", policy)),
                &["Ignore", "Fail"],
            ));
        }
    }

    if let Some(policy) = webhook.match_policy() {
        if !matches!(policy, MatchPolicyType::Exact | MatchPolicyType::Equivalent) {
            all_errs.push(not_supported(
                &path.child("matchPolicy"),
                BadValue::String(format!("{:?}", policy)),
                &["Exact", "Equivalent"],
            ));
        }
    }

    match webhook.side_effects() {
        None => {
            all_errs.push(required(&path.child("sideEffects"), ""));
        }
        Some(value) => {
            let value_str = format!("{:?}", value);
            if value_str != "None" && value_str != "NoneOnDryRun" {
                all_errs.push(not_supported(
                    &path.child("sideEffects"),
                    BadValue::String(value_str),
                    &["None", "NoneOnDryRun"],
                ));
            }
        }
    }

    if let Some(timeout) = webhook.timeout_seconds() {
        if !(1..=30).contains(&timeout) {
            all_errs.push(invalid(
                &path.child("timeoutSeconds"),
                BadValue::Int(timeout as i64),
                "the timeout value must be between 1 and 30 seconds",
            ));
        }
    }

    all_errs.extend(validate_webhook_client_config(
        webhook.client_config(),
        &path.child("clientConfig"),
    ));

    for (i, cond) in webhook.match_conditions().iter().enumerate() {
        all_errs.extend(validate_match_condition(
            cond,
            &path.child("matchConditions").index(i),
        ));
    }

    all_errs
}

fn validate_webhook_client_config(config: &WebhookClientConfig, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    let has_url = config.url.as_ref().map_or(false, |s| !s.is_empty());
    let has_service = config.service.is_some();
    if has_url == has_service {
        all_errs.push(required(path, "exactly one of url or service is required"));
    } else if let Some(ref url) = config.url {
        if url.is_empty() {
            all_errs.push(required(&path.child("url"), ""));
        }
    } else if let Some(ref svc) = config.service {
        all_errs.extend(validate_service_reference(svc, &path.child("service")));
    }
    all_errs
}

fn validate_service_reference(service: &ServiceReference, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if service.namespace.is_empty() {
        all_errs.push(required(&path.child("namespace"), ""));
    }
    if service.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    }
    if let Some(port) = service.port {
        if port < 1 || port > 65535 {
            all_errs.push(invalid(
                &path.child("port"),
                BadValue::Int(port as i64),
                "port must be between 1 and 65535",
            ));
        }
    }
    all_errs
}

fn validate_admission_review_versions(versions: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if versions.is_empty() {
        all_errs.push(required(path, "must specify at least one version"));
        return all_errs;
    }

    let mut seen = BTreeSet::new();
    let mut has_recognized = false;
    for (i, version) in versions.iter().enumerate() {
        if !seen.insert(version.clone()) {
            all_errs.push(duplicate(&path.index(i), BadValue::String(version.clone())));
            continue;
        }
        for msg in is_dns1035_label(version) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(version.clone()),
                &msg,
            ));
        }
        if version == "v1" || version == "v1beta1" {
            has_recognized = true;
        }
    }

    if !has_recognized {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", versions)),
            "must include at least one of v1, v1beta1",
        ));
    }

    all_errs
}

fn validate_rule_with_operations(
    rule: &RuleWithOperations,
    path: &Path,
    allow_subresource: bool,
) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.operations.is_empty() {
        all_errs.push(required(&path.child("operations"), ""));
    } else if rule.operations.len() > 1 && rule.operations.contains(&OperationType::All) {
        all_errs.push(invalid(
            &path.child("operations"),
            BadValue::String(format!("{:?}", rule.operations)),
            "if '*' is present, must not specify other operations",
        ));
    }

    for (i, op) in rule.operations.iter().enumerate() {
        let valid = matches!(
            op,
            OperationType::All
                | OperationType::Create
                | OperationType::Update
                | OperationType::Delete
                | OperationType::Connect
        );
        if !valid {
            all_errs.push(not_supported(
                &path.child("operations").index(i),
                BadValue::String(format!("{:?}", op)),
                &["*", "CREATE", "UPDATE", "DELETE", "CONNECT"],
            ));
        }
    }

    let rule_only = Rule {
        api_groups: rule.api_groups.clone(),
        api_versions: rule.api_versions.clone(),
        resources: rule.resources.clone(),
        scope: rule.scope.clone(),
    };
    all_errs.extend(validate_rule(&rule_only, path, allow_subresource));

    all_errs
}

fn validate_rule(rule: &Rule, path: &Path, allow_subresource: bool) -> ErrorList {
    let mut all_errs = ErrorList::new();

    if rule.api_groups.is_empty() {
        all_errs.push(required(&path.child("apiGroups"), ""));
    }
    if rule.api_groups.len() > 1 && has_wildcard(&rule.api_groups) {
        all_errs.push(invalid(
            &path.child("apiGroups"),
            BadValue::String(format!("{:?}", rule.api_groups)),
            "if '*' is present, must not specify other API groups",
        ));
    }

    if rule.api_versions.is_empty() {
        all_errs.push(required(&path.child("apiVersions"), ""));
    }
    if rule.api_versions.len() > 1 && has_wildcard(&rule.api_versions) {
        all_errs.push(invalid(
            &path.child("apiVersions"),
            BadValue::String(format!("{:?}", rule.api_versions)),
            "if '*' is present, must not specify other API versions",
        ));
    }
    for (i, version) in rule.api_versions.iter().enumerate() {
        if version.is_empty() {
            all_errs.push(required(&path.child("apiVersions").index(i), ""));
        }
    }

    if allow_subresource {
        all_errs.extend(validate_resources(
            &rule.resources,
            &path.child("resources"),
        ));
    } else {
        all_errs.extend(validate_resources_no_subresources(
            &rule.resources,
            &path.child("resources"),
        ));
    }

    if let Some(scope) = rule.scope.as_ref() {
        if *scope != crate::admissionregistration::v1::ScopeType::Cluster
            && *scope != crate::admissionregistration::v1::ScopeType::Namespaced
            && *scope != crate::admissionregistration::v1::ScopeType::AllScopes
        {
            all_errs.push(not_supported(
                &path.child("scope"),
                BadValue::String(format!("{:?}", scope)),
                &["Cluster", "Namespaced", "*"],
            ));
        }
    }

    all_errs
}

fn validate_resources(resources: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if resources.is_empty() {
        all_errs.push(required(path, ""));
    }

    let mut resources_with_wildcard_sub = BTreeSet::new();
    let mut sub_resources_with_wildcard_res = BTreeSet::new();
    let mut has_double_wildcard = false;
    let mut has_single_wildcard = false;
    let mut has_resource_without_subresource = false;

    for (i, res_sub) in resources.iter().enumerate() {
        if res_sub.is_empty() {
            all_errs.push(required(&path.index(i), ""));
            continue;
        }
        if res_sub == "*/*" {
            has_double_wildcard = true;
        }
        if res_sub == "*" {
            has_single_wildcard = true;
        }
        let parts: Vec<&str> = res_sub.splitn(2, '/').collect();
        if parts.len() == 1 {
            has_resource_without_subresource = res_sub != "*";
            continue;
        }
        let res = parts[0];
        let sub = parts[1];
        if resources_with_wildcard_sub.contains(res) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(res_sub.clone()),
                &format!(
                    "if '{}/{}' is present, must not specify {}",
                    res, "*", res_sub
                ),
            ));
        }
        if sub_resources_with_wildcard_res.contains(sub) {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(res_sub.clone()),
                &format!("if '*/{}' is present, must not specify {}", sub, res_sub),
            ));
        }
        if sub == "*" {
            resources_with_wildcard_sub.insert(res.to_string());
        }
        if res == "*" {
            sub_resources_with_wildcard_res.insert(sub.to_string());
        }
    }

    if resources.len() > 1 && has_double_wildcard {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", resources)),
            "if '*/*' is present, must not specify other resources",
        ));
    }
    if has_single_wildcard && has_resource_without_subresource {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", resources)),
            "if '*' is present, must not specify other resources without subresources",
        ));
    }

    all_errs
}

fn validate_resources_no_subresources(resources: &[String], path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if resources.is_empty() {
        all_errs.push(required(path, ""));
    }
    for (i, resource) in resources.iter().enumerate() {
        if resource.is_empty() {
            all_errs.push(required(&path.index(i), ""));
        }
        if resource.contains('/') {
            all_errs.push(invalid(
                &path.index(i),
                BadValue::String(resource.clone()),
                "must not specify subresources",
            ));
        }
    }
    if resources.len() > 1 && has_wildcard(resources) {
        all_errs.push(invalid(
            path,
            BadValue::String(format!("{:?}", resources)),
            "if '*' is present, must not specify other resources",
        ));
    }
    all_errs
}

fn has_wildcard(values: &[String]) -> bool {
    values.iter().any(|v| v == "*")
}

fn validate_match_condition(condition: &MatchCondition, path: &Path) -> ErrorList {
    let mut all_errs = ErrorList::new();
    if condition.name.is_empty() {
        all_errs.push(required(&path.child("name"), ""));
    } else {
        all_errs.extend(validate_qualified_name(
            &condition.name,
            &path.child("name"),
        ));
    }
    if condition.expression.is_empty() {
        all_errs.push(required(&path.child("expression"), ""));
    }
    all_errs
}

fn name_is_dns_subdomain(name: &str, prefix: bool) -> Vec<String> {
    crate::common::validation::name_is_dns_subdomain(name, prefix)
}

trait WebhookAdapter {
    fn name(&self) -> &str;
    fn rules(&self) -> &[RuleWithOperations];
    fn failure_policy(&self) -> Option<FailurePolicyType>;
    fn match_policy(&self) -> Option<MatchPolicyType>;
    fn side_effects(&self) -> Option<SideEffectClass>;
    fn timeout_seconds(&self) -> Option<i32>;
    fn admission_review_versions(&self) -> &[String];
    fn client_config(&self) -> &WebhookClientConfig;
    fn match_conditions(&self) -> &[MatchCondition];
}

impl WebhookAdapter for ValidatingWebhook {
    fn name(&self) -> &str {
        &self.name
    }
    fn rules(&self) -> &[RuleWithOperations] {
        &self.rules
    }
    fn failure_policy(&self) -> Option<FailurePolicyType> {
        self.failure_policy.clone()
    }
    fn match_policy(&self) -> Option<MatchPolicyType> {
        self.match_policy.clone()
    }
    fn side_effects(&self) -> Option<SideEffectClass> {
        self.side_effects.clone()
    }
    fn timeout_seconds(&self) -> Option<i32> {
        self.timeout_seconds
    }
    fn admission_review_versions(&self) -> &[String] {
        &self.admission_review_versions
    }
    fn client_config(&self) -> &WebhookClientConfig {
        &self.client_config
    }
    fn match_conditions(&self) -> &[MatchCondition] {
        &self.match_conditions
    }
}

impl WebhookAdapter for MutatingWebhook {
    fn name(&self) -> &str {
        &self.name
    }
    fn rules(&self) -> &[RuleWithOperations] {
        &self.rules
    }
    fn failure_policy(&self) -> Option<FailurePolicyType> {
        self.failure_policy.clone()
    }
    fn match_policy(&self) -> Option<MatchPolicyType> {
        self.match_policy.clone()
    }
    fn side_effects(&self) -> Option<SideEffectClass> {
        self.side_effects.clone()
    }
    fn timeout_seconds(&self) -> Option<i32> {
        self.timeout_seconds
    }
    fn admission_review_versions(&self) -> &[String] {
        &self.admission_review_versions
    }
    fn client_config(&self) -> &WebhookClientConfig {
        &self.client_config
    }
    fn match_conditions(&self) -> &[MatchCondition] {
        &self.match_conditions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TypeMeta;

    #[test]
    fn validate_validating_webhook_configuration_requires_side_effects() {
        let obj = crate::admissionregistration::internal::ValidatingWebhookConfiguration {
            type_meta: TypeMeta::default(),
            metadata: crate::common::ObjectMeta {
                name: Some("cfg".to_string()),
                ..Default::default()
            },
            webhooks: vec![ValidatingWebhook {
                name: "example.com/hook".to_string(),
                client_config: WebhookClientConfig {
                    url: Some("https://example.com".to_string()),
                    service: None,
                    ca_bundle: Default::default(),
                },
                admission_review_versions: vec!["v1".to_string()],
                ..Default::default()
            }],
        };

        let errs = validate_validating_webhook_configuration(&obj);
        assert!(!errs.is_empty());
    }
}
