#!/usr/bin/python
import os
import subprocess
import sys
from PyQt6.QtWidgets import QApplication, QWidget, QLabel, QComboBox, QPushButton, QVBoxLayout, QHBoxLayout, QMessageBox, QTextEdit, QLineEdit, QFileDialog

class SnapshotRestoreApp(QWidget):
    def __init__(self):
        super().__init__()

        self.btr_pool_dir = "/mnt/btr_pool"
        self.snapshots_dir = "/mnt/btr_pool/btrbk_snapshots"

        self.initUI()

    def initUI(self):
        self.setWindowTitle('btrbk Snapshot Restore')

        # Imposta le dimensioni iniziali della finestra
        self.resize(900, 500)

        layout = QVBoxLayout()

        # Layout orizzontale per i percorsi e i pulsanti di browse
        pathLayout = QHBoxLayout()

        self.btr_pool_dir_label = QLabel('BTR Pool Directory:')
        pathLayout.addWidget(self.btr_pool_dir_label)

        self.btr_pool_dir_edit = QLineEdit(self.btr_pool_dir)
        pathLayout.addWidget(self.btr_pool_dir_edit)

        self.btr_pool_dir_button = QPushButton('Browse')
        self.btr_pool_dir_button.clicked.connect(self.browseBtrPoolDir)
        pathLayout.addWidget(self.btr_pool_dir_button)

        self.snapshots_dir_label = QLabel('Snapshots Directory:')
        pathLayout.addWidget(self.snapshots_dir_label)

        self.snapshots_dir_edit = QLineEdit(self.snapshots_dir)
        pathLayout.addWidget(self.snapshots_dir_edit)

        self.snapshots_dir_button = QPushButton('Browse')
        self.snapshots_dir_button.clicked.connect(self.browseSnapshotsDir)
        pathLayout.addWidget(self.snapshots_dir_button)

        layout.addLayout(pathLayout)

        self.label = QLabel('Seleziona uno snapshot da ripristinare:')
        layout.addWidget(self.label)

        self.comboBox = QComboBox()
        self.populateComboBox()
        layout.addWidget(self.comboBox)

        self.outputTextEdit = QTextEdit()
        self.outputTextEdit.setReadOnly(True)
        layout.addWidget(self.outputTextEdit)

        # Layout orizzontale per i pulsanti
        buttonLayout = QHBoxLayout()

        self.restoreButton = QPushButton('Ripristina')
        self.restoreButton.setStyleSheet("background-color: #ff7979; color: black;")  # Rosso chiaro pastello
        self.restoreButton.setFixedSize(100, 30)  # Dimensioni standard
        self.restoreButton.clicked.connect(self.restoreSnapshot)
        buttonLayout.addWidget(self.restoreButton)

        self.exitButton = QPushButton('Esci')
        self.exitButton.setFixedSize(100, 30)  # Dimensioni standard
        self.exitButton.clicked.connect(self.close)
        buttonLayout.addWidget(self.exitButton)

        layout.addLayout(buttonLayout)

        self.setLayout(layout)

    def browseBtrPoolDir(self):
        directory = QFileDialog.getExistingDirectory(self, "Seleziona la directory BTR Pool")
        if directory:
            self.btr_pool_dir_edit.setText(directory)
            self.populateComboBox()

    def browseSnapshotsDir(self):
        directory = QFileDialog.getExistingDirectory(self, "Seleziona la directory Snapshots")
        if directory:
            self.snapshots_dir_edit.setText(directory)
            self.populateComboBox()

    def populateComboBox(self):
        self.snapshots_dir = self.snapshots_dir_edit.text()
        folders = [f for f in os.listdir(self.snapshots_dir) if os.path.isdir(os.path.join(self.snapshots_dir, f))]
        folders.reverse()  # Inverti l'ordine degli elementi
        self.comboBox.clear()
        for folder in folders:
            self.comboBox.addItem(folder)
        if folders:
            self.comboBox.setCurrentIndex(0)  # Seleziona il primo elemento come default

    def restoreSnapshot(self):
        self.btr_pool_dir = self.btr_pool_dir_edit.text()
        self.snapshots_dir = self.snapshots_dir_edit.text()

        choice = self.comboBox.currentIndex()

        folders = [f for f in os.listdir(self.snapshots_dir) if os.path.isdir(os.path.join(self.snapshots_dir, f))]
        folders.reverse()  # Inverti l'ordine degli elementi

        if 0 <= choice < len(folders):
            selected_folder = folders[choice]
            source_path = os.path.join(self.snapshots_dir, selected_folder)

            if selected_folder.startswith("@."):
                snapshot_type = "@"
                snapshot_name = f"{self.btr_pool_dir}/@"
                self.runCommand(["mv", "--verbose", f"{self.btr_pool_dir}/@", f"{self.btr_pool_dir}/@.BROKEN"])
            elif selected_folder.startswith("@home."):
                snapshot_type = "@home"
                snapshot_name = f"{self.btr_pool_dir}/@home"
                self.runCommand(["mv", "--verbose", f"{self.btr_pool_dir}/@home", f"{self.btr_pool_dir}/@home.BROKEN"])
            else:
                QMessageBox.critical(self, 'Errore', 'Snapshot non valido')
                return

            self.runCommand(["btrfs", "subvolume", "snapshot", source_path, snapshot_name])

            doRemoveBROKEN = QMessageBox.question(self, 'Conferma', f'Vuoi eliminare lo snapshot {snapshot_type}.BROKEN?', QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No)
            if doRemoveBROKEN == QMessageBox.StandardButton.Yes:
                self.runCommand(["btrfs", "subvolume", "delete", f"{self.btr_pool_dir}/{snapshot_type}.BROKEN"])

            doReboot = QMessageBox.question(self, 'Conferma', 'Vuoi riavviare il sistema?', QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No)
            if doReboot == QMessageBox.StandardButton.Yes:
                self.runCommand(["reboot"])
        else:
            QMessageBox.critical(self, 'Errore', 'Scelta non valida')

    def runCommand(self, command):
        result = subprocess.run(command, capture_output=True, text=True)
        self.outputTextEdit.append(f"Comando: {' '.join(command)}")
        self.outputTextEdit.append(f"Output:\n{result.stdout}")
        self.outputTextEdit.append(f"Errore:\n{result.stderr}")
        self.outputTextEdit.append("\n")

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = SnapshotRestoreApp()
    ex.show()
    sys.exit(app.exec())
