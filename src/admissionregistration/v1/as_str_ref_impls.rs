use super::*;

crate::impl_as_str_ref!(ScopeType, {
    Cluster => "Cluster",
    Namespaced => "Namespaced",
    AllScopes => "AllScopes",
});

crate::impl_as_str_ref!(FailurePolicyType, {
    Ignore => "Ignore",
    Fail => "Fail",
});

crate::impl_as_str_ref!(ParameterNotFoundActionType, {
    Allow => "Allow",
    Deny => "Deny",
});

crate::impl_as_str_ref!(MatchPolicyType, {
    Exact => "Exact",
    Equivalent => "Equivalent",
});

crate::impl_as_str_ref!(SideEffectClass, {
    Unknown => "Unknown",
    None => "None",
    Some => "Some",
    NoneOnDryRun => "NoneOnDryRun",
});

crate::impl_as_str_ref!(ValidationAction, {
    Deny => "Deny",
    Warn => "Warn",
    Audit => "Audit",
});

crate::impl_as_str_ref!(OperationType, {
    All => "All",
    Create => "Create",
    Update => "Update",
    Delete => "Delete",
    Connect => "Connect",
});

crate::impl_as_str_ref!(ReinvocationPolicyType, {
    Never => "Never",
    IfNeeded => "IfNeeded",
});
