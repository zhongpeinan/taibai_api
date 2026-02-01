# Team D Storage & Networking - Task Tracking Checklist

Last updated: 2026-02-01
Worktree: `worktrees/team-d-storage-networking`

## Phase 0: Kickoff
- [x] Read task brief: `agents/claude/outputs/tasks/team_d_storage_networking_tasks.md`
- [ ] Confirm branch: `team/d/storage-networking`
- [ ] Set working dir: `worktrees/team-d-storage-networking`

## Task 1: storage validation (new)
- [x] Create `src/storage/internal/validation/`
- [x] Create `src/storage/v1/validation/`
- [x] Implement StorageClass validation
- [x] Implement CSIDriver validation
- [x] Implement CSINode validation
- [x] Implement VolumeAttachment validation
- [x] Implement CSIStorageCapacity validation
- [x] Add/adjust tests for storage validation

## Task 2: networking validation (refactor)
- [x] Refactor `src/networking/v1/validation.rs` to wrapper
- [x] Create `src/networking/internal/validation/`
- [x] Implement NetworkPolicy validation
- [x] Implement Ingress validation
- [x] Implement IngressClass validation
- [x] Evaluate ServiceCIDR validation
- [x] Evaluate ClusterCIDR validation (no ClusterCIDR types found)
- [x] Add/adjust tests for networking validation

## Task 3: flowcontrol validation (new)
- [x] Create `src/flowcontrol/internal/validation/`
- [x] Create `src/flowcontrol/v1/validation/`
- [x] Implement FlowSchema validation
- [x] Implement PriorityLevelConfiguration validation
- [x] Add/adjust tests for flowcontrol validation

## Task 4: extensions validation (evaluate)
- [x] Review `src/extensions` status (deprecated resources)
- [x] Decide whether to add validation modules (no upstream validation found)
- [ ] If needed, create `src/extensions/internal/validation/`
- [ ] If needed, create `src/extensions/v1/validation/`
- [ ] Add/adjust tests for extensions validation (if implemented)

## Task 5: apiserverinternal validation (special handling)
- [x] Review `src/apiserverinternal/validation.rs` layout
- [x] Decide whether to move into standard internal/v1 validation paths
- [x] If needed, create `src/apiserverinternal/internal/validation/`
- [x] If needed, create `src/apiserverinternal/v1alpha1/validation/`
- [x] Verify StorageVersion validation behavior
- [x] Add/adjust tests for apiserverinternal validation

## Acceptance & Quality Gates
- [ ] All v1 validation modules are wrapper-style
- [ ] Unit tests coverage > 80% for touched modules
- [x] `cargo fmt` (run from `taibai_api/`)
- [x] `cargo check` (run from `taibai_api/`)
- [x] `cargo test --package taibai_api -- storage::`
- [x] `cargo test --package taibai_api -- networking::`
- [x] `cargo test --package taibai_api -- flowcontrol::`
- [ ] `cargo test --package taibai_api -- extensions::`
- [x] `cargo test --package taibai_api -- apiserverinternal::`
