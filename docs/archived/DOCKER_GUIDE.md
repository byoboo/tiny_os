# Docker Development Environment Guide

## Quick Start

1. **Build the environment:**

   ```bash
   make setup
   ```

2. **Enter development shell:**

   ```bash
   make dev-shell
   ```

3. **Build and test:**

   ```bash
   make build
   make test
   ```

## Available Commands

### Development Workflow

- `make setup` - Build Docker development environment
- `make dev-shell` - Interactive development shell
- `make build` - Build TinyOS kernel
- `make test` - Run complete test suite
- `make format` - Format code
- `make lint` - Run linter

### CI/CD Integration

- `make ci-shell` - Enter CI environment
- `make validate-ci` - Validate CI environment
- `make ci-local` - Run local CI simulation

### Utilities

- `make clean` - Clean build artifacts
- `make status` - Show Docker environment status
- `make help` - Show all commands

## Docker Services

### Development (`dev`)

- **Purpose:** Interactive development with persistent volumes
- **Features:**
  - Persistent cargo cache
  - Source code mounted as volume
  - Non-root user for file permissions
  - QEMU for testing
- **Usage:** `docker-compose run --rm dev bash`

### CI (`ci`)

- **Purpose:** Matches GitHub Actions environment
- **Features:**
  - Minimal production-like environment
  - Automated testing
  - No persistent volumes
- **Usage:** `docker-compose run --rm ci ./test_tinyos.sh`

### Build (`build`)

- **Purpose:** Quick build verification
- **Features:**
  - Fast compilation
  - Cached dependencies
- **Usage:** `docker-compose run --rm build`

### Test (`test`)

- **Purpose:** Complete test suite execution
- **Features:**
  - QEMU integration
  - All test categories
  - Privileged mode for hardware emulation
- **Usage:** `docker-compose run --rm test`

## Volume Management

### Persistent Volumes

- `cargo-cache` - Cargo registry cache
- `cargo-git-cache` - Cargo git dependencies
- `target-cache` - Rust build artifacts

### Benefits

- Faster builds after initial setup
- Reduced network usage
- Consistent dependency versions

### Cleanup

```bash
# Remove all volumes and rebuild
make clean
make setup
```

## Environment Variables

### Build Configuration

- `RUST_BACKTRACE=1` - Enable Rust backtraces
- `CARGO_TARGET_DIR=/workspace/target` - Standardized target directory

### Custom Configuration

Create `.env` file in project root:

```bash
# Custom environment variables
RUST_LOG=debug
CUSTOM_BUILD_FLAGS=--verbose
```

## File Permissions

### Development Container

- Uses non-root user `dev` (UID 1000)
- Matches most Linux desktop user IDs
- Prevents permission issues with mounted volumes

### CI Container

- Uses root user for CI compatibility
- Matches GitHub Actions environment
- Isolated from development files

## Troubleshooting

### Permission Issues

```bash
# Fix file permissions
sudo chown -R $USER:$USER .
```

### Build Cache Issues

```bash
# Clear all caches
make clean
docker system prune -a
make setup
```

### QEMU Issues

```bash
# Check QEMU availability
make dev-shell
qemu-system-aarch64 --version
```

### Container Issues

```bash
# Check container status
make status

# Restart containers
docker-compose down
docker-compose up -d
```

## Integration with VS Code

### Dev Containers Extension

1. Install "Dev Containers" extension
2. Open command palette (Ctrl+Shift+P)
3. Select "Dev Containers: Reopen in Container"
4. Choose TinyOS development environment

### Manual Integration

1. Start development container: `make dev-shell`
2. In another terminal: `code .`
3. Use VS Code normally while container runs

## Performance Optimization

### Build Speed

- Use `make build` instead of `cargo build` for caching
- Keep containers running between builds
- Use volume mounts for persistent caches

### Testing Speed

- Use `make test` for optimized test execution
- Run specific test categories when needed
- Use CI environment for final validation

## Legacy Compatibility

### Host-Based Development

- `make build-host` - Build on host system
- `make test-host` - Test on host system
- Requires manual Rust and QEMU installation

### Migration Path

1. Start with Docker development: `make dev-shell`
2. Gradually move workflows to containerized versions
3. Use host fallbacks only when needed

## Best Practices

### Development Workflow

1. Use `make dev-shell` for interactive development
2. Use `make build` and `make test` for validation
3. Use `make ci-local` before pushing changes
4. Use `make format` and `make lint` for code quality

### CI/CD Integration

1. Use `make validate-ci` to ensure CI compatibility
2. Match CI environment with `make ci-shell`
3. Test locally before pushing with `make ci-local`

### Maintenance

1. Regular cleanup: `make clean`
2. Keep Docker images updated
3. Monitor volume usage
4. Update dependencies regularly
