# Team A Core Workloads Task Checklist

Updated: 2026-02-01
Worktree: worktrees/team-a-core-workloads
Branch: team/a/core-workloads

## Apps (Highest Priority)
- [x] Create `src/apps/internal/validation/` directory
- [x] Add `src/apps/internal/validation/mod.rs`
- [x] Implement `deployment.rs` validation (internal type)
- [x] Implement `daemonset.rs` validation (internal type)
- [x] Implement `statefulset.rs` validation (internal type)
- [x] Implement `replicaset.rs` validation (internal type)
- [x] Implement `controllerrevision.rs` validation (internal type)
- [x] Refactor `src/apps/v1/validation.rs` to wrapper (v1 -> internal -> validate)
- [x] Add/adjust unit tests for apps validation

## Batch
- [x] Create `src/batch/internal/validation/` directory
- [x] Add `src/batch/internal/validation/mod.rs`
- [x] Implement `job.rs` validation (internal type)
- [x] Implement `cronjob.rs` validation (internal type)
- [x] Refactor `src/batch/v1/validation.rs` to wrapper (v1 -> internal -> validate)
- [x] Add/adjust unit tests for batch validation

## Autoscaling
- [x] Create `src/autoscaling/internal/validation/` directory
- [x] Add `src/autoscaling/internal/validation/mod.rs`
- [x] Implement `hpa.rs` validation (internal type)
- [x] Implement `v1/validation.rs` wrapper
- [x] Add/adjust unit tests for autoscaling validation
  - [x] HPA validation aligned to upstream in internal layer

## Cross-cutting / Verification
- [x] Ensure core types are referenced correctly
- [ ] Unit test coverage > 80% for modified modules
- [x] Run `cargo fmt` from `taibai_api/`
- [x] Run `cargo check` from `taibai_api/`
- [x] Run targeted tests:
  - [x] `cargo test --package taibai_api -- apps::`
  - [x] `cargo test --package taibai_api -- batch::`
  - [x] `cargo test --package taibai_api -- autoscaling::`
