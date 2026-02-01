# Core-group validation migration plan (internal-first)

## Goals
- Align Taibai validation semantics with upstream Kubernetes: validate resources on internal types after v1 defaulting + conversion.
- Keep v1 validation only for version-specific options/subresources and as thin wrappers over internal validation.

## Migration order (core-group first, then dependents)
1) Core/internal validation foundation
   - Create: `src/core/internal/validation/mod.rs`
   - Add internal validation modules to mirror core v1 validation scope:
     - `pod.rs`, `pod_spec.rs`
     - `volume.rs`, `storage.rs`
     - `service.rs`
     - `node.rs`
     - `namespace.rs`
     - `quota.rs`
     - `events.rs`
     - `endpoints.rs`
     - `replication_controller.rs`
     - `container.rs`, `env.rs`, `probe.rs`, `resources.rs`, `security.rs`, `selector.rs`, `affinity.rs`
   - Wire `src/core/internal/mod.rs` to expose `pub mod validation;` and the modules.

2) Core/v1 validation becomes a thin wrapper
   - For each `src/core/v1/validation/*.rs` that validates resource specs:
     - Convert v1 → internal (use existing conversions; add minimal conversion helpers where missing).
     - Call internal validation and return its errors.
   - Keep v1-only validations in v1 (e.g., v1 options like PodLogOptions, versioned-only fields).

3) Downstream group alignment (depend on core)
   - Update groups that depend on core PodSpec/Volume/etc. to validate internal types:
     - apps, batch, autoscaling, policy, networking, storage, node, scheduling, discovery, coordination, rbac, certificates, authentication, authorization, apiserverinternal, storagemigration, resource.
   - Ensure their v1 validation is wrapper-only unless it is a true versioned option/subresource.

4) Generated validators
   - For `zz_generated.validations.go` wrappers, ensure they call internal validation after conversion, or remain no-ops if they are strictly versioned-only structural checks.

5) Tests
   - Move or duplicate v1 validation tests into internal validation tests.
   - Keep minimal v1 tests to assert conversion + internal validation wiring.

## Progress tracking checklist

### Core/internal validation (foundation)
- [x] `src/core/internal/validation/mod.rs` created and wired
- [x] `pod.rs` (internal)
- [x] `pod_spec.rs` (internal)
- [x] `pod_spec.rs` uses native internal logic (no v1 conversion)
- [ ] internal validation uses native internal logic (no v1 wrapper remaining)
- [x] `volume.rs` (internal, native types)
- [x] `storage.rs` (internal, native types)
- [x] `service.rs` (internal)
- [x] `node.rs` (internal)
- [x] `namespace.rs` (internal)
- [x] `quota.rs` (internal)
- [x] `events.rs` (internal)
- [ ] `endpoints.rs` (internal)
- [ ] `replication_controller.rs` (internal)
- [x] `container.rs` (internal)
- [x] `env.rs` (internal)
- [x] `probe.rs` (internal)
- [x] `resources.rs` (internal)
- [ ] `security.rs` (internal)
- [ ] `selector.rs` (internal)
- [ ] `affinity.rs` (internal)

### Core/v1 wrapper thinning
- [x] `src/core/v1/validation/pod.rs` uses internal validation
- [x] `src/core/v1/validation/pod_spec.rs` uses internal validation
- [x] `src/core/v1/validation/volume.rs` uses internal validation
- [x] `src/core/v1/validation/storage.rs` uses internal validation
- [x] `src/core/v1/validation/service.rs` uses internal validation
- [x] `src/core/v1/validation/node.rs` uses internal validation
- [x] `src/core/v1/validation/namespace.rs` uses internal validation
- [x] `src/core/v1/validation/resource_quota.rs` uses internal validation
- [x] `src/core/v1/validation/events.rs` uses internal validation
- [ ] `src/core/v1/validation/endpoints.rs` uses internal validation
- [ ] `src/core/v1/validation/replication_controller.rs` uses internal validation
- [x] `src/core/v1/validation/container.rs` uses internal validation
- [x] `src/core/v1/validation/env.rs` uses internal validation
- [x] `src/core/v1/validation/probe.rs` uses internal validation
- [x] `src/core/v1/validation/resources.rs` uses internal validation
- [ ] `src/core/v1/validation/selector.rs` uses internal validation
- [ ] `src/core/v1/validation/affinity.rs` uses internal validation
- [ ] v1-only validations audited and retained (PodLogOptions, etc.)

### Downstream groups (internal validation)
- [ ] apps
- [ ] batch
- [ ] autoscaling
- [ ] policy
- [ ] networking
- [ ] storage
- [ ] node
- [ ] scheduling
- [ ] discovery
- [ ] coordination
- [ ] rbac
- [ ] certificates
- [ ] authentication
- [ ] authorization
- [ ] apiserverinternal
- [ ] storagemigration
- [ ] resource

### Tests
- [ ] internal validation tests added/ported for core
- [ ] v1 wrapper tests reduced to conversion + internal validation wiring
- [ ] targeted `cargo test` scopes updated per module

## Notes
- Validate ordering: Decode → Default (v1) → Convert (v1→internal) → Validate (internal)
- If conversion is missing for a type, implement minimal conversion before wiring validation.
