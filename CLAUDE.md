# emyu

Android emulator lifecycle management for AI agents via MCP. Create, manage, interact with emulators. Consumes AdbTransport trait.

## Build

```bash
nix build
nix run .#emyu
cargo build
```

## Architecture

- Binary: `emyu`
- Language: Rust (edition 2024, rust-version 1.91.0)
- License: MIT
- Nix: substrate `rust-tool-release-flake.nix` pattern
