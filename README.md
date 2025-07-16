# BTRBK Restore Tool

Un set di strumenti per il ripristino di snapshot Btrfs creati con btrbk, disponibile in Python e Rust con diverse interfacce utente.

## Descrizione

Questo progetto fornisce strumenti per ripristinare facilmente snapshot di subvolumi Btrfs (@ per root, @home per home e @games per games) creati dal tool btrbk. Gli strumenti permettono di:

- Visualizzare gli snapshot disponibili per tutti i subvolumi
- Selezionare e ripristinare snapshot specifici
- Gestire automaticamente il backup dei subvolumi esistenti
- Configurazione persistente e condivisa tra le versioni
- Pulizia intelligente degli snapshot vecchi
- Riavvio sistema con indicatori visivi
- Opzionalmente riavviare il sistema dopo il ripristino

## Versioni Disponibili

### Python
- **`btrbk_restore.py`** - Versione CLI semplice con menu testuale
- **`btrbk_restore_tui_pro.py`** - Interfaccia TUI professionale con configurazione persistente e 3 colonne

### Rust
- **`btrbk_restore_rust/`** - Versione TUI performante scritta in Rust con ncurses (identica alla versione Pro Python)

## Prerequisiti

### Per le versioni Python:
```bash
# Versione base CLI
python3

# Versione TUI professionale
python3 (con modulo curses incluso)
```

### Per la versione Rust:
```bash
# Installazione Rust (edition 2021)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build del progetto
cd btrbk_restore_rust
cargo build --release
```

## Configurazione

Il tool assume per default:
- **Pool Btrfs**: `/mnt/btr_pool`
- **Directory snapshot**: `/mnt/btr_pool/btrbk_snapshots`

**Configurazione Condivisa**: Le versioni TUI Pro (Python) e Rust condividono lo stesso file di configurazione JSON in `~/.config/btrbk_restore/config.json`, garantendo un'esperienza utente completamente coerente.

## Utilizzo

### Versione CLI Python
```bash
sudo ./btrbk_restore.py
```

### Versione TUI Professionale Python
```bash
sudo ./btrbk_restore_tui_pro.py
```

### Versione TUI Rust (identica alla Pro Python)
```bash
cd btrbk_restore_rust
sudo ./target/release/btrbk_restore
```

## Funzionalità

### Versione CLI (`btrbk_restore.py`)
- Lista numerata di tutti gli snapshot
- Selezione tramite numero
- Supporto completo per @, @home, @games
- Interfaccia semplice per uso occasionale

### Versione TUI Professionale (`btrbk_restore_tui_pro.py`)
- **Interfaccia a 3 colonne**: ROOT | HOME | GAMES
- **Configurazione persistente**: Salvataggio automatico in `~/.config/btrbk_restore/config.json`
- **Navigazione avanzata**: Frecce direzionali per navigazione fluida
- **Schermata settaggi completa**: Tasto `S` per configurazione avanzata
- **Settaggi configurabili**: Directory, auto-cleanup, conferme, timestamp
- **Messaggi di stato**: Feedback in tempo reale delle operazioni
- **Temi e colori**: Interfaccia professionale con evidenziazione
- **Auto-salvataggio**: Ogni modifica viene salvata automaticamente
- **Purge intelligente**: Pulizia automatica snapshot vecchi
- **Sistema reboot**: Indicatori visivi e shortcut dedicati

### Versione TUI Rust (`btrbk_restore_rust/`)
- **Performance ottimizzata**: Implementazione nativa in Rust
- **Interfaccia identica**: Layout e funzionalità identiche alla versione Pro Python
- **Configurazione condivisa**: Usa esattamente lo stesso file JSON della versione Python
- **Schermata settaggi completa**: Stesse funzionalità di modifica della versione Python
- **Gestione memoria efficiente**: Ideale per sistemi con risorse limitate
- **Compatibilità totale**: Zero differenze funzionali con la versione Python Pro
- **Compilazione ottimizzata**: Rust edition 2021, zero errori e warning
- **Purge e reboot**: Tutte le funzionalità avanzate implementate

## Struttura Snapshot Supportata

Il tool gestisce snapshot con questa nomenclatura:
- `@.YYYYMMDD_HHMMSS` - Snapshot del subvolume root
- `@home.YYYYMMDD_HHMMSS` - Snapshot del subvolume home
- `@games.YYYYMMDD_HHMMSS` - Snapshot del subvolume games

## Controlli TUI

### Versioni con 3 colonne (TUI Pro Python/Rust):

#### Schermata Principale:
- **↑↓**: Navigazione verticale negli snapshot
- **←→**: Cambio colonna (ROOT ↔ HOME ↔ GAMES)
- **ENTER**: Selezione e ripristino snapshot
- **S**: Accesso schermata settaggi
- **R**: Refresh lista snapshot
- **P**: Purge snapshot vecchi (mantiene solo il più recente per tipo)
- **H**: Riavvio sistema (quando necessario)
- **Q**: Uscita dall'applicazione

#### Schermata Settaggi:
- **↑↓**: Navigazione tra le opzioni
- **ENTER**: Modifica valore (per stringhe)
- **SPACE**: Toggle valore (per booleani)
- **S**: Salvataggio manuale (opzionale, auto-salvataggio attivo)
- **ESC**: Ritorno alla schermata principale

### Funzionalità Avanzate:

#### Purge Intelligente (Tasto P):
- **Analizza** tutti gli snapshot per tipo (@, @home, @games)
- **Mantiene** solo lo snapshot più recente per ogni tipo
- **Elimina** tutti gli snapshot più vecchi automaticamente
- **Conferma** prima dell'operazione per sicurezza
- **Feedback** dettagliato su quanti snapshot sono stati eliminati
- **Gestione errori**: Continua l'operazione anche se singole eliminazioni falliscono
- **Ottimizzazione spazio**: Libera automaticamente spazio disco mantenendo i backup essenziali

#### Reboot Intelligente:
- **Tasto R**: Sempre disponibile per refresh lista snapshot
- **Tasto H**: Appare nel footer dopo un restore per riavvio rapido
- **Warning persistente**: Barra di stato mostra "⚠ REBOOT REQUIRED" dopo ogni restore
- **Tasti dedicati**: R per refresh, H per reboot, P per purge - nessuna confusione
- **Indicatori visivi**: Footer dinamico che cambia in base al contesto

## File Desktop

Incluso `snapshot-restore.desktop` per l'integrazione nel desktop environment.

## Sicurezza

⚠️ **ATTENZIONE**: Questi strumenti richiedono privilegi di root e modificano i subvolumi del sistema. Usare con cautela e sempre dopo aver verificato la presenza di backup validi.

### Misure di Sicurezza Implementate:
- **Conferme obbligatorie**: Dialog di conferma per tutte le operazioni critiche
- **Backup automatico**: I subvolumi esistenti vengono rinominati in .BROKEN prima del ripristino
- **Gestione errori**: Operazioni robuste con fallback e messaggi di errore chiari
- **Auto-cleanup opzionale**: Pulizia automatica configurabile dei file .BROKEN

## Compatibilità

- **Sistema operativo**: Linux con filesystem Btrfs
- **Dipendenze**: btrfs-progs, btrbk
- **Desktop**: Testato su KDE Plasma, compatibile con altri DE
- **Subvolumi supportati**: @, @home, @games
- **Architetture**: x86_64, ARM64 (Rust), tutte le architetture supportate da Python

## Configurazione Avanzata

Entrambe le versioni TUI (Python Pro e Rust) condividono la configurazione salvata in:
```
~/.config/btrbk_restore/config.json
```

### Impostazioni configurabili:
- **btr_pool_dir**: Directory del pool Btrfs (default: `/mnt/btr_pool`)
- **snapshots_dir**: Directory degli snapshot (default: `/mnt/btr_pool/btrbk_snapshots`)
- **auto_cleanup**: Auto-cleanup dei file .BROKEN (default: `false`)
- **confirm_actions**: Conferma delle azioni (default: `true`)
- **show_timestamps**: Visualizzazione timestamp formattati (default: `true`)
- **theme**: Tema dell'interfaccia (default: `"default"`)

### Esempio file di configurazione:
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

### Gestione Configurazione:
- **Caricamento automatico**: All'avvio di qualsiasi versione TUI
- **Salvataggio automatico**: Ad ogni modifica nelle versioni TUI
- **Sincronizzazione**: Modifiche in una versione si applicano immediatamente all'altra
- **Fallback**: Se il file è corrotto o mancante, vengono usati i valori di default

## Quale Versione Scegliere?

### **CLI (`btrbk_restore.py`)**
- ✅ Uso occasionale o sporadico
- ✅ Script automatizzati
- ✅ Ambienti con risorse limitate
- ✅ Quando serve solo funzionalità base

### **TUI Pro (`btrbk_restore_tui_pro.py`)**
- ✅ Uso frequente e interattivo
- ✅ Configurazione avanzata e personalizzazione
- ✅ Quando Python è preferito per modifiche
- ✅ Sviluppo e debugging
- ✅ Gestione completa degli snapshot

### **Rust (`btrbk_restore_rust/`)**
- ✅ Massime performance e velocità
- ✅ Sistemi con Python limitato o assente
- ✅ Ambienti di produzione
- ✅ Quando serve efficienza di memoria
- ✅ Tutte le funzionalità della versione Pro

## Vantaggi dell'Allineamento Completo

### **Configurazione Unificata:**
- Un solo file di configurazione per entrambe le versioni TUI
- Modifiche sincronizzate automaticamente
- Esperienza utente coerente

### **Funzionalità Identiche:**
- Stessa interfaccia e controlli
- Stesse opzioni di configurazione
- Stesso comportamento e workflow
- Stesse funzionalità avanzate (purge, reboot, settaggi)

### **Flessibilità Totale:**
- Passa da Python a Rust senza perdere configurazioni
- Scegli il linguaggio in base alle esigenze specifiche
- Manutenzione semplificata con configurazione condivisa

### **Performance Ottimizzate:**
- Python: Facilità di modifica e debugging
- Rust: Velocità di esecuzione e efficienza memoria
- Entrambe: Stessa esperienza utente

## Struttura Progetto

```
btrbk_restore/
├── README.md                      # Documentazione completa
├── btrbk_restore.py              # Versione CLI semplice
├── btrbk_restore_tui_pro.py      # Versione TUI professionale Python
├── btrbk_restore_rust/           # Versione TUI professionale Rust
│   ├── Cargo.toml               # Configurazione Rust (edition 2021)
│   ├── src/main.rs              # Codice sorgente Rust
│   └── target/release/          # Binario compilato
├── snapshot-restore.desktop      # File desktop per integrazione DE
└── .git/                         # Repository Git
```

## Sviluppo e Contributi

### **Linguaggi utilizzati:**
- **Python 3**: Versioni CLI e TUI Pro
- **Rust 2021**: Versione TUI performante
- **JSON**: Configurazione condivisa

### **Dipendenze:**
- **Python**: modulo `curses`, `json`, `pathlib`, `subprocess`, `os`
- **Rust**: `ncurses`, `serde`, `serde_json`, `chrono`, `dirs`, `libc`

### **Testing:**
- Testato su Arch Linux con KDE Plasma 6
- Compatibile con altri desktop environment Linux
- Supporto completo per filesystem Btrfs

## Workflow Tipico di Utilizzo

1. **Avvio**: `sudo ./btrbk_restore_tui_pro.py` o versione Rust
2. **Navigazione**: Usa frecce per esplorare snapshot disponibili
3. **Configurazione**: Premi `S` per modificare settaggi se necessario
4. **Selezione**: Scegli snapshot da ripristinare con `ENTER`
5. **Conferma**: Conferma l'operazione di ripristino
6. **Reboot**: Scegli se riavviare immediatamente o continuare
7. **Pulizia**: Usa `P` per eliminare snapshot vecchi quando necessario
8. **Riavvio rapido**: Usa `H` per riavviare quando indicato

## Licenza

Progetto open source - vedere il codice sorgente per i dettagli di implementazione.

## Contributi

Contributi benvenuti! Il progetto dimostra l'implementazione della stessa funzionalità in linguaggi diversi (Python/Rust) con interfacce ottimizzate per diversi casi d'uso, mantenendo piena compatibilità di configurazione e esperienza utente identica.

### **Caratteristiche del progetto:**
- Architettura modulare e ben strutturata
- Configurazione condivisa tra linguaggi diversi
- Interfacce utente professionali e intuitive
- Gestione robusta degli errori
- Performance ottimizzate per ogni linguaggio
- Documentazione completa e aggiornata
- Funzionalità avanzate di gestione snapshot
- Sistema di sicurezza integrato
