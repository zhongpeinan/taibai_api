# Team B Security & Policy - Task Tracking Checklist

Last updated: 2026-02-01
Worktree: `worktrees/team-b-security-policy`

## Phase 0: Kickoff
- [x] Read task brief: `agents/claude/outputs/tasks/team_b_security_policy_tasks.md`
- [ ] Confirm branch: `team/b/security-policy`
- [ ] Set working dir: `worktrees/team-b-security-policy`

## Task 1: rbac validation (new)
- [x] Create `src/rbac/internal/validation/`
- [x] Create `src/rbac/v1/validation/`
- [x] Implement Role validation
- [x] Implement ClusterRole validation
- [x] Implement RoleBinding validation
- [x] Implement ClusterRoleBinding validation
- [x] Add v1 wrapper modules
- [x] Add/adjust tests for rbac validation

## Task 2: admission validation (refactor)
- [x] Refactor `src/admission/v1/validation.rs` to wrapper
- [x] Create `src/admission/internal/validation/`
- [x] Implement AdmissionReview validation
- [x] Implement AdmissionRequest validation
- [x] Implement AdmissionResponse validation
- [x] Add/adjust tests for admission validation

## Task 3: admissionregistration validation (new)
- [x] Create `src/admissionregistration/internal/validation/`
- [x] Create `src/admissionregistration/v1/validation/`
- [x] Implement ValidatingWebhookConfiguration validation
- [x] Implement MutatingWebhookConfiguration validation
- [x] Add/adjust tests for admissionregistration validation

## Task 4: authorization validation (new)
- [x] Create `src/authorization/internal/validation/`
- [x] Create `src/authorization/v1/validation/`
- [x] Implement SubjectAccessReview validation
- [x] Implement SelfSubjectAccessReview validation
- [x] Implement LocalSubjectAccessReview validation
- [x] Add/adjust tests for authorization validation

## Task 5: certificates validation (new)
- [x] Create `src/certificates/internal/validation/`
- [x] Create `src/certificates/v1/validation/`
- [x] Implement CertificateSigningRequest validation
- [x] Add/adjust tests for certificates validation

## Task 6: authentication validation (check/refactor)
- [x] Review `src/authentication/validation.rs` (current pattern)
- [x] If needed, refactor to internal validation + v1 wrapper
- [x] Implement TokenRequest validation
- [x] Implement TokenReview validation
- [x] Add/adjust tests for authentication validation

## Task 7: imagepolicy validation (new)
- [x] Create `src/imagepolicy/internal/validation/`
- [x] Create `src/imagepolicy/v1/validation/`
- [x] Implement ImageReview validation
- [x] Add/adjust tests for imagepolicy validation

## Acceptance & Quality Gates
- [ ] All v1 validation modules are wrapper-style
- [ ] Unit tests coverage > 80% for touched modules
- [ ] `cargo fmt`
- [ ] `cargo check`
- [ ] `cargo test --package taibai_api -- rbac::`
- [ ] `cargo test --package taibai_api -- admission::`
- [ ] `cargo test --package taibai_api -- admissionregistration::`
- [ ] `cargo test --package taibai_api -- authorization::`
- [ ] `cargo test --package taibai_api -- certificates::`
- [ ] `cargo test --package taibai_api -- authentication::`
- [ ] `cargo test --package taibai_api -- imagepolicy::`
