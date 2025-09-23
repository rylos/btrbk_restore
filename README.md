# BTRBK TUI

A comprehensive set of tools for restoring Btrfs snapshots created with btrbk, available in Python and Rust with different user interfaces.

## Description

This project provides tools to easily restore Btrfs subvolume snapshots created by the btrbk tool. The tools allow you to:

- **Automatic detection** of all snapshot types present
- **Dynamic interface** that adapts to the number of groups found
- View available snapshots for all subvolumes
- Select and restore specific snapshots
- Automatically manage backup of existing subvolumes
- Persistent configuration shared between versions
- Intelligent cleanup of old snapshots
- System reboot with visual indicators
- Optionally reboot the system after restoration

## ✨ Features v2.5 - Bug Fixes & Improvements

### 🔄 **Automatic Detection:**
- **No longer limited** to 3 fixed types (@, @home, @games)
- **Automatically scans** the snapshots directory
- **Detects any prefix** (@, @home, @games, @custom, @backup, @work, etc.)
- **Automatically adapts** to any user's btrbk configuration

### 🐛 **Critical Bug Fixes:**
- **Fixed timestamp parsing** - Now supports both `YYYYMMDDTHHMMSS` and `YYYYMMDD_HHMMSS` formats
- **Fixed .BROKEN conflicts** - Unique timestamps prevent restore failures
- **Fixed hardcoded restore logic** - Now fully dynamic for all subvolume types
- **Fixed purge function** - Dynamic detection instead of hardcoded types
- **Simplified log display** - Removed problematic side borders

### 📊 **Adaptive Interface:**
- **Dynamic columns**: Number of columns adapts to groups found
- **Automatic width**: Columns resize automatically
- **Smart sorting**: @ always first, then alphabetical order
- **Snapshot count**: Shows number of snapshots per group

### 🎯 **Supported Configuration Examples:**
```
Basic User:     @ | @home
Gaming User:    @ | @home | @games  
Pro User:       @ | @home | @games | @work | @backup
Server User:    @ | @home | @var | @opt | @srv | @data
```

### 🎨 **Enhanced Interface:**
- **Separator lines** at full screen width
- **Perfect visual consistency** between header and footer
- **Optimized colors** for better readability

### 🎯 **Supported Configuration Examples:**
```
Basic User:     @ | @home
Gaming User:    @ | @home | @games  
Pro User:       @ | @home | @games | @work | @backup
Server User:    @ | @home | @var | @opt | @srv | @data
```

### 🎨 **Enhanced Interface:**
- **Separator lines** at full screen width
- **Perfect visual consistency** between header and footer
- **Optimized colors** for better readability

## Available Versions

### Python
- **`btrbk_tui.py`** - Simple CLI version with text menu
- **`btrbk_tui_pro.py`** - Professional TUI interface with persistent configuration and dynamic columns

### Rust
- **`btrbk_tui_rust/`** - High-performance TUI version written in Rust with ncurses (identical to Python Pro version)

## Prerequisites

### For Python versions:
```bash
# Basic CLI version
python3

# Professional TUI version
python3 (with curses module included)
```

### For Rust version:
```bash
# Rust installation (edition 2021)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cd btrbk_tui_rust
cargo build --release
```

## Configuration

The tool assumes by default:
- **Btrfs Pool**: `/mnt/btr_pool`
- **Snapshots directory**: `/mnt/btr_pool/btrbk_snapshots`

**Shared Configuration**: The TUI Pro (Python) and Rust versions share the same JSON configuration file at `~/.config/btrbk_tui/config.json`, ensuring a completely consistent user experience.

## Usage

### Python CLI Version
```bash
sudo ./btrbk_tui.py
```

### Python Professional TUI Version
```bash
sudo ./btrbk_tui_pro.py
```

### Rust TUI Version (identical to Python Pro)
```bash
cd btrbk_tui_rust
sudo ./target/release/btrbk_tui
```

## Features

### CLI Version (`btrbk_tui.py`)
- **Numbered list** of all snapshots organized by type
- **Number-based selection** with simple interface
- **Complete dynamic support** for any configuration (@, @home, @games, @custom, @backup, etc.)
- **Simple interface** for occasional use
- **Automatic management** of .BROKEN backups
- **Automatic detection** of all snapshot types present

### Professional TUI Version (`btrbk_tui_pro.py`)
- **Dynamic interface**: Columns that automatically adapt to groups found
- **Persistent configuration**: Automatic saving to `~/.config/btrbk_tui/config.json`
- **Advanced navigation**: Arrow keys for fluid navigation
- **Complete settings screen**: `S` key for advanced configuration
- **Configurable settings**: Directories, auto-cleanup, confirmations, timestamps
- **Status messages**: Real-time operation feedback
- **Themes and colors**: Professional interface with highlighting
- **Auto-save**: Every change is automatically saved
- **Snapshot creation**: Dedicated interface for `btrbk run --progress`
- **Smart purge**: Automatic cleanup of old snapshots
- **Reboot system**: Visual indicators and dedicated shortcuts

### Rust TUI Version (`btrbk_tui_rust/`)
- **Optimized performance**: Native Rust implementation
- **Identical interface**: Layout and functionality identical to Python Pro version
- **Shared configuration**: Uses exactly the same JSON file as Python version
- **Complete settings screen**: Same editing functionality as Python version
- **Efficient memory management**: Ideal for resource-limited systems
- **Total compatibility**: Zero functional differences with Python Pro version
- **Optimized compilation**: Rust edition 2021, zero errors and warnings
- **Snapshot creation**: Multi-threaded interface for real-time output
- **Purge and reboot**: All advanced features implemented

## Supported Snapshot Structure

The tool automatically handles snapshots with this nomenclature:
- `@.YYYYMMDD_HHMMSS` - Root subvolume snapshot
- `@home.YYYYMMDD_HHMMSS` - Home subvolume snapshot
- `@games.YYYYMMDD_HHMMSS` - Games subvolume snapshot
- `@custom.YYYYMMDD_HHMMSS` - Custom subvolume snapshots
- `@backup.YYYYMMDD_HHMMSS` - Backup snapshots
- `@work.YYYYMMDD_HHMMSS` - Work snapshots
- **And any other prefix** that starts with `@` followed by a dot

**The tool automatically adapts** to any user's btrbk configuration!

## TUI Controls

### Dynamic Column Versions (TUI Pro Python/Rust):

#### Main Screen:
- **↑↓**: Vertical navigation through snapshots
- **←→**: Dynamic column switching (adaptive to number of groups)
- **ENTER**: Snapshot selection and restoration
- **S**: Access settings screen
- **R**: Refresh snapshot list
- **I**: Create new snapshots (btrbk run --progress)
- **P**: Purge old snapshots (keeps only most recent per type)
- **H**: System reboot (when needed)
- **Q**: Exit application

#### Settings Screen:
- **↑↓**: Navigate between options
- **ENTER**: Edit value (for strings)
- **SPACE**: Toggle value (for booleans)
- **S**: Manual save (optional, auto-save active)
- **ESC**: Return to main screen

### Advanced Features:

#### Instant Snapshot Creation (I Key):
- **Executes**: `btrbk run --progress` with dedicated interface
- **Real-time output**: Professional progress visualization
- **Dedicated window**: Fullscreen with borders and title
- **Cancellation**: ESC to interrupt operation at any time
- **Auto-scroll**: Automatic scrolling for long output
- **Complete feedback**: Colored success/error messages
- **Stderr handling**: Perfectly aligned output without overlaps

#### Smart Purge (P Key):
- **Analyzes** all snapshots by type (@, @home, @games)
- **Keeps** only the most recent snapshot per type
- **Deletes** all older snapshots automatically
- **Confirmation** before operation for safety
- **Detailed feedback** on how many snapshots were deleted
- **Error handling**: Continues operation even if individual deletions fail
- **Space optimization**: Automatically frees disk space while maintaining essential backups

#### Smart Reboot:
- **R Key**: Always available for snapshot list refresh
- **H Key**: Appears in footer after restore for quick reboot
- **Persistent warning**: Status bar shows "⚠ REBOOT REQUIRED" after each restore
- **Dedicated keys**: R for refresh, H for reboot, I for snapshot, P for purge - no confusion
- **Visual indicators**: Dynamic footer that changes based on context

## Desktop File

Includes `btrbk-tui.desktop` for desktop environment integration.

## Security

⚠️ **WARNING**: These tools require root privileges and modify system subvolumes. Use with caution and always after verifying the presence of valid backups.

### Implemented Security Measures:
- **Mandatory confirmations**: Confirmation dialogs for all critical operations
- **Automatic backup**: Existing subvolumes are renamed to .BROKEN before restoration
- **Error handling**: Robust operations with fallback and clear error messages
- **Optional auto-cleanup**: Configurable automatic cleanup of .BROKEN files

## Compatibility

- **Operating System**: Linux with Btrfs filesystem
- **Dependencies**: btrfs-progs, btrbk
- **Desktop**: Tested on KDE Plasma, compatible with other DEs
- **Supported subvolumes**: Any configuration starting with @ (dynamic)
- **Architectures**: x86_64, ARM64 (Rust), all architectures supported by Python

## Advanced Configuration

Both TUI versions (Python Pro and Rust) share the configuration saved at:
```
~/.config/btrbk_tui/config.json
```

### Configurable settings:
- **btr_pool_dir**: Btrfs pool directory (default: `/mnt/btr_pool`)
- **snapshots_dir**: Snapshots directory (default: `/mnt/btr_pool/btrbk_snapshots`)
- **auto_cleanup**: Auto-cleanup of .BROKEN files (default: `false`)
- **confirm_actions**: Action confirmation (default: `true`)
- **show_timestamps**: Display formatted timestamps (default: `true`)
- **theme**: Interface theme (default: `"default"`)

### Example configuration file:
```json
{
  "btr_pool_dir": "/mnt/btr_pool",
  "snapshots_dir": "/mnt/btr_pool/btrbk_snapshots",
  "auto_cleanup": false,
  "confirm_actions": true,
  "show_timestamps": true,
  "theme": "default"
}
```

### Configuration Management:
- **Automatic loading**: At startup of any TUI version
- **Automatic saving**: On every change in TUI versions
- **Synchronization**: Changes in one version apply immediately to the other
- **Fallback**: If file is corrupted or missing, default values are used

## Which Version to Choose?

### **CLI (`btrbk_tui.py`)**
- ✅ Occasional or sporadic use
- ✅ Automated scripts
- ✅ Resource-limited environments
- ✅ When only basic functionality is needed

### **TUI Pro (`btrbk_tui_pro.py`)**
- ✅ Frequent and interactive use
- ✅ Advanced configuration and customization
- ✅ When Python is preferred for modifications
- ✅ Development and debugging
- ✅ Complete snapshot management
- ✅ Dynamic interface that adapts to any configuration

### **Rust (`btrbk_tui_rust/`)**
- ✅ Maximum performance and speed
- ✅ Systems with limited or absent Python
- ✅ Production environments
- ✅ When memory efficiency is needed
- ✅ All Pro version features
- ✅ Dynamic interface identical to Python version

## Benefits of Complete Alignment

### **Unified Configuration:**
- Single configuration file for both TUI versions
- Automatically synchronized changes
- Consistent user experience

### **Identical Features:**
- Same interface and controls
- Same configuration options
- Same behavior and workflow
- Same advanced features (purge, reboot, settings)

### **Total Flexibility:**
- Switch from Python to Rust without losing configurations
- Choose language based on specific needs
- Simplified maintenance with shared configuration

### **Optimized Performance:**
- Python: Ease of modification and debugging
- Rust: Execution speed and memory efficiency
- Both: Same user experience

## Project Structure

```
btrbk_tui/
├── README.md                      # Complete documentation
├── btrbk_tui.py                  # Simple CLI version
├── btrbk_tui_pro.py              # Python professional TUI version
├── btrbk_tui_rust/           # Rust professional TUI version
│   ├── Cargo.toml               # Rust configuration (edition 2021)
│   ├── src/main.rs              # Rust source code
│   └── target/release/          # Compiled binary
├── btrbk-tui.desktop             # Desktop file for DE integration
└── .git/                         # Git repository
```

## Development and Contributions

### **Languages used:**
- **Python 3**: CLI and TUI Pro versions
- **Rust 2021**: High-performance TUI version
- **JSON**: Shared configuration

### **Dependencies:**
- **Python**: `curses`, `json`, `pathlib`, `subprocess`, `os` modules
- **Rust**: `ncurses`, `serde`, `serde_json`, `chrono`, `dirs`, `libc`

### **Testing:**
- Tested on Arch Linux with KDE Plasma 6
- Compatible with other Linux desktop environments
- Full support for Btrfs filesystem

## Typical Usage Workflow

1. **Startup**: `sudo ./btrbk_tui_pro.py` or Rust version
2. **Navigation**: Use arrows to explore available snapshots
3. **Configuration**: Press `S` to modify settings if needed
4. **Snapshot creation**: Use `I` to create new snapshots with btrbk
5. **Selection**: Choose snapshot to restore with `ENTER`
6. **Confirmation**: Confirm the restoration operation
7. **Reboot**: Choose whether to reboot immediately or continue
8. **Cleanup**: Use `P` to delete old snapshots when needed
9. **Quick reboot**: Use `H` to reboot when indicated

## License

Open source project - see source code for implementation details.

## Contributing

Contributions welcome! The project demonstrates implementing the same functionality in different languages (Python/Rust) with interfaces optimized for different use cases, while maintaining full configuration compatibility and identical user experience.

### **Project characteristics:**
- Modular and well-structured architecture
- Shared configuration between different languages
- Professional and intuitive user interfaces
- Robust error handling
- Performance optimized for each language
- Complete and up-to-date documentation
- Advanced snapshot management features
- Integrated security system

## Installation

### Quick Start
```bash
# Clone the repository
git clone https://github.com/rylos/btrbk_tui.git
cd btrbk_tui

# Make scripts executable
chmod +x btrbk_tui.py btrbk_tui_pro.py

# For Rust version
cd btrbk_tui_rust
cargo build --release
cd ..

# Run (requires root privileges)
sudo ./btrbk_tui_pro.py
```

### Requirements Check
```bash
# Verify btrfs tools
which btrfs btrbk

# Verify Python
python3 --version

# Verify Rust (for Rust version)
rustc --version
```

## Screenshots

The dynamic interface automatically adapts to your btrbk configuration:

**2 Groups (Basic):**
```
@ (3) | @HOME (2)
```

**4 Groups (Advanced):**
```
@ (3) | @HOME (2) | @GAMES (4) | @WORK (1)
```

**6+ Groups (Server):**
```
@ (3) | @HOME (2) | @VAR (1) | @OPT (2) | @SRV (1) | @DATA (3)
```

## Support

- **Issues**: Report bugs or request features via GitHub Issues
- **Documentation**: Complete documentation in this README
- **Community**: Contributions and feedback welcome

---

**Made with ❤️ for the Btrfs and btrbk community**
