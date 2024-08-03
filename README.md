# Helm Plugin

[![ci](https://github.com/fluentci-io/helm-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/helm-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [Helm](https://helm.sh/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm helm setup
```

## Functions

| Name     | Description                                |
| -------- | ------------------------------------------ |
| setup    | Installs a specific version of helm.       |
| install  | Install a chart                            |
| lint     | Examine a chart for possible issues        |
| package  | Package a chart directory into a chart archive |
| pull     | Pull a chart from a repository and (optionally) unpack it in local directory  |
| push     | Push a chart package to a chart repository |
| registry | login to or logout from a registry       |
| rollback | Rollback to a previous release             |
| test     | Run tests for a release                    |
| upgrade  | Upgrade a release                          |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/helm@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: helm
    args: |
      setup
- name: Show helm version
  run: |
    type helm
    helm version
```
