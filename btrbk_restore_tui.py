#!/usr/bin/env python3
import curses
import curses.textpad
import os
import subprocess
import sys

# Configurazione iniziale delle cartelle (parametri di default)
BTR_POOL_DIR_DEFAULT = "/mnt/btr_pool"
SNAPSHOTS_DIR_DEFAULT = "/mnt/btr_pool/btrbk_snapshots"

def get_snapshot_folders(snapshots_dir):
    try:
        return [f for f in os.listdir(snapshots_dir) if os.path.isdir(os.path.join(snapshots_dir, f))]
    except Exception:
        return []

def run_command(cmd_list):
    try:
        subprocess.run(cmd_list, check=True)
        return True
    except subprocess.CalledProcessError:
        return False

def clear_line(stdscr, y):
    stdscr.move(y, 0)
    stdscr.clrtoeol()

def confirm_action(stdscr, prompt, y_confirm):
    """
    Visualizza una richiesta di conferma in una riga specifica (y_confirm).
    Il prompt viene visualizzato e l'utente deve premere 's' o 'n'.
    Dopo la scelta, l'area del prompt viene pulita.
    """
    prompt_text = prompt + " (s/n): "
    stdscr.addstr(y_confirm, 0, prompt_text)
    stdscr.refresh()
    while True:
        c = stdscr.getch()
        if c in (ord('s'), ord('S')):
            clear_line(stdscr, y_confirm)
            stdscr.refresh()
            return True
        elif c in (ord('n'), ord('N')):
            clear_line(stdscr, y_confirm)
            stdscr.refresh()
            return False

def display_message(stdscr, message, pause=True):
    # Per messaggi "a schermo intero" usiamo clear, per semplicità
    stdscr.clear()
    stdscr.addstr(0, 0, message)
    if pause:
        stdscr.addstr(2, 0, "Premi un tasto per continuare...")
        stdscr.refresh()
        stdscr.getch()
    else:
        stdscr.refresh()

def edit_config_fields(stdscr, btr_pool_dir_default, snapshots_dir_default):
    """
    Mostra le istruzioni e due campi editabili per modificare le variabili di configurazione.
    Restituisce i valori inseriti dall'utente.
    """
    curses.curs_set(1)  # mostra il cursore per l'editing
    stdscr.clear()
    stdscr.addstr(0, 0, "Istruzioni:")
    stdscr.addstr(1, 0, "Usa le frecce per muoverti, BACKSPACE per cancellare e INVIO per confermare ciascun campo.")
    stdscr.addstr(3, 0, "Modifica se necessario i campi sottostanti e conferma con INVIO:")

    # Campo per BTR_POOL_DIR
    stdscr.addstr(5, 2, "BTR_POOL_DIR: ")
    editwin1 = curses.newwin(1, 50, 5, 18)
    box1 = curses.textpad.Textbox(editwin1)
    editwin1.addstr(0, 0, btr_pool_dir_default)
    stdscr.refresh()
    btr_pool_dir = box1.edit().strip()

    # Campo per SNAPSHOTS_DIR
    stdscr.addstr(7, 2, "SNAPSHOTS_DIR: ")
    editwin2 = curses.newwin(1, 50, 7, 18)
    box2 = curses.textpad.Textbox(editwin2)
    editwin2.addstr(0, 0, snapshots_dir_default)
    stdscr.refresh()
    snapshots_dir = box2.edit().strip()

    curses.curs_set(0)  # nasconde il cursore al termine dell'editing
    return btr_pool_dir, snapshots_dir

def main(stdscr):
    curses.curs_set(0)
    curses.start_color()
    curses.init_pair(1, curses.COLOR_BLACK, curses.COLOR_CYAN)  # Evidenziazione
    curses.init_pair(2, curses.COLOR_RED, curses.COLOR_BLACK)   # Header

    # Schermata di editing della configurazione
    btr_pool_dir, snapshots_dir = edit_config_fields(stdscr, BTR_POOL_DIR_DEFAULT, SNAPSHOTS_DIR_DEFAULT)

    while True:
        # Aggiorna la lista degli snapshot
        folders = get_snapshot_folders(snapshots_dir)
        root_snapshots = sorted([f for f in folders if f.startswith("@.")])
        home_snapshots = sorted([f for f in folders if f.startswith("@home.")])

        if not root_snapshots and not home_snapshots:
            display_message(stdscr, "Nessun snapshot trovato.", pause=True)
            return

        # Calcola la posizione delle colonne
        col1_x = 2
        max_len_root = max([len("root")] + [len(snap) for snap in root_snapshots]) if root_snapshots else len("root")
        col2_x = col1_x + max_len_root + 10  # distanza di 10 spazi dalla fine della colonna root

        # Determina l'ultima riga usata dalle colonne
        last_row_root = 4 + len(root_snapshots) - 1 if root_snapshots else 3
        last_row_home = 4 + len(home_snapshots) - 1 if home_snapshots else 3
        last_row = max(last_row_root, last_row_home)
        # La riga in cui visualizzare la conferma: 3 spazi dopo l'ultima riga occupata
        y_confirm = last_row + 3

        # Imposta la selezione iniziale
        if root_snapshots:
            global_row = len(root_snapshots) - 1
            current_col = 0  # parte dalla colonna root
        else:
            global_row = 0
            current_col = 1  # se non ci sono snapshot in root, passa a home

        # Menu di selezione snapshot
        while True:
            stdscr.clear()
            stdscr.addstr(0, 0,
                "Usa le frecce (su/giù: scorri, sinistra/destra: cambia colonna), INVIO per selezionare, q per uscire.",
                curses.A_BOLD)
            stdscr.addstr(1, 0, f"BTR_POOL_DIR: {btr_pool_dir}    SNAPSHOTS_DIR: {snapshots_dir}", curses.A_DIM)

            # Disegna la colonna "root"
            stdscr.attron(curses.color_pair(2) | curses.A_BOLD)
            stdscr.addstr(3, col1_x, "root")
            stdscr.attroff(curses.color_pair(2) | curses.A_BOLD)
            for idx, snap in enumerate(root_snapshots):
                y = 4 + idx
                if current_col == 0 and idx == global_row:
                    stdscr.attron(curses.color_pair(1))
                    stdscr.addstr(y, col1_x, snap)
                    stdscr.attroff(curses.color_pair(1))
                else:
                    stdscr.addstr(y, col1_x, snap)

            # Disegna la colonna "home"
            stdscr.attron(curses.color_pair(2) | curses.A_BOLD)
            stdscr.addstr(3, col2_x, "home")
            stdscr.attroff(curses.color_pair(2) | curses.A_BOLD)
            for idx, snap in enumerate(home_snapshots):
                y = 4 + idx
                if current_col == 1 and idx == global_row:
                    stdscr.attron(curses.color_pair(1))
                    stdscr.addstr(y, col2_x, snap)
                    stdscr.attroff(curses.color_pair(1))
                else:
                    stdscr.addstr(y, col2_x, snap)

            stdscr.refresh()

            key = stdscr.getch()
            if key == ord('q'):
                return
            elif key == curses.KEY_LEFT:
                if current_col == 1 and root_snapshots:
                    current_col = 0
                    if global_row >= len(root_snapshots):
                        global_row = len(root_snapshots) - 1
            elif key == curses.KEY_RIGHT:
                if current_col == 0 and home_snapshots:
                    current_col = 1
                    if global_row >= len(home_snapshots):
                        global_row = len(home_snapshots) - 1
            elif key == curses.KEY_UP:
                if global_row > 0:
                    global_row -= 1
            elif key == curses.KEY_DOWN:
                if current_col == 0 and global_row < len(root_snapshots) - 1:
                    global_row += 1
                elif current_col == 1 and global_row < len(home_snapshots) - 1:
                    global_row += 1
            elif key in [curses.KEY_ENTER, 10, 13]:
                # Usa y_confirm per posizionare il prompt in una riga 3 spazi dopo le colonne
                if not confirm_action(stdscr, "Confermi di procedere?", y_confirm):
                    break  # Torna al menu principale

                # Selezione effettuata nella colonna corrente
                if current_col == 0 and root_snapshots:
                    selected_snapshot = root_snapshots[global_row]
                    snapshot_category = "root"
                    snapshot_header = "@"
                elif current_col == 1 and home_snapshots:
                    selected_snapshot = home_snapshots[global_row]
                    snapshot_category = "home"
                    snapshot_header = "@home"
                else:
                    continue

                source_path = os.path.join(snapshots_dir, selected_snapshot)
                snapshot_name = os.path.join(btr_pool_dir, snapshot_header)

                # Rinomina il subvolume attuale in .BROKEN
                if snapshot_category == "root":
                    msg = f"Rinomino {btr_pool_dir}/@ in {btr_pool_dir}/@.BROKEN..."
                    target_broken = os.path.join(btr_pool_dir, "@.BROKEN")
                else:
                    msg = f"Rinomino {btr_pool_dir}/@home in {btr_pool_dir}/@home.BROKEN..."
                    target_broken = os.path.join(btr_pool_dir, "@home.BROKEN")
                stdscr.addstr(y_confirm - 1, 0, msg)
                stdscr.refresh()
                if snapshot_category == "root":
                    run_command(["mv", "--verbose", os.path.join(btr_pool_dir, "@"), target_broken])
                else:
                    run_command(["mv", "--verbose", os.path.join(btr_pool_dir, "@home"), target_broken])

                # Ripristino dello snapshot selezionato
                stdscr.addstr(y_confirm - 1, 0, f"Ripristino dello snapshot '{selected_snapshot}'...", curses.A_BOLD)
                stdscr.refresh()
                if not run_command(["btrfs", "subvolume", "snapshot", source_path, snapshot_name]):
                    stdscr.addstr(y_confirm, 0, "Errore nel ripristino dello snapshot.")
                    stdscr.refresh()
                    stdscr.getch()
                    break

                # Conferma per eliminare lo snapshot .BROKEN
                if confirm_action(stdscr, f"Vuoi eliminare lo snapshot {snapshot_header}.BROKEN?", y_confirm):
                    run_command(["btrfs", "subvolume", "delete", target_broken])
                    stdscr.addstr(y_confirm - 1, 0, f"Snapshot {snapshot_header}.BROKEN eliminato.")
                    stdscr.refresh()
                    stdscr.getch()
                else:
                    stdscr.addstr(y_confirm - 1, 0, "Eliminazione dello snapshot .BROKEN annullata.")
                    stdscr.refresh()
                    stdscr.getch()

                # Conferma per il riavvio del sistema
                if confirm_action(stdscr, "Vuoi riavviare il sistema?", y_confirm):
                    stdscr.addstr(y_confirm - 1, 0, "Riavvio in corso...")
                    stdscr.refresh()
                    run_command(["reboot"])
                    return  # Termina il programma se il sistema si riavvia
                else:
                    stdscr.addstr(y_confirm - 1, 0, "Riavvio annullato. Operazione completata.")
                    stdscr.refresh()
                    stdscr.getch()
                    break  # Torna al menu principale

if __name__ == "__main__":
    try:
        curses.wrapper(main)
    except Exception as err:
        sys.exit(f"Errore: {err}")
