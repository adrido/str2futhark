# str2futhark

Kleines CLI-Tool zur Transliteration von Text in Elder Futhark Runen.

Hauptsächlich ein Test wie weit eine KI schon code generieren kann

## Inhaltsverzeichnis

- Kurzbeschreibung
- Installation
- Nutzung
- Tests
- Beitragende
- Lizenz
- Danksagung

## Kurzbeschreibung

`str2futhark` nimmt Text (Positionsargumente oder Datei) und transliteriert ihn in
Elder Futhark Runen. Das Tool bietet einfache Mappings, unterstützt einige
digraphen und skandinavische Zeichen.

## Installation

Voraussetzung: Rust und Cargo installiert (https://www.rust-lang.org/tools/install).

Im Projektverzeichnis bauen:

```bash
cargo build --release
```

Für schnelle Entwicklung/test:

```bash
cargo run -- "Dein Text hier"
```

## Nutzung

Hilfe und Optionen:

```bash
cargo run -- --help
```

Beispiele:

- Positionsargumente (einfach):

```bash
cargo run -- "Hello World"
# Ausgabe: ᚺᛖᛚᛚᛟ ᚹᛟᚱᛚᛞ
```

- Aus Datei lesen und in Datei schreiben:

```bash
echo "Hallo Datei" > in.txt
cargo run -- -i in.txt -o out.txt
cat out.txt
```

- Wenn keine Eingabe angegeben ist, zeigt das Programm die Hilfe an:

```bash
cargo run --
```

## Tests

Unit-Tests sind im Modul `src/translit.rs` enthalten. Führe sie mit:

```bash
cargo test
```

## Beitragende

Beiträge, Fehlerberichte und Vorschläge sind willkommen. Öffne Issues oder Pull
Requests im Repository. Für kleinere Änderungen kannst du direkt Tests und
Dokumentation ergänzen.

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Der vollständige Lizenztext
ist in der Datei `LICENSE` enthalten.

Kurzfassung: Du darfst die Software nutzen, kopieren, ändern, fusionieren,
veröffentlichen, vertreiben, unterlizenzieren und/oder verkaufen, solange der
obige Urheberrechtshinweis und dieser Genehmigungshinweis in allen Kopien oder
wesentlichen Teilen der Software enthalten sind.

## Danksagung / Hinweis zur Entstehung

Dieses Programm wurde mithilfe von KI-Unterstützung erstellt und anschließend
manuell geprüft und angepasst.
