use ncurses::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    btr_pool_dir: String,
    snapshots_dir: String,
    auto_cleanup: bool,
    confirm_actions: bool,
    show_timestamps: bool,
    theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            btr_pool_dir: "/mnt/btr_pool".to_string(),
            snapshots_dir: "/mnt/btr_pool/btrbk_snapshots".to_string(),
            auto_cleanup: false,
            confirm_actions: true,
            show_timestamps: true,
            theme: "default".to_string(),
        }
    }
}

struct App {
    config: Config,
    config_path: PathBuf,
    current_screen: String,
    selected_row: i32,
    selected_col: i32,
    status_message: String,
    status_timeout: i32,
    reboot_needed: bool,  // Track if reboot is needed
}

impl App {
    fn new() -> Self {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join(".config")
            .join("btrbk_restore")
            .join("config.json");
        
        let mut app = App {
            config: Config::default(),
            config_path,
            current_screen: "main".to_string(),
            selected_row: 0,
            selected_col: 0,
            status_message: String::new(),
            status_timeout: 0,
            reboot_needed: false,  // Initialize reboot flag
        };
        
        app.load_config();
        app
    }
    
    fn load_config(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.config_path) {
            if let Ok(saved_config) = serde_json::from_str::<Config>(&content) {
                self.config = saved_config;
            }
        }
    }
    
    fn save_config(&self) -> bool {
        if let Some(parent) = self.config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        match serde_json::to_string_pretty(&self.config) {
            Ok(json) => fs::write(&self.config_path, json).is_ok(),
            Err(_) => false,
        }
    }
    
    fn get_snapshots(&self) -> (Vec<String>, Vec<String>, Vec<String>) {
        match fs::read_dir(&self.config.snapshots_dir) {
            Ok(entries) => {
                let folders: Vec<String> = entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        if entry.path().is_dir() {
                            Some(entry.file_name().to_string_lossy().into_owned())
                        } else {
                            None
                        }
                    })
                    .collect();
                
                let mut root_snapshots: Vec<String> = folders
                    .iter()
                    .filter(|f| f.starts_with("@.") && !f.starts_with("@home.") && !f.starts_with("@games."))
                    .cloned()
                    .collect();
                
                let mut home_snapshots: Vec<String> = folders
                    .iter()
                    .filter(|f| f.starts_with("@home."))
                    .cloned()
                    .collect();
                
                let mut games_snapshots: Vec<String> = folders
                    .iter()
                    .filter(|f| f.starts_with("@games."))
                    .cloned()
                    .collect();
                
                // Sort by date (newest first)
                root_snapshots.sort_by(|a, b| b.cmp(a));
                home_snapshots.sort_by(|a, b| b.cmp(a));
                games_snapshots.sort_by(|a, b| b.cmp(a));
                
                (root_snapshots, home_snapshots, games_snapshots)
            }
            Err(_) => (Vec::new(), Vec::new(), Vec::new()),
        }
    }
    
    fn format_snapshot_name(&self, snapshot: &str) -> String {
        if !self.config.show_timestamps {
            return snapshot.to_string();
        }
        
        let timestamp_str = if snapshot.starts_with("@.") {
            &snapshot[2..]
        } else if snapshot.starts_with("@home.") {
            &snapshot[6..]
        } else if snapshot.starts_with("@games.") {
            &snapshot[7..]
        } else {
            return snapshot.to_string();
        };
        
        if let Ok(dt) = NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d_%H%M%S") {
            format!("{} ({})", snapshot, dt.format("%Y-%m-%d %H:%M:%S"))
        } else {
            snapshot.to_string()
        }
    }
    
    fn init_colors(&self) {
        start_color();
        use_default_colors();
        
        init_pair(1, COLOR_BLACK, COLOR_CYAN);    // Selected item
        init_pair(2, COLOR_RED, -1);              // Headers
        init_pair(3, COLOR_GREEN, -1);            // Success
        init_pair(4, COLOR_YELLOW, -1);           // Warning
        init_pair(5, COLOR_WHITE, COLOR_BLUE);    // Status bar
        init_pair(6, COLOR_CYAN, -1);             // Info
    }
    
    fn set_status(&mut self, message: &str, timeout: i32) {
        self.status_message = message.to_string();
        self.status_timeout = timeout;
    }
    
    fn purge_old_snapshots(&self) -> (i32, Vec<String>) {
        let snapshots_dir = &self.config.snapshots_dir;
        
        match fs::read_dir(snapshots_dir) {
            Ok(entries) => {
                let mut all_snapshots: Vec<String> = entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        if entry.path().is_dir() {
                            let name = entry.file_name().to_string_lossy().into_owned();
                            if name.starts_with("@.") || name.starts_with("@home.") || name.starts_with("@games.") {
                                Some(entry.path().to_string_lossy().into_owned())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                
                if all_snapshots.is_empty() {
                    return (0, Vec::new());
                }
                
                // Sort snapshots
                all_snapshots.sort();
                
                // Group by type and find old snapshots to delete
                let mut to_delete = Vec::new();
                
                let process_type = |prefix: &str, snapshots: &[String], to_delete: &mut Vec<String>| {
                    let type_snapshots: Vec<&String> = snapshots
                        .iter()
                        .filter(|s| {
                            let basename = s.split('/').last().unwrap_or("");
                            basename.starts_with(&format!("{}.", prefix))
                        })
                        .collect();
                    
                    if type_snapshots.len() > 1 {
                        // Keep the last (most recent) one, delete the rest
                        for snapshot in &type_snapshots[..type_snapshots.len() - 1] {
                            to_delete.push((*snapshot).clone());
                        }
                    }
                };
                
                process_type("@", &all_snapshots, &mut to_delete);
                process_type("@home", &all_snapshots, &mut to_delete);
                process_type("@games", &all_snapshots, &mut to_delete);
                
                if to_delete.is_empty() {
                    return (0, Vec::new());
                }
                
                // Delete old snapshots
                let mut deleted_count = 0;
                let deleted_names: Vec<String> = to_delete
                    .iter()
                    .map(|path| path.split('/').last().unwrap_or("").to_string())
                    .collect();
                
                for snapshot_path in &to_delete {
                    if run_command(&["btrfs", "subvolume", "delete", snapshot_path]) {
                        deleted_count += 1;
                    }
                }
                
                (deleted_count, deleted_names)
            }
            Err(_) => (-1, Vec::new()), // Error occurred
        }
    }
    
    fn draw_header(&self) {
        let (_, width) = get_max_yx();
        
        let title = "BTRBK Restore Tool v2.0";
        attron(COLOR_PAIR(5) | A_BOLD());
        let centered_title = format!("{:^width$}", title, width = width as usize);
        mvaddstr(0, 0, &centered_title[..std::cmp::min(centered_title.len(), width as usize - 1)]);
        attroff(COLOR_PAIR(5) | A_BOLD());
        
        mvaddstr(1, 0, &"-".repeat(std::cmp::min(width as usize - 1, 120)));
    }
    
    fn draw_footer(&self) {
        let (height, width) = get_max_yx();
        
        // Key bindings - show H: Reboot when needed
        let keys = if self.reboot_needed {
            vec![
                "Up/Down: Navigate", "Left/Right: Switch", "ENTER: Select",
                "S: Settings", "R: Refresh", "P: Purge", "H: REBOOT", "Q: Quit"
            ]
        } else {
            vec![
                "Up/Down: Navigate", "Left/Right: Switch", "ENTER: Select",
                "S: Settings", "R: Refresh", "P: Purge", "Q: Quit"
            ]
        };
        let footer_text = keys.join(" | ");
        
        attron(COLOR_PAIR(5));
        mvaddstr(height - 2, 0, &"-".repeat(std::cmp::min(width as usize - 1, 120)));
        mvaddstr(height - 1, 0, &footer_text[..std::cmp::min(footer_text.len(), width as usize - 1)]);
        attroff(COLOR_PAIR(5));
    }
    
    fn draw_status(&mut self) {
        let (height, width) = get_max_yx();
        
        // Show reboot warning if needed (persistent)
        if self.reboot_needed {
            attron(COLOR_PAIR(4) | A_BOLD());  // Yellow/Warning color
            let warning_msg = "WARNING: REBOOT REQUIRED - Press H to reboot system";
            mvaddstr(height - 3, 0, &warning_msg[..std::cmp::min(warning_msg.len(), width as usize - 1)]);
            attroff(COLOR_PAIR(4) | A_BOLD());
        } else if !self.status_message.is_empty() && self.status_timeout > 0 {
            // Show temporary status messages
            attron(COLOR_PAIR(6));
            mvaddstr(height - 3, 0, &self.status_message[..std::cmp::min(self.status_message.len(), width as usize - 1)]);
            attroff(COLOR_PAIR(6));
            self.status_timeout -= 1;
        } else if self.status_timeout <= 0 {
            self.status_message.clear();
        }
    }
    
    fn draw_main_screen(&self) {
        let (height, width) = get_max_yx();
        let (root_snapshots, home_snapshots, games_snapshots) = self.get_snapshots();
        
        if root_snapshots.is_empty() && home_snapshots.is_empty() && games_snapshots.is_empty() {
            attron(COLOR_PAIR(4) | A_BOLD());
            mvaddstr(height / 2, (width - 20) / 2, "No snapshots found!");
            attroff(COLOR_PAIR(4) | A_BOLD());
            return;
        }
        
        let col_width = (width - 8) / 3;
        let col1_x = 2;
        let col2_x = col1_x + col_width;
        let col3_x = col2_x + col_width;
        let start_y = 4;
        
        // Column headers
        attron(COLOR_PAIR(2) | A_BOLD());
        mvaddstr(start_y - 1, col1_x, &format!("ROOT ({})", root_snapshots.len()));
        mvaddstr(start_y - 1, col2_x, &format!("HOME ({})", home_snapshots.len()));
        mvaddstr(start_y - 1, col3_x, &format!("GAMES ({})", games_snapshots.len()));
        attroff(COLOR_PAIR(2) | A_BOLD());
        
        let max_display = height - 8;
        
        // Draw snapshots
        for (i, snapshot) in root_snapshots.iter().enumerate().take(max_display as usize) {
            if start_y + i as i32 >= height - 4 { break; }
            let y = start_y + i as i32;
            let display_name = self.format_snapshot_name(snapshot);
            let truncated = &display_name[..std::cmp::min(display_name.len(), col_width as usize - 2)];
            
            if self.selected_col == 0 && i as i32 == self.selected_row {
                attron(COLOR_PAIR(1));
                mvaddstr(y, col1_x, truncated);
                attroff(COLOR_PAIR(1));
            } else {
                mvaddstr(y, col1_x, truncated);
            }
        }
        
        for (i, snapshot) in home_snapshots.iter().enumerate().take(max_display as usize) {
            if start_y + i as i32 >= height - 4 { break; }
            let y = start_y + i as i32;
            let display_name = self.format_snapshot_name(snapshot);
            let truncated = &display_name[..std::cmp::min(display_name.len(), col_width as usize - 2)];
            
            if self.selected_col == 1 && i as i32 == self.selected_row {
                attron(COLOR_PAIR(1));
                mvaddstr(y, col2_x, truncated);
                attroff(COLOR_PAIR(1));
            } else {
                mvaddstr(y, col2_x, truncated);
            }
        }
        
        for (i, snapshot) in games_snapshots.iter().enumerate().take(max_display as usize) {
            if start_y + i as i32 >= height - 4 { break; }
            let y = start_y + i as i32;
            let display_name = self.format_snapshot_name(snapshot);
            let truncated = &display_name[..std::cmp::min(display_name.len(), col_width as usize - 2)];
            
            if self.selected_col == 2 && i as i32 == self.selected_row {
                attron(COLOR_PAIR(1));
                mvaddstr(y, col3_x, truncated);
                attroff(COLOR_PAIR(1));
            } else {
                mvaddstr(y, col3_x, truncated);
            }
        }
        
        // Config info
        let config_info = format!("Pool: {} | Snapshots: {}", self.config.btr_pool_dir, self.config.snapshots_dir);
        attron(A_DIM());
        mvaddstr(2, 2, &config_info[..std::cmp::min(config_info.len(), width as usize - 4)]);
        attroff(A_DIM());
    }
    
    fn draw_settings_screen(&self) {
        let (height, width) = get_max_yx();
        let settings = vec![
            ("BTR Pool Directory", "btr_pool_dir"),
            ("Snapshots Directory", "snapshots_dir"),
            ("Auto Cleanup .BROKEN", "auto_cleanup"),
            ("Confirm Actions", "confirm_actions"),
            ("Show Timestamps", "show_timestamps"),
        ];
        
        let start_y = 4;
        
        attron(COLOR_PAIR(2) | A_BOLD());
        mvaddstr(start_y - 1, 4, "SETTINGS");
        attroff(COLOR_PAIR(2) | A_BOLD());
        
        for (i, (label, key)) in settings.iter().enumerate() {
            if start_y + (i * 2) as i32 >= height - 8 { break; }
            
            let y = start_y + (i * 2) as i32;
            let value = match *key {
                "btr_pool_dir" => &self.config.btr_pool_dir,
                "snapshots_dir" => &self.config.snapshots_dir,
                "auto_cleanup" => if self.config.auto_cleanup { "Yes" } else { "No" },
                "confirm_actions" => if self.config.confirm_actions { "Yes" } else { "No" },
                "show_timestamps" => if self.config.show_timestamps { "Yes" } else { "No" },
                _ => "",
            };
            
            if i as i32 == self.selected_row {
                attron(COLOR_PAIR(1));
            }
            
            mvaddstr(y, 4, &format!("{}:", label)[..std::cmp::min(label.len() + 1, width as usize - 6)]);
            mvaddstr(y + 1, 6, &value[..std::cmp::min(value.len(), width as usize - 8)]);
            
            if i as i32 == self.selected_row {
                attroff(COLOR_PAIR(1));
            }
        }
        
        // Config file info
        attron(A_DIM());
        let config_path = format!("Config: {}", self.config_path.display());
        let config_exists = if self.config_path.exists() { "EXISTS" } else { "NOT FOUND" };
        let config_info = format!("{} ({})", config_path, config_exists);
        mvaddstr(height - 7, 4, &config_info[..std::cmp::min(config_info.len(), width as usize - 6)]);
        mvaddstr(height - 6, 4, "ENTER: Edit | SPACE: Toggle | ESC: Back | S: Save");
        attroff(A_DIM());
    }
    
    fn confirm_dialog(&self, message: &str) -> bool {
        if !self.config.confirm_actions {
            return true;
        }
        
        let (height, width) = get_max_yx();
        let dialog_width = std::cmp::min(message.len() + 10, width as usize - 4);
        let dialog_height = 5;
        let dialog_y = height / 2 - 2;
        let dialog_x = (width as usize - dialog_width) / 2;
        
        // Draw dialog
        for i in 0..dialog_height {
            mvaddstr(dialog_y + i, dialog_x as i32, &" ".repeat(dialog_width));
        }
        
        let top_border = format!("+{}+", "-".repeat(dialog_width - 2));
        mvaddstr(dialog_y, dialog_x as i32, &top_border);
        mvaddstr(dialog_y + dialog_height - 1, dialog_x as i32, &top_border);
        for i in 1..dialog_height - 1 {
            mvaddstr(dialog_y + i, dialog_x as i32, "|");
            mvaddstr(dialog_y + i, (dialog_x + dialog_width - 1) as i32, "|");
        }
        
        mvaddstr(dialog_y + 1, (dialog_x + 2) as i32, &message[..std::cmp::min(message.len(), dialog_width - 4)]);
        mvaddstr(dialog_y + 3, (dialog_x + 2) as i32, "Y: Yes | N: No");
        refresh();
        
        loop {
            match getch() {
                121 | 89 => return true,  // 'y' or 'Y'
                110 | 78 | 27 => return false,  // 'n' or 'N' or ESC
                _ => continue,
            }
        }
    }
    
    fn restore_snapshot(&self, snapshot: &str, snapshot_type: &str) -> bool {
        let source_path = Path::new(&self.config.snapshots_dir).join(snapshot);
        
        let (current_subvol, broken_subvol, new_subvol) = match snapshot_type {
            "root" => (
                Path::new(&self.config.btr_pool_dir).join("@"),
                Path::new(&self.config.btr_pool_dir).join("@.BROKEN"),
                Path::new(&self.config.btr_pool_dir).join("@"),
            ),
            "home" => (
                Path::new(&self.config.btr_pool_dir).join("@home"),
                Path::new(&self.config.btr_pool_dir).join("@home.BROKEN"),
                Path::new(&self.config.btr_pool_dir).join("@home"),
            ),
            "games" => (
                Path::new(&self.config.btr_pool_dir).join("@games"),
                Path::new(&self.config.btr_pool_dir).join("@games.BROKEN"),
                Path::new(&self.config.btr_pool_dir).join("@games"),
            ),
            _ => return false,
        };
        
        // Move current to .BROKEN
        if !run_command(&["mv", &current_subvol.to_string_lossy(), &broken_subvol.to_string_lossy()]) {
            return false;
        }
        
        // Create new snapshot
        if !run_command(&["btrfs", "subvolume", "snapshot", &source_path.to_string_lossy(), &new_subvol.to_string_lossy()]) {
            return false;
        }
        
        // Auto cleanup if enabled
        if self.config.auto_cleanup {
            run_command(&["btrfs", "subvolume", "delete", &broken_subvol.to_string_lossy()]);
        }
        
        true
    }
    
    fn handle_main_input(&mut self, key: i32) {
        let (root_snapshots, home_snapshots, games_snapshots) = self.get_snapshots();
        
        match key {
            KEY_UP => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                }
            }
            KEY_DOWN => {
                let max_rows = match self.selected_col {
                    0 => root_snapshots.len() as i32,
                    1 => home_snapshots.len() as i32,
                    2 => games_snapshots.len() as i32,
                    _ => 0,
                };
                if self.selected_row < max_rows - 1 {
                    self.selected_row += 1;
                }
            }
            KEY_LEFT => {
                if self.selected_col > 0 {
                    self.selected_col -= 1;
                    let max_rows = match self.selected_col {
                        0 => root_snapshots.len() as i32,
                        1 => home_snapshots.len() as i32,
                        _ => 0,
                    };
                    if self.selected_row >= max_rows && max_rows > 0 {
                        self.selected_row = max_rows - 1;
                    }
                }
            }
            KEY_RIGHT => {
                if self.selected_col < 2 {
                    self.selected_col += 1;
                    let max_rows = match self.selected_col {
                        1 => home_snapshots.len() as i32,
                        2 => games_snapshots.len() as i32,
                        _ => 0,
                    };
                    if self.selected_row >= max_rows && max_rows > 0 {
                        self.selected_row = max_rows - 1;
                    }
                }
            }
            10 | 13 => {  // ENTER
                let (snapshot, snapshot_type) = match self.selected_col {
                    0 if !root_snapshots.is_empty() && (self.selected_row as usize) < root_snapshots.len() => {
                        (&root_snapshots[self.selected_row as usize], "root")
                    }
                    1 if !home_snapshots.is_empty() && (self.selected_row as usize) < home_snapshots.len() => {
                        (&home_snapshots[self.selected_row as usize], "home")
                    }
                    2 if !games_snapshots.is_empty() && (self.selected_row as usize) < games_snapshots.len() => {
                        (&games_snapshots[self.selected_row as usize], "games")
                    }
                    _ => return,
                };
                
                if !self.confirm_dialog(&format!("Restore {} snapshot?", snapshot_type)) {
                    self.set_status("Restoration cancelled", 50);
                    return;
                }
                
                self.set_status("Restoring snapshot...", 100);
                refresh();
                
                if self.restore_snapshot(snapshot, snapshot_type) {
                    self.reboot_needed = true;  // Set reboot flag
                    self.set_status("Snapshot restored! Press H to reboot or continue working", 200);
                    if self.confirm_dialog("Reboot system now?") {
                        run_command(&["reboot"]);
                    }
                } else {
                    self.set_status("Failed to restore snapshot!", 100);
                }
            }
            115 | 83 => {  // 's' or 'S'
                self.current_screen = "settings".to_string();
                self.selected_row = 0;
            }
            114 | 82 => {  // 'r' or 'R'
                // Always refresh
                self.set_status("Refreshed snapshot list", 50);
            }
            104 | 72 => {  // 'h' or 'H'
                // Reboot if needed
                if self.reboot_needed {
                    if self.confirm_dialog("Reboot system now?") {
                        run_command(&["reboot"]);
                    } else {
                        self.set_status("Reboot cancelled", 50);
                    }
                } else {
                    self.set_status("No reboot needed", 50);
                }
            }
            112 | 80 => {  // 'p' or 'P'
                // Purge old snapshots
                if self.confirm_dialog("Purge old snapshots (keep only most recent)?") {
                    self.set_status("Purging old snapshots...", 100);
                    refresh();
                    
                    let (deleted_count, _deleted_list) = self.purge_old_snapshots();
                    
                    if deleted_count == -1 {
                        self.set_status("Error during purge operation!", 100);
                    } else if deleted_count == 0 {
                        self.set_status("No old snapshots to purge", 100);
                    } else {
                        self.set_status(&format!("Purged {} old snapshots successfully", deleted_count), 150);
                    }
                } else {
                    self.set_status("Purge cancelled", 50);
                }
            }
            _ => {}
        }
    }
    
    fn handle_settings_input(&mut self, key: i32) {
        match key {
            KEY_UP => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                }
            }
            KEY_DOWN => {
                if self.selected_row < 4 {
                    self.selected_row += 1;
                }
            }
            10 | 13 => {  // ENTER
                self.edit_setting();
            }
            32 => {  // SPACE
                self.toggle_setting();
            }
            115 | 83 => {  // 's' or 'S'
                if self.save_config() {
                    self.set_status("Settings saved manually!", 50);
                } else {
                    self.set_status("Failed to save settings!", 50);
                }
            }
            27 => {  // ESC
                self.current_screen = "main".to_string();
                self.selected_row = 0;
            }
            _ => {}
        }
    }
    
    fn edit_setting(&mut self) {
        match self.selected_row {
            0 | 1 => {  // String settings
                let (height, width) = get_max_yx();
                let field_name = if self.selected_row == 0 { "btr_pool_dir" } else { "snapshots_dir" };
                let current_value = if self.selected_row == 0 { &self.config.btr_pool_dir } else { &self.config.snapshots_dir };
                
                // Clear area for input
                for i in 0..5 {
                    mvaddstr(height / 2 - 2 + i, 4, &" ".repeat(width as usize - 8));
                }
                
                mvaddstr(height / 2 - 1, 4, &format!("Edit {}: ", field_name));
                mvaddstr(height / 2, 4, &format!("Current: {}", current_value));
                mvaddstr(height / 2 + 1, 4, "New: ");
                mvaddstr(height / 2 + 3, 4, "Press ENTER to confirm, ESC to cancel");
                refresh();
                
                curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
                echo();
                
                let mut input = String::new();
                let mut ch = getch();
                
                while ch != 10 && ch != 13 && ch != 27 {
                    if ch == KEY_BACKSPACE || ch == 127 || ch == 8 {
                        if !input.is_empty() {
                            input.pop();
                            mvaddstr(height / 2 + 1, 9, &format!("{} ", input));
                        }
                    } else if ch >= 32 && ch < 127 {
                        input.push(ch as u8 as char);
                        mvaddstr(height / 2 + 1, 9, &input);
                    }
                    refresh();
                    ch = getch();
                }
                
                noecho();
                curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                
                if ch != 27 && !input.trim().is_empty() {
                    if self.selected_row == 0 {
                        self.config.btr_pool_dir = input.trim().to_string();
                    } else {
                        self.config.snapshots_dir = input.trim().to_string();
                    }
                    self.save_config();
                    self.set_status(&format!("Updated {}", field_name), 50);
                } else {
                    self.set_status("Edit cancelled", 50);
                }
            }
            2 | 3 | 4 => {  // Boolean settings
                self.toggle_setting();
            }
            _ => {}
        }
    }
    
    fn toggle_setting(&mut self) {
        match self.selected_row {
            2 => {
                self.config.auto_cleanup = !self.config.auto_cleanup;
                self.save_config();
                self.set_status("Toggled auto_cleanup", 50);
            }
            3 => {
                self.config.confirm_actions = !self.config.confirm_actions;
                self.save_config();
                self.set_status("Toggled confirm_actions", 50);
            }
            4 => {
                self.config.show_timestamps = !self.config.show_timestamps;
                self.save_config();
                self.set_status("Toggled show_timestamps", 50);
            }
            _ => {}
        }
    }
    
    fn run(&mut self) {
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        timeout(100);
        self.init_colors();
        
        loop {
            clear();
            
            self.draw_header();
            
            match self.current_screen.as_str() {
                "main" => self.draw_main_screen(),
                "settings" => self.draw_settings_screen(),
                _ => {}
            }
            
            self.draw_status();
            self.draw_footer();
            
            refresh();
            
            let key = getch();
            
            if key == -1 {
                continue;
            } else if key == 113 || key == 81 {  // 'q' or 'Q'
                break;
            } else {
                match self.current_screen.as_str() {
                    "main" => self.handle_main_input(key),
                    "settings" => self.handle_settings_input(key),
                    _ => {}
                }
            }
        }
    }
}

fn run_command(cmd: &[&str]) -> bool {
    Command::new(cmd[0])
        .args(&cmd[1..])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn get_max_yx() -> (i32, i32) {
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    (max_y, max_x)
}

fn main() {
    // Check for root privileges
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("Error: This tool requires root privileges.");
        eprintln!("Please run with sudo.");
        std::process::exit(1);
    }
    
    // Initialize ncurses
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    
    // Create and run the TUI app
    let mut app = App::new();
    app.run();
    
    // Cleanup
    endwin();
}
