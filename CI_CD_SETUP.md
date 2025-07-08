# TinyOS CI/CD Pipeline Documentation

## Overview

TinyOS uses **GitHub Actions** for automated build, test, and release management with semantic versioning and comprehensive validation.

## Branch Strategy

```
master (production)     0.2.0, 0.3.0, 1.0.0          [Stable releases]
  ↑
dev (development)       0.2.0-dev.1, 0.2.0-dev.2     [Auto-versioned pre-releases]
  ↑
feature/* (features)    No version changes            [Validated builds only]
```

## Workflow Files

### 1. `.github/workflows/ci.yml` - Main CI/CD Pipeline

**Triggers**: Push to `master`, `dev`; PRs to `master`, `dev`; Weekly schedule (Monday 10 AM UTC)

**Jobs**:
- **build-and-test**: Runs on all branches
  - Rust formatting check (`cargo fmt`)
  - Clippy linting (`cargo clippy`)
  - Build debug and release
  - Run comprehensive test suite
  - Generate `kernel8.img`
  - Upload build artifacts

- **dev-release**: Unified dev release job for both push and scheduled triggers
  - Auto-increment build number (`git rev-list --count HEAD`)
  - Update version to `0.2.0-dev.{build}`
  - Create pre-release with artifacts
  - Tag: `dev-v0.2.0-dev.{build}` (push) or `dev-weekly-v0.2.0-dev.{build}` (scheduled)
  - Runs additional tests for scheduled releases

- **production-release**: Only on `master` branch pushes
  - Use version from `Cargo.toml`
  - Create stable release with artifacts
  - Tag: `production-v{version}`

- **security-scan**: Dependency vulnerability scanning
  - `cargo audit` for security issues
  - `cargo outdated` for dependency updates

### 2. `.github/workflows/pr.yml` - Pull Request Validation

**Triggers**: PRs to `master`, `dev`

**Features**:
- Comprehensive validation (build, test, format, clippy)
- Performance analysis (build time, binary size)
- Documentation verification
- Test coverage analysis
- Breaking change detection (for PRs to `master`)
- Automated PR comments with results

### 3. `.github/workflows/feature.yml` - Feature Branch Validation

**Triggers**: Push to `feature/*`, `bugfix/*`, `hotfix/*`

**Features**:
- Quick build validation
- Feature-specific testing based on branch name
- Generates feature-specific kernel (`kernel8-{feature}.img`)
- Uploads artifacts with 14-day retention

### 4. `.github/workflows/deps.yml` - Dependency Management

**Triggers**: Weekly schedule (Mondays 9 AM UTC), manual dispatch

**Features**:
- Automated dependency updates
- Security vulnerability scanning
- Test updated dependencies
- Create PRs for dependency updates
- Rust toolchain update monitoring

## Version Management

### Semantic Versioning Format

- **Stable**: `X.Y.Z` (e.g., `0.2.0`, `1.0.0`)
- **Development**: `X.Y.Z-dev.{build}` (e.g., `0.2.0-dev.15`)
- **Feature**: No version changes (use artifacts only)

### Version Management Script

Use `./scripts/version.sh` for manual version management:

```bash
# Check current version and git info
./scripts/version.sh check

# Set specific version
./scripts/version.sh set 0.3.0

# Bump version
./scripts/version.sh bump major    # 0.2.0 → 1.0.0
./scripts/version.sh bump minor    # 0.2.0 → 0.3.0
./scripts/version.sh bump patch    # 0.2.0 → 0.2.1

# Set development version
./scripts/version.sh dev           # 0.2.0 → 0.2.0-dev.1

# Set stable version (remove pre-release)
./scripts/version.sh stable        # 0.2.0-dev.1 → 0.2.0
```

## Development Workflow

### 1. Feature Development

```bash
# Create feature branch
git checkout -b feature/new-driver

# Develop feature
# ... make changes ...

# Push triggers feature validation
git push origin feature/new-driver

# Create PR to dev
gh pr create --base dev --title "Add new driver"
```

### 2. Integration Testing

```bash
# Merge to dev triggers auto-versioned dev release
git checkout dev
git merge feature/new-driver
git push origin dev

# This creates: dev-v0.2.0-dev.{build} (or dev-weekly-v0.2.0-dev.{build} for scheduled)
```

### 3. Stable Release

```bash
# When ready for stable release
git checkout master
git merge dev
git push origin master

# This creates: production-v0.2.0 (using version from Cargo.toml)
```

## Release Artifacts

### Development Releases (`dev` branch)

- **Tag**: `dev-v0.2.0-dev.{build}` (push) or `dev-weekly-v0.2.0-dev.{build}` (scheduled)
- **Files**: `kernel8.img`, `tiny_os` (ELF)
- **Pre-release**: Yes
- **Triggers**: 
  - Push to `dev` branch (immediate)
  - Weekly schedule (Monday 10 AM UTC)

### Production Releases (`master` branch)

- **Tag**: `production-v{version}` (from Cargo.toml)
- **Files**: `kernel8.img`, `tiny_os` (ELF)
- **Pre-release**: No
- **Triggers**: Push to `master` branch (immediate)

## CI/CD Features

### ✅ Automated Testing

- **Build Validation**: Debug and release builds
- **Test Suites**: Comprehensive, modular, integration tests
- **QEMU Testing**: Boot validation in emulated environment
- **Code Quality**: Formatting, linting, security scanning

### ✅ Smart Validation

- **Branch-specific**: Different validation for different branches
- **Feature Detection**: Runs appropriate tests based on branch name
- **Performance Monitoring**: Build time and binary size tracking
- **Security**: Dependency vulnerability scanning

### ✅ Release Management

- **Auto-versioning**: Development builds get auto-incremented versions
- **Artifact Management**: Kernel binaries and debug symbols
- **Release Notes**: Automated generation with build info
- **Retention**: Different retention periods for different branches

## Status Badges

Add these to your README for CI/CD visibility:

```markdown
[![CI/CD Pipeline](https://github.com/your-username/tiny_os/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/tiny_os/actions/workflows/ci.yml)
[![Pull Request Validation](https://github.com/your-username/tiny_os/actions/workflows/pr.yml/badge.svg)](https://github.com/your-username/tiny_os/actions/workflows/pr.yml)
[![Security Scan](https://github.com/your-username/tiny_os/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/your-username/tiny_os/actions/workflows/ci.yml)
```

## Configuration

### GitHub Actions Versions

All workflows use the latest stable versions of GitHub Actions:

- **actions/checkout@v4** - Repository checkout
- **dtolnay/rust-toolchain@stable** - Rust toolchain setup (replaces deprecated actions-rs/toolchain)
- **actions/cache@v4** - Dependency caching
- **actions/upload-artifact@v4** - Artifact uploads
- **softprops/action-gh-release@v1** - Release creation
- **actions/github-script@v7** - GitHub API interactions

### Required Secrets

The workflows use standard GitHub tokens and don't require additional secrets for basic functionality.

### Optional Enhancements

- **Slack/Discord Integration**: Add notifications for releases
- **Hardware Testing**: Add self-hosted runners with real Pi hardware
- **Performance Regression**: Track performance metrics over time
- **Documentation**: Auto-generate and deploy documentation

## Troubleshooting

### Common Issues

1. **Build Failures**: Check Rust toolchain compatibility
2. **Test Failures**: Verify QEMU installation and test scripts
3. **Version Conflicts**: Use version management script
4. **Permission Issues**: Ensure scripts are executable

### Debugging

- Check workflow logs in GitHub Actions tab
- Use `./scripts/version.sh check` to verify version state
- Run tests locally with `./test_tinyos.sh`
- Validate build with `./build.sh`

## Next Steps

1. **Setup Repository**: Push workflows to your GitHub repository
2. **Configure Branches**: Set up `master` and `dev` branches
3. **Test Workflows**: Create a test PR to verify functionality
4. **Customize**: Adjust workflows for your specific needs
5. **Monitor**: Watch CI/CD pipeline performance and iterate

The CI/CD pipeline is designed to be robust, secure, and developer-friendly while maintaining the high quality standards of the TinyOS project.
