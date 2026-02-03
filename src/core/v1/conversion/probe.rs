//! Probe-related conversions for core v1 ↔ internal API.

use crate::common::{FromInternal, ToInternal};
use crate::core::internal;
use crate::core::v1::probe as v1_probe;

impl ToInternal<internal::ExecAction> for v1_probe::ExecAction {
    fn to_internal(self) -> internal::ExecAction {
        internal::ExecAction {
            command: self.command,
        }
    }
}

impl FromInternal<internal::ExecAction> for v1_probe::ExecAction {
    fn from_internal(value: internal::ExecAction) -> Self {
        Self {
            command: value.command,
        }
    }
}

impl ToInternal<internal::HTTPHeader> for v1_probe::HTTPHeader {
    fn to_internal(self) -> internal::HTTPHeader {
        internal::HTTPHeader {
            name: self.name,
            value: self.value,
        }
    }
}

impl FromInternal<internal::HTTPHeader> for v1_probe::HTTPHeader {
    fn from_internal(value: internal::HTTPHeader) -> Self {
        Self {
            name: value.name,
            value: value.value,
        }
    }
}

impl ToInternal<internal::HTTPGetAction> for v1_probe::HTTPGetAction {
    fn to_internal(self) -> internal::HTTPGetAction {
        internal::HTTPGetAction {
            path: self.path,
            port: self.port,
            host: self.host,
            scheme: self.scheme,
            http_headers: self
                .http_headers
                .into_iter()
                .map(v1_probe::HTTPHeader::to_internal)
                .collect(),
        }
    }
}

impl FromInternal<internal::HTTPGetAction> for v1_probe::HTTPGetAction {
    fn from_internal(value: internal::HTTPGetAction) -> Self {
        Self {
            path: value.path,
            port: value.port,
            host: value.host,
            scheme: value.scheme,
            http_headers: value
                .http_headers
                .into_iter()
                .map(v1_probe::HTTPHeader::from_internal)
                .collect(),
        }
    }
}

impl ToInternal<internal::TCPSocketAction> for v1_probe::TCPSocketAction {
    fn to_internal(self) -> internal::TCPSocketAction {
        internal::TCPSocketAction {
            port: self.port,
            host: self.host,
        }
    }
}

impl FromInternal<internal::TCPSocketAction> for v1_probe::TCPSocketAction {
    fn from_internal(value: internal::TCPSocketAction) -> Self {
        Self {
            port: value.port,
            host: value.host,
        }
    }
}

impl ToInternal<internal::GRPCAction> for v1_probe::GRPCAction {
    fn to_internal(self) -> internal::GRPCAction {
        internal::GRPCAction {
            port: self.port,
            service: self.service.unwrap_or_default(),
        }
    }
}

impl FromInternal<internal::GRPCAction> for v1_probe::GRPCAction {
    fn from_internal(value: internal::GRPCAction) -> Self {
        Self {
            port: value.port,
            service: if value.service.is_empty() {
                None
            } else {
                Some(value.service)
            },
        }
    }
}

impl ToInternal<internal::ProbeHandler> for v1_probe::ProbeHandler {
    fn to_internal(self) -> internal::ProbeHandler {
        internal::ProbeHandler {
            exec: self.exec.map(ToInternal::to_internal),
            http_get: self.http_get.map(ToInternal::to_internal),
            tcp_socket: self.tcp_socket.map(ToInternal::to_internal),
            grpc: self.grpc.map(ToInternal::to_internal),
        }
    }
}

impl FromInternal<internal::ProbeHandler> for v1_probe::ProbeHandler {
    fn from_internal(value: internal::ProbeHandler) -> Self {
        Self {
            exec: value.exec.map(v1_probe::ExecAction::from_internal),
            http_get: value.http_get.map(v1_probe::HTTPGetAction::from_internal),
            tcp_socket: value
                .tcp_socket
                .map(v1_probe::TCPSocketAction::from_internal),
            grpc: value.grpc.map(v1_probe::GRPCAction::from_internal),
        }
    }
}

impl ToInternal<internal::Probe> for v1_probe::Probe {
    fn to_internal(self) -> internal::Probe {
        internal::Probe {
            probe_handler: self.probe_handler.to_internal(),
            initial_delay_seconds: self.initial_delay_seconds,
            timeout_seconds: self.timeout_seconds,
            period_seconds: self.period_seconds,
            success_threshold: self.success_threshold,
            failure_threshold: self.failure_threshold,
            termination_grace_period_seconds: self.termination_grace_period_seconds,
        }
    }
}

impl FromInternal<internal::Probe> for v1_probe::Probe {
    fn from_internal(value: internal::Probe) -> Self {
        Self {
            probe_handler: v1_probe::ProbeHandler::from_internal(value.probe_handler),
            initial_delay_seconds: value.initial_delay_seconds,
            timeout_seconds: value.timeout_seconds,
            period_seconds: value.period_seconds,
            success_threshold: value.success_threshold,
            failure_threshold: value.failure_threshold,
            termination_grace_period_seconds: value.termination_grace_period_seconds,
        }
    }
}

impl ToInternal<internal::LifecycleHandler> for v1_probe::LifecycleHandler {
    fn to_internal(self) -> internal::LifecycleHandler {
        internal::LifecycleHandler {
            exec: self.exec.map(ToInternal::to_internal),
            http_get: self.http_get.map(ToInternal::to_internal),
            tcp_socket: self.tcp_socket.map(ToInternal::to_internal),
            sleep: self.sleep.map(|sleep| sleep.seconds),
        }
    }
}

impl FromInternal<internal::LifecycleHandler> for v1_probe::LifecycleHandler {
    fn from_internal(value: internal::LifecycleHandler) -> Self {
        Self {
            exec: value.exec.map(v1_probe::ExecAction::from_internal),
            http_get: value.http_get.map(v1_probe::HTTPGetAction::from_internal),
            tcp_socket: value
                .tcp_socket
                .map(v1_probe::TCPSocketAction::from_internal),
            sleep: value.sleep.map(|seconds| v1_probe::SleepAction { seconds }),
        }
    }
}

impl ToInternal<internal::Lifecycle> for v1_probe::Lifecycle {
    fn to_internal(self) -> internal::Lifecycle {
        internal::Lifecycle {
            post_start: self.post_start.map(ToInternal::to_internal),
            pre_stop: self.pre_stop.map(ToInternal::to_internal),
            stop_signal: None,
        }
    }
}

impl FromInternal<internal::Lifecycle> for v1_probe::Lifecycle {
    fn from_internal(value: internal::Lifecycle) -> Self {
        Self {
            post_start: value
                .post_start
                .map(v1_probe::LifecycleHandler::from_internal),
            pre_stop: value
                .pre_stop
                .map(v1_probe::LifecycleHandler::from_internal),
        }
    }
}
