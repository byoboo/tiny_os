# Troubleshooting "Can't find crate for test" Error

## Quick Fix (Most Common)

1. **Ensure rust-analyzer is installed:**
   ```bash
   rustup component add rust-analyzer --toolchain nightly
   ```

2. **Run the reset script:**
   ```bash
   ./reset_rust_analyzer.sh
   ```

3. **Restart VSCode completely** (close all windows, reopen)

4. **Wait for rust-analyzer to initialize** (check status bar)

## Common Issues

### "Unknown binary 'rust-analyzer'" Error
If you see this error:
```
error: Unknown binary 'rust-analyzer' in official toolchain 'nightly-x86_64-unknown-linux-gnu'.
```

**Solution:**
```bash
# Install rust-analyzer component
rustup component add rust-analyzer --toolchain nightly

# Verify it's installed
rustup component list --installed --toolchain nightly | grep rust-analyzer

# Test it works
rustup run nightly rust-analyzer --version
```

This happens because rust-analyzer is not included by default in minimal toolchain installations.

## If Error Persists

### Check for Interfering Files

```bash
# Look for any test-related configurations
grep -r "test" .vscode/ .cargo/ *.toml 2>/dev/null | grep -v "# " | grep -v "dead_code\|is never used\|generated.*warning"

# Check for hidden rust-analyzer cache
find . -name "*rust-analyzer*" -o -name ".ra_*" 2>/dev/null
```

### Manual Steps

1. **Verify configurations:**
   ```bash
   ./verify_rust_analyzer_fix.sh
   ```

2. **Force restart rust-analyzer:**
   - `Ctrl+Shift+P` → `Rust Analyzer: Restart Server`

3. **Check global rust-analyzer config:**
   ```bash
   # Linux/macOS
   ls ~/.config/rust-analyzer/ 2>/dev/null || echo "No global config"
   
   # If global config exists, temporarily rename it
   mv ~/.config/rust-analyzer ~/.config/rust-analyzer.backup
   ```

4. **Clear VSCode workspace state:**
   - Close VSCode
   - Delete `.vscode/settings.json.bak` if it exists
   - Reopen project

## Still Not Working?

### Nuclear Option - Complete Cleanup

```bash
# 1. Clean everything
cargo clean
rm -rf target/
pkill -f rust-analyzer

# 2. Remove any global rust-analyzer config temporarily
mv ~/.config/rust-analyzer ~/.config/rust-analyzer.backup 2>/dev/null || true

# 3. Restart VSCode completely
# 4. Reopen project
# 5. Let rust-analyzer initialize from scratch
```

### Check for Project Structure Issues

```bash
# Ensure no test modules in source files
find src/ -name "*.rs" -exec grep -l "mod tests\|#\[cfg(test)\]\|#\[test\]" {} \; 2>/dev/null || echo "No test code found (good!)"

# Ensure Cargo.toml has no test profiles
grep -n "\[profile\.test\]" Cargo.toml || echo "No test profile found (good!)"
```

## Success Indicators

✅ No "can't find crate for test" in Problems panel  
✅ Rust-analyzer shows "Ready" in status bar  
✅ Code completion works for embedded types  
✅ No test-related UI elements in VSCode  
✅ `cargo check` builds without test errors

## Prevention

- Never add `cargo test` tasks to `.vscode/tasks.json`
- Don't add `[profile.test]` to `Cargo.toml` 
- Keep `.rust-analyzer.toml` with `cargo.unsetTest = true`
- Use shell-based testing instead (see `TESTING_INFRASTRUCTURE.md`)

---

**If none of these steps work, the issue might be with your rust-analyzer version or VSCode installation. Consider updating both.**
