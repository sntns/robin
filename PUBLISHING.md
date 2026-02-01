# Publishing to crates.io

This document explains how to publish new releases of `batman-robin` to crates.io using `cargo-release`.

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

This project uses `cargo-release` to automate the release process. The tool handles:
- Version bumping in `Cargo.toml`
- Creating a git commit with the version change
- Creating and pushing a git tag
- Publishing to crates.io

### Using GitHub Actions (Recommended)

1. Go to the [Actions tab](https://github.com/sntns/robin/actions/workflows/release.yml) in your GitHub repository
2. Click "Run workflow"
3. Select the release level:
   - **patch**: Bug fixes (e.g., 0.1.0 → 0.1.1)
   - **minor**: New features (e.g., 0.1.1 → 0.2.0)
   - **major**: Breaking changes (e.g., 0.2.0 → 1.0.0)
4. Click "Run workflow"

The workflow will automatically:
- Bump the version in `Cargo.toml`
- Commit the change
- Create and push a git tag (e.g., `v0.2.0`)
- Publish to crates.io

### Manual Release (Local)

If you prefer to release manually from your local machine:

1. Install `cargo-release`:
   ```bash
   cargo install cargo-release
   ```

2. Run the release command:
   ```bash
   # For a patch release (0.1.0 → 0.1.1)
   cargo release patch --execute
   
   # For a minor release (0.1.1 → 0.2.0)
   cargo release minor --execute
   
   # For a major release (0.2.0 → 1.0.0)
   cargo release major --execute
   ```

   cargo-release will automatically push the changes and tags to GitHub.

**Note**: Make sure you have:
- Your crates.io API token configured in `~/.cargo/credentials`
- Permission to push to the main branch
- A clean working directory with no uncommitted changes

## Configuration

The release configuration is defined in `Cargo.toml` under `[package.metadata.release]`:

```toml
[package.metadata.release]
sign-commit = false
push = true
publish = true
allow-branch = ["main"]
pre-release-commit-message = "Release {{version}}"
tag-message = "Release {{version}}"
tag-name = "v{{version}}"
```

This configuration:
- Does not sign commits (set to `true` if you use GPG signing)
- Automatically pushes changes and tags to GitHub
- Publishes to crates.io
- Only allows releases from the `main` branch
- Uses "Release X.Y.Z" as the commit and tag message
- Creates tags in the format `vX.Y.Z`

## Workflow Details

The `.github/workflows/release.yml` workflow uses `cargo-release` to:

1. Bump the version in `Cargo.toml`
2. Create a commit with the message "Release X.Y.Z"
3. Create a git tag `vX.Y.Z`
4. Publish the crate to crates.io
5. Push the commit and tag to GitHub

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

### Dry Run

To test the release process without actually publishing:

```bash
cargo release patch --dry-run
```

This will show you what changes would be made without executing them.

### Version Already Published

If you accidentally try to publish a version that already exists on crates.io, the workflow will fail. You'll need to:

1. Revert the version change
2. Run the release workflow again with the correct version level

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: New functionality (backward compatible)
- **PATCH** version: Bug fixes (backward compatible)

Examples:
- `v0.1.0` → `v0.1.1`: Bug fix
- `v0.1.1` → `v0.2.0`: New feature
- `v0.9.0` → `v1.0.0`: Breaking change or stable release
