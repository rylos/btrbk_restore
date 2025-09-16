# Task Completion Guidelines

## When a Task is Completed

### Code Changes
1. **Test the changes**:
   ```bash
   # For Python changes
   sudo ./btrbk_restore.py  # Test CLI
   sudo ./btrbk_restore_tui_pro.py  # Test TUI Pro
   
   # For Rust changes
   cd btrbk_restore_rust
   cargo build --release
   sudo ./target/release/btrbk_restore
   ```

2. **Check syntax and build**:
   ```bash
   # Python syntax check
   python3 -m py_compile btrbk_restore.py
   python3 -m py_compile btrbk_restore_tui_pro.py
   
   # Rust build check
   cd btrbk_restore_rust && cargo check
   ```

### Documentation Updates
- Update README.md if new features or changes affect usage
- Update version numbers in Cargo.toml if Rust changes
- Ensure desktop file is updated if entry points change

### Version Consistency
- Maintain feature parity between Python TUI Pro and Rust versions
- Ensure shared configuration compatibility
- Test configuration file loading/saving

### Security Considerations
- All tools require root privileges (sudo)
- Test with actual btrfs snapshots when possible
- Verify backup creation (.BROKEN files) works correctly

### Final Checks
- Verify executable permissions: `chmod +x *.py`
- Test on actual btrfs system if available
- Ensure error handling works for missing directories
- Check that configuration file creation works

## No Automated Testing
This project doesn't have automated tests - testing is done manually with actual btrfs snapshots.