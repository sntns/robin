# Publishing to crates.io

This document explains how to publish new releases of `batman-robin` to crates.io.

## Prerequisites

Before publishing, you need to set up a `CARGO_REGISTRY_TOKEN` secret in your GitHub repository:

1. Log in to [crates.io](https://crates.io) and generate an API token:
   - Go to https://crates.io/settings/tokens
   - Create a new token with appropriate permissions
   - Copy the token (you won't be able to see it again)

2. Add the token to your GitHub repository:
   - Go to your repository on GitHub
   - Navigate to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: Paste your crates.io API token
   - Click "Add secret"

## Publishing a Release

To publish a new release:

### 1. Update Version Number

Update the version in the root `Cargo.toml`:

```toml
# Cargo.toml
[package]
version = "0.2.0"  # Update this
```

### 2. Commit Changes

```bash
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git push origin main
```

### 3. Create and Push Tag

```bash
# Create a version tag (must match pattern v*.*.*) 
git tag v0.2.0

# Push the tag to trigger the workflow
git push origin v0.2.0
```

### 4. Monitor the Workflow

The GitHub Actions workflow will automatically:
1. Create a GitHub release at https://github.com/sntns/robin/releases
2. Publish the `batman-robin` crate to crates.io (includes both library and CLI binary)

You can monitor the workflow progress at:
https://github.com/sntns/robin/actions/workflows/release.yml

## Workflow Details

The `.github/workflows/release.yml` workflow consists of two jobs:

1. **create-release**: Creates a GitHub release with installation instructions
2. **publish**: Publishes the unified crate to crates.io

The crate `batman-robin` includes:
- The library (`batman_robin`)
- The CLI binary (`robctl`)

## Installation After Publishing

Users can install the crate in different ways:

### As a Library Dependency
```toml
[dependencies]
batman-robin = "0.2.0"
```

### As a CLI Tool
```bash
cargo install batman-robin
```

## Troubleshooting

### Workflow Fails to Publish

If the workflow fails during publishing:

1. Check the GitHub Actions logs for error messages
2. Verify the `CARGO_REGISTRY_TOKEN` secret is set correctly
3. Ensure the version number is unique (not already published)
4. Make sure all Cargo.toml metadata is valid

### Version Mismatch

Ensure the version in `Cargo.toml` matches the git tag (without the 'v' prefix):

```bash
# Check current version
grep '^version' Cargo.toml
```

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: New functionality (backward compatible)
- **PATCH** version: Bug fixes (backward compatible)

Examples:
- `v0.1.0` → `v0.1.1`: Bug fix
- `v0.1.1` → `v0.2.0`: New feature
- `v0.9.0` → `v1.0.0`: Breaking change or stable release
