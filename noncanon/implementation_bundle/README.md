## Reference Runner

`slime-runner/` contains the open-source reference runner for SLIME.

By default it compiles with the `stub_ab` feature — a simple capacity-check
resolver that demonstrates the SLIME interface without the proprietary
Anathema-Breaker engine.

The public checkout validates the open-source harness only:
```
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
```

### Note

This is a non-canonical implementation. Nothing in this directory modifies
the SLIME v0 formal specification. The runner demonstrates form only.

The `real_ab` feature name is reserved for private enterprise wiring and is
not backed by this public manifest.

The `private_validation` feature gates MB01-MB05 checks that require private
artifacts or Unix-specific tooling. Those validations are not part of the
default public `cargo test` path.

The `integration_demo` feature exists only to support the bounded public
SAFA -> SLIME cross-integration proof on non-Unix machines. It replaces the
fail-closed Unix socket requirement with a local file sink controlled by
`SLIME_DEMO_EGRESS_FILE`. That feature is non-default, non-canonical, and
must not be read as deployment parity or real actuator proof.
