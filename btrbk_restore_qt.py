#!/usr/bin/env python3
import os
import subprocess
import sys
import logging
from pathlib import Path
from PyQt6.QtWidgets import (QApplication, QWidget, QLabel, QComboBox, QPushButton, 
                             QVBoxLayout, QHBoxLayout, QMessageBox, QTextEdit, 
                             QLineEdit, QFileDialog, QProgressBar)
from PyQt6.QtCore import QThread, pyqtSignal

# Set up logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

class CommandWorker(QThread):
    finished = pyqtSignal(str, str)

    def __init__(self, command):
        super().__init__()
        self.command = command

    def run(self):
        try:
            result = subprocess.run(self.command, capture_output=True, text=True, check=True)
            self.finished.emit(result.stdout, result.stderr)
        except subprocess.CalledProcessError as e:
            self.finished.emit("", f"Command failed: {e}")

class SnapshotRestoreApp(QWidget):
    def __init__(self):
        super().__init__()
        self.btr_pool_dir = Path("/mnt/btr_pool")
        self.snapshots_dir = Path("/mnt/btr_pool/btrbk_snapshots")
        self.initUI()

    def initUI(self):
        self.setWindowTitle('btrbk Snapshot Restore')
        self.resize(900, 500)

        layout = QVBoxLayout()

        # Path selection layout
        pathLayout = QHBoxLayout()
        self.setupPathSelection(pathLayout)
        layout.addLayout(pathLayout)

        # Snapshot selection
        self.label = QLabel('Select a snapshot to restore:')
        layout.addWidget(self.label)

        self.comboBox = QComboBox()
        self.populateComboBox()
        layout.addWidget(self.comboBox)

        # Output display
        self.outputTextEdit = QTextEdit()
        self.outputTextEdit.setReadOnly(True)
        layout.addWidget(self.outputTextEdit)

        # Progress bar
        self.progressBar = QProgressBar()
        self.progressBar.setRange(0, 0)  # Indeterminate progress
        self.progressBar.hide()
        layout.addWidget(self.progressBar)

        # Button layout
        buttonLayout = QHBoxLayout()
        self.setupButtons(buttonLayout)
        layout.addLayout(buttonLayout)

        self.setLayout(layout)

    def setupPathSelection(self, layout):
        self.btr_pool_dir_label = QLabel('BTR Pool Directory:')
        layout.addWidget(self.btr_pool_dir_label)

        self.btr_pool_dir_edit = QLineEdit(str(self.btr_pool_dir))
        layout.addWidget(self.btr_pool_dir_edit)

        self.btr_pool_dir_button = QPushButton('Browse')
        self.btr_pool_dir_button.clicked.connect(self.browseBtrPoolDir)
        layout.addWidget(self.btr_pool_dir_button)

        self.snapshots_dir_label = QLabel('Snapshots Directory:')
        layout.addWidget(self.snapshots_dir_label)

        self.snapshots_dir_edit = QLineEdit(str(self.snapshots_dir))
        layout.addWidget(self.snapshots_dir_edit)

        self.snapshots_dir_button = QPushButton('Browse')
        self.snapshots_dir_button.clicked.connect(self.browseSnapshotsDir)
        layout.addWidget(self.snapshots_dir_button)

    def setupButtons(self, layout):
        self.restoreButton = QPushButton('Restore')
        self.restoreButton.setStyleSheet("background-color: #ff7979; color: black;")
        self.restoreButton.setFixedSize(100, 30)
        self.restoreButton.clicked.connect(self.restoreSnapshot)
        layout.addWidget(self.restoreButton)

        self.exitButton = QPushButton('Exit')
        self.exitButton.setFixedSize(100, 30)
        self.exitButton.clicked.connect(self.close)
        layout.addWidget(self.exitButton)

    def browseBtrPoolDir(self):
        directory = QFileDialog.getExistingDirectory(self, "Select BTR Pool Directory")
        if directory:
            self.btr_pool_dir_edit.setText(directory)
            self.populateComboBox()

    def browseSnapshotsDir(self):
        directory = QFileDialog.getExistingDirectory(self, "Select Snapshots Directory")
        if directory:
            self.snapshots_dir_edit.setText(directory)
            self.populateComboBox()

    def populateComboBox(self):
        self.snapshots_dir = Path(self.snapshots_dir_edit.text())
        try:
            folders = sorted([f for f in self.snapshots_dir.iterdir() if f.is_dir()], reverse=True)
            self.comboBox.clear()
            for folder in folders:
                self.comboBox.addItem(folder.name)
            if folders:
                self.comboBox.setCurrentIndex(0)
        except Exception as e:
            logging.error(f"Error populating combo box: {e}")
            QMessageBox.critical(self, 'Error', f'Failed to read snapshots directory: {e}')

    def restoreSnapshot(self):
        self.btr_pool_dir = Path(self.btr_pool_dir_edit.text())
        self.snapshots_dir = Path(self.snapshots_dir_edit.text())

        if not self.btr_pool_dir.is_dir() or not self.snapshots_dir.is_dir():
            QMessageBox.critical(self, 'Error', 'Invalid directory path')
            return

        choice = self.comboBox.currentIndex()
        folders = sorted((f for f in self.snapshots_dir.iterdir() if f.is_dir()), reverse=True)

        if 0 <= choice < len(folders):
            selected_folder = folders[choice]
            source_path = self.snapshots_dir / selected_folder.name

            try:
                if selected_folder.name.startswith("@."):
                    snapshot_type = "@"
                    snapshot_name = self.btr_pool_dir / "@"
                elif selected_folder.name.startswith("@home."):
                    snapshot_type = "@home"
                    snapshot_name = self.btr_pool_dir / "@home"
                else:
                    QMessageBox.critical(self, 'Error', 'Invalid snapshot')
                    return

                self.runCommandAsync(["mv", "--verbose", str(snapshot_name), f"{snapshot_name}.BROKEN"])
                self.runCommandAsync(["btrfs", "subvolume", "snapshot", str(source_path), str(snapshot_name)])

                if QMessageBox.question(self, 'Confirm', f'Do you want to delete the {snapshot_type}.BROKEN snapshot?',
                                        QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No) == QMessageBox.StandardButton.Yes:
                    self.runCommandAsync(["btrfs", "subvolume", "delete", f"{self.btr_pool_dir}/{snapshot_type}.BROKEN"])

                if QMessageBox.question(self, 'Confirm', 'Do you want to reboot the system?',
                                        QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No) == QMessageBox.StandardButton.Yes:
                    self.runCommandAsync(["reboot"])

            except Exception as e:
                logging.error(f"An error occurred: {e}")
                QMessageBox.critical(self, 'Error', f'An error occurred: {e}')
        else:
            QMessageBox.critical(self, 'Error', 'Invalid choice')

    def runCommandAsync(self, command):
        self.progressBar.show()
        self.worker = CommandWorker(command)
        self.worker.finished.connect(self.onCommandFinished)
        self.worker.start()

    def onCommandFinished(self, stdout, stderr):
        self.progressBar.hide()
        self.outputTextEdit.append(f"Command: {' '.join(self.worker.command)}")
        self.outputTextEdit.append(f"Output:\n{stdout}")
        if stderr:
            self.outputTextEdit.append(f"Error:\n{stderr}")
        self.outputTextEdit.append("\n")

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = SnapshotRestoreApp()
    ex.show()
    sys.exit(app.exec())
