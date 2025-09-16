# Code Style and Conventions

## Python Style
- **Shebang**: `#!/usr/bin/env python3` (TUI Pro) o `#!/usr/bin/python` (CLI)
- **Type Hints**: Utilizzati nella versione TUI Pro (`from typing import Dict, List, Optional, Tuple`)
- **Docstrings**: Stile semplice con descrizione funzione
- **Naming**: snake_case per funzioni e variabili
- **Classes**: PascalCase (Config, SnapshotManager, TUIApp)
- **Constants**: UPPER_CASE (CONFIG_FILE, DEFAULT_CONFIG)

## Code Organization
- **Imports**: Standard library prima, poi third-party
- **Configuration**: Costanti globali all'inizio
- **Classes**: Organizzate logicamente (Config, SnapshotManager, TUIApp)
- **Error Handling**: Try-catch con fallback ai valori di default

## Rust Style
- **Edition**: 2024 (Cargo.toml)
- **Dependencies**: ncurses, serde, serde_json, chrono, dirs, libc
- **Naming**: snake_case per funzioni, PascalCase per struct

## File Structure
```
btrbk_restore/
├── btrbk_restore.py              # CLI version
├── btrbk_restore_tui_pro.py      # TUI Pro Python
├── btrbk_restore_rust/           # Rust TUI version
│   ├── Cargo.toml
│   └── src/main.rs
├── README.md
└── snapshot-restore.desktop
```