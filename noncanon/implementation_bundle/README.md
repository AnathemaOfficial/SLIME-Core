## Reference Runner

`slime-runner/` contains the open-source reference runner for SLIME.

By default it compiles with the `stub_ab` feature — a simple capacity-check
resolver that demonstrates the SLIME interface without the proprietary
Anathema-Breaker engine.

To compile with the real law engine (requires private AB-S dependency):
```
cargo build --no-default-features --features real_ab
```

### Note

This is a non-canonical implementation. Nothing in this directory modifies
the SLIME v0 formal specification. The runner demonstrates form only.
