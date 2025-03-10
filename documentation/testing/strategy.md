# Testing Strategy

Priority is given to **backend testing** (unit and integration tests for `src/domain`).

- **Unit & Integration Tests**: Rust + Cargo + LLVM coverage (`cargo-llvm-cov`), Nextest (`cargo-nextest`).
- **Mutation Testing**: `cargo-mutants`

End-to-end testing is not yet defined.
