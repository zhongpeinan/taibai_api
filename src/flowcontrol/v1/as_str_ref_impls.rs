use super::*;

crate::impl_as_str_ref!(FlowDistinguisherMethodType, {
    ByUser => flow_distinguisher_method_type::BY_USER,
    ByNamespace => flow_distinguisher_method_type::BY_NAMESPACE,
});

crate::impl_as_str_ref!(SubjectKind, {
    User => subject_kind::USER,
    Group => subject_kind::GROUP,
    ServiceAccount => subject_kind::SERVICE_ACCOUNT,
});

crate::impl_as_str_ref!(FlowSchemaConditionType, {
    Dangling => flow_schema_condition_type::DANGLING,
});

crate::impl_as_str_ref!(PriorityLevelEnablement, {
    Exempt => priority_level_enablement::EXEMPT,
    Limited => priority_level_enablement::LIMITED,
});

crate::impl_as_str_ref!(LimitResponseType, {
    Queue => limit_response_type::QUEUE,
    Reject => limit_response_type::REJECT,
});

crate::impl_as_str_ref!(PriorityLevelConfigurationConditionType, {
    ConcurrencyShared => priority_level_condition_type::CONCURRENCY_SHARED,
});

crate::impl_as_str_ref!(ConditionStatus, {
    True => condition_status::TRUE,
    False => condition_status::FALSE,
    Unknown => condition_status::UNKNOWN,
});
