# Team C Infrastructure - Task Tracking Checklist

Last updated: 2026-02-01
Worktree: `worktrees/team-c-infrastructure`

## Phase 0: Kickoff
- [x] Read task brief: `agents/claude/outputs/tasks/team_c_infrastructure_tasks.md`
- [ ] Confirm branch: `team/c/infrastructure`
- [ ] Set working dir: `worktrees/team-c-infrastructure`

## Task 1: coordination validation (refactor)
- [x] Refactor `src/coordination/validation.rs` to internal validation
- [x] Create `src/coordination/internal/validation/`
- [x] Create `src/coordination/v1/validation/` wrapper
- [x] Implement Lease validation (internal type)
- [ ] Add/adjust tests for coordination validation

## Task 2: node validation (refactor)
- [x] Refactor `src/node/validation.rs` to internal validation
- [x] Create `src/node/internal/validation/`
- [x] Create `src/node/v1/validation/` wrapper
- [x] Implement RuntimeClass validation (internal type)
- [x] Add/adjust tests for node validation

## Task 3: events validation (refactor)
- [x] Refactor `src/events/v1/validation.rs` to wrapper
- [x] Create `src/events/internal/validation/`
- [x] Implement Event validation (internal type)
- [x] Add/adjust tests for events validation

## Task 4: scheduling validation (new)
- [x] Create `src/scheduling/internal/validation/`
- [x] Create `src/scheduling/v1/validation/`
- [x] Implement PriorityClass validation
- [x] Add/adjust tests for scheduling validation

## Task 5: discovery validation (new)
- [x] Create `src/discovery/internal/validation/`
- [x] Create `src/discovery/v1/validation/`
- [x] Implement EndpointSlice validation
- [x] Add/adjust tests for discovery validation

## Task 6: policy validation (new)
- [x] Create `src/policy/internal/validation/`
- [x] Create `src/policy/v1/validation/`
- [x] Implement PodDisruptionBudget validation
- [x] Add/adjust tests for policy validation

## Task 7: resource validation (check/refactor)
- [x] Review `src/resource/validation.rs` (current pattern)
- [x] If needed, refactor to internal validation + v1 wrapper
- [x] Verify DeviceClass validation
- [x] Verify ResourceClaim validation
- [x] Verify ResourceSlice validation
- [x] Add/adjust tests for resource validation

## Task 8: storagemigration validation (new)
- [x] Create `src/storagemigration/internal/validation/`
- [x] Create `src/storagemigration/v1alpha1/validation/`
- [x] Implement StorageVersionMigration validation
- [x] Add/adjust tests for storagemigration validation

## Acceptance & Quality Gates
- [ ] All v1 validation modules are wrapper-style
- [ ] Unit tests coverage > 80% for touched modules
- [ ] `cargo fmt` (run from `taibai_api/`)
- [ ] `cargo check` (run from `taibai_api/`)
- [ ] `cargo test --package taibai_api -- coordination::`
- [ ] `cargo test --package taibai_api -- node::`
- [ ] `cargo test --package taibai_api -- events::`
- [ ] `cargo test --package taibai_api -- scheduling::`
- [ ] `cargo test --package taibai_api -- discovery::`
- [ ] `cargo test --package taibai_api -- policy::`
- [ ] `cargo test --package taibai_api -- resource::`
- [ ] `cargo test --package taibai_api -- storagemigration::`
