# Clippy Configuration

This project uses clippy configuration to enforce code quality and prevent common issues.

## Key Configurations

### Format Macro Enforcement
- **`uninlined_format_args = "warn"`**: Enforces modern inline variable syntax in format macros
  - ✅ Good: `format!("{var}")`, `println!("{value}")`
  - ❌ Bad: `format!("{}", var)`, `println!("{}", value)`

### Configuration Files
- **`clippy.toml`**: Project-level clippy configuration
- **`Cargo.toml` [lints.clippy]**: Workspace-level lint configuration

### CI Integration
The CI workflow runs `cargo clippy -- -D warnings` which treats all clippy warnings as errors, ensuring that format macro violations (and other issues) will cause CI to fail.

### Testing the Configuration
To test if the configuration works:

1. Temporarily introduce a violation: `println!("{}", some_var)`
2. Run `cargo clippy` - should show a warning
3. Run `cargo clippy -- -D warnings` - should fail (as in CI)
4. Revert the change

This prevents future commits from accidentally introducing old-style format macro usage.
