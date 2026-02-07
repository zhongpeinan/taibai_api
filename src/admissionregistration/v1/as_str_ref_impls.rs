use super::*;

crate::impl_as_str_ref!(ScopeType, {
    Cluster => scope_type::CLUSTER,
    Namespaced => scope_type::NAMESPACED,
    AllScopes => scope_type::ALL_SCOPES,
});

crate::impl_as_str_ref!(FailurePolicyType, {
    Ignore => failure_policy_type::IGNORE,
    Fail => failure_policy_type::FAIL,
});

crate::impl_as_str_ref!(ParameterNotFoundActionType, {
    Allow => parameter_not_found_action_type::ALLOW,
    Deny => parameter_not_found_action_type::DENY,
});

crate::impl_as_str_ref!(MatchPolicyType, {
    Exact => match_policy_type::EXACT,
    Equivalent => match_policy_type::EQUIVALENT,
});

crate::impl_as_str_ref!(SideEffectClass, {
    Unknown => side_effect_class::UNKNOWN,
    None => side_effect_class::NONE,
    Some => side_effect_class::SOME,
    NoneOnDryRun => side_effect_class::NONE_ON_DRY_RUN,
});

crate::impl_as_str_ref!(ValidationAction, {
    Deny => validation_action::DENY,
    Warn => validation_action::WARN,
    Audit => validation_action::AUDIT,
});

crate::impl_as_str_ref!(OperationType, {
    All => operation_type::ALL,
    Create => operation_type::CREATE,
    Update => operation_type::UPDATE,
    Delete => operation_type::DELETE,
    Connect => operation_type::CONNECT,
});

crate::impl_as_str_ref!(ReinvocationPolicyType, {
    Never => reinvocation_policy_type::NEVER,
    IfNeeded => reinvocation_policy_type::IF_NEEDED,
});
