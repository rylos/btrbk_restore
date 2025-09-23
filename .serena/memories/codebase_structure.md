# Codebase Structure

## Main Files

### btrbk_restore.py (CLI Version)
- **Purpose**: Versione CLI semplice con menu numerato
- **Key Functions**:
  - `get_snapshot_groups()`: Rileva dinamicamente gruppi snapshot
  - `display_snapshots()`: Mostra lista numerata
  - `restore_snapshot()`: Esegue ripristino
  - `main()`: Loop principale
- **Dependencies**: os, subprocess
- **Size**: ~150 righe

### btrbk_restore_tui_pro.py (TUI Pro Version)
- **Purpose**: Interfaccia TUI professionale con configurazione persistente
- **Key Classes**:
  - `Config`: Gestione configurazione JSON
  - `SnapshotManager`: Gestione snapshot e operazioni btrfs
  - `TUIApp`: Interfaccia utente curses
- **Features**: Navigazione frecce, configurazione, creazione snapshot, pulizia
- **Dependencies**: curses, json, os, subprocess, sys, datetime, pathlib, typing
- **Size**: ~1000+ righe

### btrbk_tui_rust/ (Rust Version)
- **Purpose**: Versione TUI ad alte prestazioni identica alla Pro Python
- **Structure**:
  - `Cargo.toml`: Configurazione progetto Rust
  - `src/main.rs`: Implementazione completa
- **Dependencies**: ncurses, serde, serde_json, chrono, dirs, libc
- **Features**: Identiche alla versione Python Pro

## Configuration
- **Shared Config**: `~/.config/btrbk_tui/config.json`
- **Default Paths**: 
  - Pool: `/mnt/btr_pool`
  - Snapshots: `/mnt/btr_pool/btrbk_snapshots`

## Desktop Integration
- **snapshot-restore.desktop**: File desktop per integrazione DE