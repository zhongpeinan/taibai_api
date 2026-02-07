use super::*;

crate::impl_as_str_ref!(FlowDistinguisherMethodType, {
    ByUser => "ByUser",
    ByNamespace => "ByNamespace",
});

crate::impl_as_str_ref!(SubjectKind, {
    User => "User",
    Group => "Group",
    ServiceAccount => "ServiceAccount",
});

crate::impl_as_str_ref!(FlowSchemaConditionType, {
    Dangling => "Dangling",
});

crate::impl_as_str_ref!(PriorityLevelEnablement, {
    Exempt => "Exempt",
    Limited => "Limited",
});

crate::impl_as_str_ref!(LimitResponseType, {
    Queue => "Queue",
    Reject => "Reject",
});

crate::impl_as_str_ref!(PriorityLevelConfigurationConditionType, {
    ConcurrencyShared => "ConcurrencyShared",
});

crate::impl_as_str_ref!(ConditionStatus, {
    True => "True",
    False => "False",
    Unknown => "Unknown",
});
