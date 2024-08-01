import os
import subprocess
import sys
from PyQt6.QtWidgets import QApplication, QWidget, QLabel, QComboBox, QPushButton, QVBoxLayout, QMessageBox

class SnapshotRestoreApp(QWidget):
    def __init__(self):
        super().__init__()

        self.btr_pool_dir = "/mnt/btr_pool"
        self.snapshots_dir = "/mnt/btr_pool/btrbk_snapshots"

        self.initUI()

    def initUI(self):
        self.setWindowTitle('Snapshot Restore')

        layout = QVBoxLayout()

        self.label = QLabel('Seleziona uno snapshot da ripristinare:')
        layout.addWidget(self.label)

        self.comboBox = QComboBox()
        self.populateComboBox()
        layout.addWidget(self.comboBox)

        self.restoreButton = QPushButton('Ripristina')
        self.restoreButton.clicked.connect(self.restoreSnapshot)
        layout.addWidget(self.restoreButton)

        self.setLayout(layout)

    def populateComboBox(self):
        folders = [f for f in os.listdir(self.snapshots_dir) if os.path.isdir(os.path.join(self.snapshots_dir, f))]
        self.comboBox.addItem('Esci')
        for folder in folders:
            self.comboBox.addItem(folder)

    def restoreSnapshot(self):
        choice = self.comboBox.currentIndex() - 1

        if choice == -1:
            self.close()
            return

        folders = [f for f in os.listdir(self.snapshots_dir) if os.path.isdir(os.path.join(self.snapshots_dir, f))]

        if 0 <= choice < len(folders):
            selected_folder = folders[choice]
            source_path = os.path.join(self.snapshots_dir, selected_folder)

            if selected_folder.startswith("@."):
                snapshot_type = "@"
                snapshot_name = f"{self.btr_pool_dir}/@"
                subprocess.run(["mv", "--verbose", f"{self.btr_pool_dir}/@", f"{self.btr_pool_dir}/@.BROKEN"])
            elif selected_folder.startswith("@home."):
                snapshot_type = "@home"
                snapshot_name = f"{self.btr_pool_dir}/@home"
                subprocess.run(["mv", "--verbose", f"{self.btr_pool_dir}/@home", f"{self.btr_pool_dir}/@home.BROKEN"])
            else:
                QMessageBox.critical(self, 'Errore', 'Snapshot non valido')
                return

            subprocess.run(["btrfs", "subvolume", "snapshot", source_path, snapshot_name])

            doRemoveBROKEN = QMessageBox.question(self, 'Conferma', f'Vuoi eliminare lo snapshot {snapshot_type}.BROKEN?', QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No)
            if doRemoveBROKEN == QMessageBox.StandardButton.Yes:
                subprocess.run(["btrfs", "subvolume", "delete", f"{self.btr_pool_dir}/{snapshot_type}.BROKEN"])

            doReboot = QMessageBox.question(self, 'Conferma', 'Vuoi riavviare il sistema?', QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No)
            if doReboot == QMessageBox.StandardButton.Yes:
                subprocess.run(["reboot"])
        else:
            QMessageBox.critical(self, 'Errore', 'Scelta non valida')

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = SnapshotRestoreApp()
    ex.show()
    sys.exit(app.exec())
