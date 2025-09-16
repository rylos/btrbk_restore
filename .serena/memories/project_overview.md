# BTRBK Restore Tool - Project Overview

## Purpose
Tool completo per il ripristino di snapshot Btrfs creati con btrbk. Fornisce interfacce multiple (CLI, TUI) per gestire snapshot di subvolumi Btrfs con rilevamento automatico dei tipi di snapshot.

## Tech Stack
- **Python 3.12.8**: Versioni CLI e TUI Pro
- **Rust 1.87.0**: Versione TUI ad alte prestazioni
- **ncurses**: Per interfacce TUI
- **JSON**: Configurazione condivisa tra versioni
- **Btrfs**: Filesystem target
- **btrbk**: Tool per creazione snapshot

## Architecture
- **btrbk_restore.py**: Versione CLI semplice con menu testuale
- **btrbk_restore_tui_pro.py**: Versione TUI professionale con configurazione persistente
- **btrbk_restore_rust/**: Versione Rust TUI identica alla Pro Python
- **Configurazione condivisa**: `~/.config/btrbk_restore/config.json`

## Key Features
- Rilevamento automatico di tutti i tipi di snapshot (@, @home, @games, @custom, etc.)
- Interfaccia dinamica che si adatta al numero di gruppi trovati
- Gestione automatica backup subvolumi esistenti (.BROKEN)
- Configurazione persistente condivisa tra versioni
- Creazione snapshot integrata (btrbk run --progress)
- Pulizia intelligente snapshot vecchi
- Riavvio sistema con indicatori visivi