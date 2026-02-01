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

### 1. Update Version Numbers

Update the version in both `Cargo.toml` files:

```toml
# lib/Cargo.toml
[package]
version = "0.2.0"  # Update this

# cli/Cargo.toml
[package]
version = "0.2.0"  # Update this
```

### 2. Commit Changes

```bash
git add lib/Cargo.toml cli/Cargo.toml
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
2. Publish `batman-robin-lib` to crates.io
3. Wait for the library to be indexed on crates.io
4. Publish `batman-robin-cli` to crates.io

You can monitor the workflow progress at:
https://github.com/sntns/robin/actions/workflows/release.yml

## Workflow Details

The `.github/workflows/release.yml` workflow consists of three jobs:

1. **create-release**: Creates a GitHub release with installation instructions
2. **publish-lib**: Publishes the library crate (`batman-robin-lib`)
3. **publish-cli**: Publishes the CLI crate (`batman-robin-cli`) after the library is available

The workflow includes a wait mechanism to ensure the library is indexed on crates.io before publishing the CLI, since the CLI depends on the library.

## Troubleshooting

### Workflow Fails to Publish

If the workflow fails during publishing:

1. Check the GitHub Actions logs for error messages
2. Verify the `CARGO_REGISTRY_TOKEN` secret is set correctly
3. Ensure the version numbers are unique (not already published)
4. Make sure all Cargo.toml metadata is valid

### CLI Publish Times Out

If the CLI publish step times out waiting for the library:

1. Check if `batman-robin-lib` was successfully published to crates.io
2. The workflow waits up to 5 minutes (30 attempts × 10 seconds)
3. If needed, you can manually publish the CLI:
   ```bash
   cargo publish --manifest-path cli/Cargo.toml
   ```

### Version Mismatch

Ensure both crates have the same version number before tagging:

```bash
# Check current versions
grep '^version' lib/Cargo.toml cli/Cargo.toml
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
