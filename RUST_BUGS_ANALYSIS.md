# BTRBK Restore Rust - Bug Analysis Report

## 🚨 BUG CRITICI IDENTIFICATI

### **BUG #1 - Cargo.toml: Edition 2024** ✅ VERIFICATO OK
```toml
edition = "2024"  # ✅ ESISTE - Rilasciata con Rust 1.85.0 (Feb 2025)
```
**Status**: ✅ Confermato - Edition 2024 è disponibile e stabile

### **BUG #2 - Parsing timestamp difettoso** ❌ CRITICO
```rust
if let Ok(dt) = NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d_%H%M%S") {
```
**Problema**: Il formato `%Y%m%d_%H%M%S` non corrisponde ai timestamp reali (es: `20240802T0841`)
**Fix**: Dovrebbe essere `"%Y%m%dT%H%M"` o gestire entrambi i formati

### **BUG #3 - Gestione thread non sicura** ❌ CRITICO
```rust
let _ = process.kill();  // Potenziale race condition
let _ = stdout_thread.join();  // Potrebbe bloccarsi
```
**Problema**: Gestione non sicura della terminazione processo e thread

### **BUG #4 - Logica purge hardcoded** ❌ CRITICO
```rust
if name.starts_with("@.") || name.starts_with("@home.") || name.starts_with("@games.") {
```
**Problema**: Non dinamica come il resto del codice, ignora @custom, @backup, etc.
**Fix**: Rendere dinamica come `get_snapshots()`

### **BUG #5 - Restore function limitata** ❌ CRITICO
```rust
match snapshot_type {
    "root" => (...),
    "home" => (...), 
    "games" => (...),
    _ => return false,  // Fallisce per altri tipi!
}
```
**Problema**: Hardcoded per solo 3 tipi, dovrebbe essere dinamica

### **BUG #6 - Logica snapshot_type errata** ❌ CRITICO
```rust
let snapshot_type = if current_prefix.starts_with('@') {
    &current_prefix[1..] // "@backup" diventa "backup", ma restore_snapshot cerca "root"/"home"/"games"
} else {
    current_prefix
};
```
**Problema**: Mismatch tra tipo generato e tipi accettati da restore_snapshot

### **BUG #7 - Comando mv con subvolumi btrfs** ✅ VERIFICATO CORRETTO
```rust
if !run_command(&["mv", &current_subvol.to_string_lossy(), &broken_subvol.to_string_lossy()]) {
```
**Status**: ✅ CONFERMATO CORRETTO - `mv` è il metodo giusto per:
- Subvolumi nello stesso filesystem btrfs (rename istantaneo)
- Subvolumi montati (snapshot+delete non funzionerebbe)
- Confermato funzionante su Arch Linux dall'utente

## PRIORITÀ CORREZIONI

1. **BUG #2, #4, #5, #6**: Critici per funzionalità dinamica
2. **BUG #3**: Critico per stabilità
3. **BUG #1**: Verificare disponibilità edition 2024
4. **BUG #7**: Verificato funzionante dall'utente

## STATO CORREZIONI

### **RUST VERSION:**
- ✅ BUG #1: Verificato - Edition 2024 esiste ed è stabile
- ✅ BUG #2: CORRETTO - Parsing timestamp ora supporta entrambi i formati
- ✅ BUG #3: CORRETTO - Gestione thread più sicura con timeout
- ✅ BUG #4: CORRETTO - Logica purge ora completamente dinamica
- ✅ BUG #5: CORRETTO - Restore function ora supporta tutti i tipi dinamicamente
- ✅ BUG #6: CORRETTO - Logica snapshot_type corretta per tutti i prefissi
- ✅ BUG #7: Verificato - mv funziona come confermato dall'utente
- ✅ **BONUS**: Timestamp unico per .BROKEN (risolve conflitti)
- ✅ **BONUS**: Riquadro log semplificato (solo bordi orizzontali)

### **PYTHON TUI VERSION:**
- ✅ BUG #2: CORRETTO - Parsing timestamp ora supporta entrambi i formati
- ✅ BUG #4: CORRETTO - Aggiunta funzione purge_old_snapshots dinamica
- ✅ BUG #5: CORRETTO - Restore function ora supporta tutti i tipi dinamicamente
- ✅ BUG #6: CORRETTO - Logica snapshot_type corretta per tutti i prefissi
- ✅ **BONUS**: Timestamp unico per .BROKEN (risolve conflitti)
- ✅ **BONUS**: Riquadro log semplificato (solo bordi orizzontali)

### **PYTHON CLI VERSION:**
- ✅ **CORRETTO**: Timestamp unico per .BROKEN (risolve conflitti preesistenti)
- ✅ **CORRETTO**: Logica subvolume_name consistente con versioni TUI
- ✅ **AGGIUNTO**: Parsing timestamp opzionale per display migliorato
- ✅ **VERIFICATO**: Mantiene semplicità CLI senza configurazione complessa

## RISULTATO FINALE
✅ **TUTTE E TRE LE VERSIONI CORRETTE** - Rust TUI, Python TUI e Python CLI ora completamente dinamici, sicuri e allineati