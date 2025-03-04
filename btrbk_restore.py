#!/usr/bin/python
import os
import subprocess

# Configura le cartelle
btr_pool_dir = "/mnt/btr_pool"
snapshots_dir = "/mnt/btr_pool/btrbk_snapshots"

# Lista le cartelle presenti in /mnt/btr_pool/btrbk_snapshots/
folders = [f for f in os.listdir(snapshots_dir) if os.path.isdir(os.path.join(snapshots_dir, f))]
snapshot_type = ""

# Stampa le cartelle numerate
print("Lista degli snapshot disponibili:\n")
print("0. Esci")
for i, folder in enumerate(folders):
    print(f"{i+1}. {folder}")

# Chiedi all'utente di scegliere una cartella
choice = int(input("\nScegli lo snapshot da ripristinare: ")) - 1

# Se 0, esci
if choice == -1:
    exit()

# Verifica che la scelta sia valida
if 0 <= choice < len(folders):
    # Esegui btrfs subvolume snapshot
    selected_folder = folders[choice]
    source_path = os.path.join(snapshots_dir, selected_folder)
    
    if selected_folder.startswith("@."):
        snapshot_type = "@"
        snapshot_name = f"{btr_pool_dir}/@"
        subprocess.run(["mv", "--verbose", f"{btr_pool_dir}/@", f"{btr_pool_dir}/@.BROKEN"])
    elif selected_folder.startswith("@home."):
        snapshot_type = "@home"
        snapshot_name = f"{btr_pool_dir}/@home"
        subprocess.run(["mv", "--verbose", f"{btr_pool_dir}/@home", f"{btr_pool_dir}/@home.BROKEN"])
    else:
        print("Snapshot non valido")
        exit()
    
    #print(f"btrfs subvolume snapshot {source_path} {snapshot_name}")
    subprocess.run(["btrfs", "subvolume", "snapshot", source_path, snapshot_name])

    # Chiedi all'utente se eseguire rimozione snapshot.BROKEN
    doRemoveBROKEN = input(f"\nVuoi eliminare lo snapshot {snapshot_type}.BROKEN? (s/n): ")
    if doRemoveBROKEN.lower() == 's':
        subprocess.run(["btrfs", "subvolume", "delete", f"{btr_pool_dir}/{snapshot_type}.BROKEN"])

    # Chiedi se vuoi riavviare
    doReboot = input("\nVuoi riavviare il sistema? (s/n): ")
    if doReboot.lower() =='s':
        subprocess.run(["reboot"])
else:
    print("Scelta non valida.")

