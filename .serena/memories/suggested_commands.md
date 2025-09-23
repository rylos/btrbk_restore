# Suggested Commands

## Development Commands

### Python Development
```bash
# Make scripts executable
chmod +x btrbk_restore.py btrbk_restore_tui_pro.py

# Run CLI version (requires root)
sudo ./btrbk_restore.py

# Run TUI Pro version (requires root)
sudo ./btrbk_restore_tui_pro.py

# Check Python version
python3 --version
```

### Rust Development
```bash
# Build Rust version
cd btrbk_tui_rust
cargo build --release

# Run Rust version (requires root)
sudo ./target/release/btrbk_restore

# Check Rust version
rustc --version
cargo --version
```

### System Commands
```bash
# Check btrfs tools
which btrfs btrbk

# List project files
ls -la

# Check snapshots directory (if accessible)
ls -la /mnt/btr_pool/btrbk_snapshots/

# View configuration
cat ~/.config/btrbk_tui/config.json
```

### Git Commands
```bash
# Standard git workflow
git status
git add .
git commit -m "message"
git push

# View project structure
find . -type f -name "*.py" -o -name "*.rs" -o -name "*.toml"
```

## Testing Commands
```bash
# Test CLI version (dry run - will show snapshots if available)
sudo ./btrbk_restore.py

# Test TUI version
sudo ./btrbk_restore_tui_pro.py

# Test Rust build
cd btrbk_tui_rust && cargo check
```