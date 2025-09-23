# System Requirements and Dependencies

## System Requirements
- **OS**: Linux con filesystem Btrfs
- **Privileges**: Root access (sudo) richiesto per operazioni btrfs
- **Architecture**: x86_64, ARM64 (Rust), tutte le architetture supportate da Python

## Dependencies

### System Dependencies
```bash
# Required system tools
btrfs-progs    # btrfs command
btrbk          # btrbk tool for snapshot creation
```

### Python Dependencies (Built-in)
- `curses` - TUI interface (included in Python standard library)
- `json` - Configuration management
- `os` - System operations
- `subprocess` - Command execution
- `sys` - System interface
- `datetime` - Timestamp handling
- `pathlib` - Path operations
- `typing` - Type hints

### Rust Dependencies (Cargo.toml)
```toml
ncurses = "5.101.0"           # TUI interface
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"            # JSON handling
chrono = { version = "0.4", features = ["serde"] }  # Date/time
dirs = "5.0"                  # Directory paths
libc = "0.2"                  # System calls
```

## Installation Requirements
- **Python**: 3.12.8+ (any Python 3.x should work)
- **Rust**: 1.87.0+ (edition 2024)
- **Desktop Environment**: Testato su KDE Plasma 6, compatibile con altri DE

## Runtime Paths
- **Configuration**: `~/.config/btrbk_tui/config.json`
- **Default Pool**: `/mnt/btr_pool`
- **Default Snapshots**: `/mnt/btr_pool/btrbk_snapshots`