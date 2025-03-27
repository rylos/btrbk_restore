use ncurses::*;
use std::fs;
use std::path::Path;
use std::process::Command;

const BTR_POOL_DIR_DEFAULT: &str = "/mnt/btr_pool";
const SNAPSHOTS_DIR_DEFAULT: &str = "/mnt/btr_pool/btrbk_snapshots";

fn get_snapshot_folders(snapshots_dir: &str) -> Vec<String> {
    fs::read_dir(snapshots_dir)
        .unwrap_or_else(|_| panic!("Failed to read directory: {}", snapshots_dir))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.path().is_dir() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

fn run_command(cmd: &[&str]) -> bool {
    Command::new(cmd[0])
        .args(&cmd[1..])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn clear_line(stdscr: WINDOW, y: i32) {
    wmove(stdscr, y, 0);
    wclrtoeol(stdscr);
}

fn confirm_action(stdscr: WINDOW, prompt: &str, y_confirm: i32) -> bool {
    // Pulisci la linea prima di scrivere il prompt
    wmove(stdscr, y_confirm, 0);
    wclrtoeol(stdscr);

    // Scrivi il nuovo prompt
    let prompt_text = format!("{} (s/n): ", prompt);
    mvwaddstr(stdscr, y_confirm, 0, &prompt_text);
    wrefresh(stdscr);

    // Gestisci l'input dell'utente
    loop {
        let c = wgetch(stdscr);
        if c == 's' as i32 || c == 'S' as i32 {
            clear_line(stdscr, y_confirm);
            wrefresh(stdscr);
            return true;
        } else if c == 'n' as i32 || c == 'N' as i32 {
            clear_line(stdscr, y_confirm);
            wrefresh(stdscr);
            return false;
        }
    }
}

fn display_message(message: &str, pause: bool) {
    clear();
    mvaddstr(0, 0, message);
    if pause {
        mvaddstr(2, 0, "Premi un tasto per continuare...");
        refresh();
        getch();
    } else {
        refresh();
    }
}

fn edit_config_fields(stdscr: WINDOW) -> (String, String) {
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); // Rende il cursore visibile
    clear(); // Pulisce lo schermo

    // Istruzioni per l'utente
    mvaddstr(0, 0, "Istruzioni:");
    mvaddstr(1, 0, "Usa le frecce per muoverti, BACKSPACE per cancellare e INVIO per confermare.");
    mvaddstr(3, 0, "Modifica se necessario i campi sottostanti e conferma con INVIO:");

    // Campo per BTR_POOL_DIR
    mvaddstr(5, 2, &format!("BTR_POOL_DIR (default: {}): ", BTR_POOL_DIR_DEFAULT));
    let mut btr_pool_dir = String::new();
    mvwgetstr(stdscr, 5, 18 + BTR_POOL_DIR_DEFAULT.len() as i32 + 10, &mut btr_pool_dir);
    if btr_pool_dir.trim().is_empty() { // Se l'input è vuoto, usa il default
        btr_pool_dir = BTR_POOL_DIR_DEFAULT.to_string();
    }

    // Campo per SNAPSHOTS_DIR
    mvaddstr(7, 2, &format!("SNAPSHOTS_DIR (default: {}): ", SNAPSHOTS_DIR_DEFAULT));
    let mut snapshots_dir = String::new();
    mvwgetstr(stdscr, 7, 18 + SNAPSHOTS_DIR_DEFAULT.len() as i32 + 11, &mut snapshots_dir);
    if snapshots_dir.trim().is_empty() { // Se l'input è vuoto, usa il default
        snapshots_dir = SNAPSHOTS_DIR_DEFAULT.to_string();
    }

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Nasconde il cursore
    (btr_pool_dir, snapshots_dir) // Restituisce i valori
}

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);

    //loop {
    //    let key = getch();
    //    mvprintw(0, 0, &format!("Codice tasto: {}", key));
    //    refresh();
    //}

    start_color();
    init_pair(1, COLOR_BLACK, COLOR_CYAN);
    init_pair(2, COLOR_RED, COLOR_BLACK);
    //init_pair(3, COLOR_WHITE, COLOR_BLACK);

    let (btr_pool_dir, snapshots_dir) = edit_config_fields(stdscr());

    loop {
        let folders = get_snapshot_folders(&snapshots_dir);
        let root_snapshots: Vec<_> = folders.iter().filter(|f| f.starts_with("@.")).cloned().collect();
        let home_snapshots: Vec<_> = folders.iter().filter(|f| f.starts_with("@home.")).cloned().collect();

        if root_snapshots.is_empty() && home_snapshots.is_empty() {
            display_message("Nessun snapshot trovato.", true);
            break;
        }

        let col1_x = 2;
        let max_len_root = root_snapshots.iter().map(|s| s.len()).max().unwrap_or(0).max(4);
        let col2_x = col1_x + max_len_root as i32 + 10;

        let last_row_root = 4 + root_snapshots.len() as i32 - 1;
        let last_row_home = 4 + home_snapshots.len() as i32 - 1;
        let last_row = last_row_root.max(last_row_home);
        let y_confirm = last_row + 3;

        let mut global_row = if !root_snapshots.is_empty() {
            root_snapshots.len() as i32 - 1
        } else {
            0
        };
        let mut current_col = if !root_snapshots.is_empty() { 0 } else { 1 };

        loop {
            clear();
            mvaddstr(0, 0, "Usa le frecce, INVIO per selezionare, q per uscire.");
            attron(A_DIM());
            mvaddstr(1, 0, &format!("BTR_POOL_DIR: {}    SNAPSHOTS_DIR: {}", btr_pool_dir, snapshots_dir));
            attroff(A_DIM());
            attron(COLOR_PAIR(2) | A_BOLD());
            mvwaddstr(stdscr(), 3, col1_x, "root");
            attroff(COLOR_PAIR(2) | A_BOLD());
            for (idx, snap) in root_snapshots.iter().enumerate() {
                let y = 4 + idx as i32;
                if current_col == 0 && idx as i32 == global_row {
                    attron(COLOR_PAIR(1));
                    mvwaddstr(stdscr(), y, col1_x, snap);
                    attroff(COLOR_PAIR(1));
                } else {
                    mvwaddstr(stdscr(), y, col1_x, snap);
                }
            }

            attron(COLOR_PAIR(2) | A_BOLD());
            mvwaddstr(stdscr(), 3, col2_x, "home");
            attroff(COLOR_PAIR(2) | A_BOLD());
            for (idx, snap) in home_snapshots.iter().enumerate() {
                let y = 4 + idx as i32;
                if current_col == 1 && idx as i32 == global_row {
                    attron(COLOR_PAIR(1));
                    mvwaddstr(stdscr(), y, col2_x, snap);
                    attroff(COLOR_PAIR(1));
                } else {
                    mvwaddstr(stdscr(), y, col2_x, snap);
                }
            }

            refresh();

            let key = getch();
            if key == 'q' as i32 {
                endwin();
                return;
            } else if key == KEY_LEFT { // Freccia sinistra
                if current_col == 1 && !root_snapshots.is_empty() { // Passa da "home" a "root"
                    current_col = 0;
                    global_row = global_row.min(root_snapshots.len() as i32 - 1); // Limita la riga
                }
            } else if key == KEY_RIGHT { // Freccia destra
                if current_col == 0 && !home_snapshots.is_empty() { // Passa da "root" a "home"
                    current_col = 1;
                    global_row = global_row.min(home_snapshots.len() as i32 - 1); // Limita la riga
                }
            } else if key == KEY_UP { // Freccia su
                if global_row > 0 { // Non andare sotto la prima riga
                    global_row -= 1;
                }
            } else if key == KEY_DOWN { // Freccia giù
                if current_col == 0 && global_row < root_snapshots.len() as i32 - 1 { // Colonna "root"
                    global_row += 1;
                } else if current_col == 1 && global_row < home_snapshots.len() as i32 - 1 { // Colonna "home"
                    global_row += 1;
                }
            } else if key == KEY_ENTER || key == 10 || key == 13 { // Selezione con Invio
                if !confirm_action(stdscr(), "Confermi di procedere?", y_confirm) {
                    break;
                }

                let selected_snapshot = if current_col == 0 && !root_snapshots.is_empty() {
                    root_snapshots[global_row as usize].clone()
                } else if current_col == 1 && !home_snapshots.is_empty() {
                    home_snapshots[global_row as usize].clone()
                } else {
                    continue;
                };

                let snapshot_category = if current_col == 0 { "root" } else { "home" };
                let snapshot_header = if current_col == 0 { "@" } else { "@home" };

                let source_path = Path::new(&snapshots_dir).join(&selected_snapshot);
                let snapshot_name = Path::new(&btr_pool_dir).join(snapshot_header);

                let target_broken = if snapshot_category == "root" {
                    Path::new(&btr_pool_dir).join("@.BROKEN")
                } else {
                    Path::new(&btr_pool_dir).join("@home.BROKEN")
                };
                let msg = if snapshot_category == "root" {
                    format!("Rinomino {}/@ in {}/@.BROKEN...", btr_pool_dir, btr_pool_dir)
                } else {
                    format!("Rinomino {}/@home in {}/@home.BROKEN...", btr_pool_dir, btr_pool_dir)
                };
                mvwaddstr(stdscr(), y_confirm - 1, 0, &msg);
                refresh();
                if snapshot_category == "root" {
                    run_command(&["mv", "--verbose", &format!("{}/@", btr_pool_dir), &target_broken.to_string_lossy()]);
                } else {
                    run_command(&["mv", "--verbose", &format!("{}/@home", btr_pool_dir), &target_broken.to_string_lossy()]);
                }

                mvwaddstr(stdscr(), y_confirm - 1, 0, &format!("Ripristino dello snapshot '{}'...", selected_snapshot));
                refresh();
                if !run_command(&["btrfs", "subvolume", "snapshot", &source_path.to_string_lossy(), &snapshot_name.to_string_lossy()]) {
                    mvwaddstr(stdscr(), y_confirm, 0, "Errore nel ripristino dello snapshot.");
                    refresh();
                    getch();
                    break;
                }

                if confirm_action(stdscr(), &format!("Vuoi eliminare lo snapshot {}.BROKEN?", snapshot_header), y_confirm) {
                    run_command(&["btrfs", "subvolume", "delete", &target_broken.to_string_lossy()]);
                    mvwaddstr(stdscr(), y_confirm - 1, 0, &format!("Snapshot {}.BROKEN eliminato.", snapshot_header));
                    refresh();
                    getch();
                } else {
                    mvwaddstr(stdscr(), y_confirm - 1, 0, "Eliminazione dello snapshot .BROKEN annullata.");
                    refresh();
                    getch();
                }

                if confirm_action(stdscr(), "Vuoi riavviare il sistema?", y_confirm) {
                    mvwaddstr(stdscr(), y_confirm - 1, 0, "Riavvio in corso...");
                    refresh();
                    run_command(&["reboot"]);
                    endwin();
                    return;
                } else {
                    mvwaddstr(stdscr(), y_confirm - 1, 0, "Riavvio annullato. Operazione completata.");
                    refresh();
                    getch();
                    break;
                }
            }
        }
    }
    endwin();
}