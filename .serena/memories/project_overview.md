# BTRBK TUI v2.2 - Project Overview

## Current Version: v2.2 - Bug Fixes & Major Improvements

### 🐛 Critical Bug Fixes Applied:
- **Fixed timestamp parsing** - Now supports both `YYYYMMDDTHHMMSS` and `YYYYMMDD_HHMMSS` formats
- **Fixed .BROKEN conflicts** - Unique timestamps prevent restore failures (`@.BROKEN.20250916_112530`)
- **Fixed hardcoded restore logic** - Now fully dynamic for all subvolume types
- **Fixed purge function** - Dynamic detection instead of hardcoded types
- **Fixed merge conflicts** - Rust version now compiles correctly

### 🎯 Available Commands in TUI Versions:

#### Main Screen Commands:
- **↑↓**: Navigate snapshots vertically
- **←→**: Switch between snapshot groups (dynamic columns)
- **ENTER**: Select and restore snapshot
- **S**: Settings screen (persistent configuration)
- **R**: Refresh snapshot list
- **I**: Create new snapshots (btrbk run --progress)
- **P**: Purge OLD snapshots (keeps only most recent per type)
- **B**: Clean BROKEN subvolumes (deletes all .BROKEN.* subvolumes)
- **H**: System reboot (appears after restore operations)
- **Q**: Quit application

#### Settings Screen Commands:
- **↑↓**: Navigate settings options
- **ENTER**: Edit string values
- **SPACE**: Toggle boolean values
- **S**: Manual save (auto-save is active)
- **ESC**: Return to main screen

### 🔧 Dynamic Features:
- **Auto-detection**: Scans for any @prefix snapshots (@, @home, @games, @custom, @backup, etc.)
- **Adaptive interface**: Columns automatically adjust to detected groups
- **Smart sorting**: @ always first, then alphabetical
- **Timestamp formatting**: Human-readable dates in display
- **Unique .BROKEN names**: Prevents conflicts during restore

### 📁 Project Structure:
```
btrbk_tui/
├── btrbk_restore.py              # CLI version (simple)
├── btrbk_restore_tui_pro.py      # Python TUI (professional)
├── btrbk_tui_rust/               # Rust TUI (high-performance)
│   ├── src/main.rs              # Rust source
│   └── target/release/          # Compiled binary
└── README.md                     # Complete documentation
```

### 🎨 Interface Labels:
- **Main bar**: `S: Settings | R: Refresh | I: Snapshot | P: Purge OLD | B: Clean BROKEN | Q: Quit`
- **With reboot**: `S: Settings | R: Refresh | I: Snapshot | P: Purge OLD | B: Clean BROKEN | H: REBOOT | Q: Quit`

### ✅ All Versions Status:
- **Python CLI**: ✅ Fixed timestamp parsing, .BROKEN conflicts, dynamic logic
- **Python TUI**: ✅ All v2.2 fixes + new B command
- **Rust TUI**: ✅ All v2.2 fixes + new B command + merge conflicts resolved

### 🚀 Latest Updates (v2.2):
- Added "B: Clean BROKEN" command to both TUI versions
- Changed "P: Purge" to "P: Purge OLD" for clarity
- All three versions now fully aligned and working
- Complete bug fixes for production use