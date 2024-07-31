use std::fs;
use std::io::{self, Write};
use std::process::{Command, exit};
use std::path::Path;
use ansi_term::Colour::{Red, Green, Yellow, Blue, Purple, Cyan, White};
use ansi_term::Style;

fn main() {
    // Configura le cartelle
    let btr_pool_dir = "/mnt/btr_pool";
    let snapshots_dir = "/mnt/btr_pool/btrbk_snapshots";

    // Lista le cartelle presenti in /mnt/btr_pool/btrbk_snapshots/
    let folders: Vec<String> = fs::read_dir(snapshots_dir)
        .expect("Impossibile leggere la directory degli snapshot")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() {
                Some(path.file_name()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    // Stampa le cartelle numerate
    println!("Lista degli snapshot disponibili:\n");
    println!("0. Esci");

    let mut unique_dates = Vec::new();
    for folder in &folders {
        if let Some(date) = folder.split('.').nth(1) {
            if !unique_dates.contains(&date.to_string()) {
                unique_dates.push(date.to_string());
            }
        }
    }

    // Trova la data più recente
    let mut latest_date = String::new();
    for date in &unique_dates {
        if latest_date.is_empty() || date > &latest_date {
            latest_date = date.clone();
        }
    }

    let colors = vec![Red, Green, Yellow, Blue, Purple, Cyan];
    let mut color_map = std::collections::HashMap::new();

    for (i, date) in unique_dates.iter().enumerate() {
        if date == &latest_date {
            color_map.insert(date.clone(), White);
        } else {
            color_map.insert(date.clone(), colors[i % colors.len()]);
        }
    }

    for (i, folder) in folders.iter().enumerate() {
        if let Some(date) = folder.split('.').nth(1) {
            let color = color_map.get(date).unwrap();
            println!("{}. {}", i + 1, Style::new().fg(*color).paint(folder));
        } else {
            println!("{}. {}", i + 1, folder);
        }
    }

    // Chiedi all'utente di scegliere una cartella
    print!("\nScegli lo snapshot da ripristinare: ");
    io::stdout().flush().unwrap();
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Errore nella lettura dell'input");
    let choice: i32 = choice_input.trim().parse().expect("Input non valido");

    // Se 0, esci
    if choice == 0 {
        exit(0);
    }

    // Verifica che la scelta sia valida
    if 1 <= choice && choice <= folders.len() as i32 {
        // Esegui btrfs subvolume snapshot
        let selected_folder = &folders[(choice - 1) as usize];
        let source_path = Path::new(snapshots_dir).join(selected_folder);

        let (snapshot_type, snapshot_name) = if selected_folder.starts_with("@.") {
            ("@", format!("{}/@", btr_pool_dir))
        } else if selected_folder.starts_with("@home.") {
            ("@home", format!("{}/@home", btr_pool_dir))
        } else {
            println!("Snapshot non valido");
            exit(1);
        };

        Command::new("mv")
            .arg("--verbose")
            .arg(format!("{}/{}", btr_pool_dir, snapshot_type))
            .arg(format!("{}/{}.BROKEN", btr_pool_dir, snapshot_type))
            .status()
            .expect("Errore nell'esecuzione del comando mv");

        Command::new("btrfs")
            .arg("subvolume")
            .arg("snapshot")
            .arg(source_path)
            .arg(snapshot_name)
            .status()
            .expect("Errore nell'esecuzione del comando btrfs subvolume snapshot");

        // Chiedi all'utente se eseguire rimozione snapshot.BROKEN
        print!("\nVuoi eliminare lo snapshot {}.BROKEN? (s/n): ", snapshot_type);
        io::stdout().flush().unwrap();
        let mut do_remove_broken_input = String::new();
        io::stdin().read_line(&mut do_remove_broken_input).expect("Errore nella lettura dell'input");
        if do_remove_broken_input.trim().to_lowercase() == "s" {
            Command::new("btrfs")
                .arg("subvolume")
                .arg("delete")
                .arg(format!("{}/{}.BROKEN", btr_pool_dir, snapshot_type))
                .status()
                .expect("Errore nell'esecuzione del comando btrfs subvolume delete");
        }

        // Chiedi se vuoi riavviare
        print!("\nVuoi riavviare il sistema? (s/n): ");
        io::stdout().flush().unwrap();
        let mut do_reboot_input = String::new();
        io::stdin().read_line(&mut do_reboot_input).expect("Errore nella lettura dell'input");
        if do_reboot_input.trim().to_lowercase() == "s" {
            Command::new("reboot").status().expect("Errore nell'esecuzione del comando reboot");
        }
    } else {
        println!("Scelta non valida.");
    }
}
