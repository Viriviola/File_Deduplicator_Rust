# File_Depduplicator_Rust

# 🧠 Intelligent File Deduplicator (Rust)

This is a beginner-friendly Rust project that scans a directory and detects duplicate files based on content. It safely quarantines duplicates and generates detailed reports in JSON or HTML format.

---

## ✨ Features

* ✅ **Multi-Algorithm Hashing** (SHA-256 implemented)
* 🚀 **Parallel Processing** using Rayon for fast hashing
* 🧰 **Advanced Filtering**: by file size, extension, date modified, and name patterns
* 🛡️ **Safe Quarantine System** before deletion
* 📊 **Detailed Reports**: JSON + HTML with space-saving stats
* 🧪 **Unit Tests** included (in `tests/` folder)

---

## 📁 Project Structure

```
file_deduplicator/
├── src/
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library module declarations
│   ├── cli.rs              # Command-line argument parser
│   ├── scan.rs             # File collection logic
│   ├── filter.rs           # File filtering by size, extension, etc.
│   ├── hash.rs             # File hashing (SHA-256)
│   ├── group.rs            # Grouping and duplicate detection
│   ├── quarantine.rs       # Safe quarantine of duplicates
│   ├── report.rs           # JSON/HTML report generation
│   └── models.rs           # Data structures (Report, DuplicateGroup)
├── tests/
│   └── tests.rs            # Integration tests
├── templates/
│   └── report.html         # HTML report template (Tera)
├── Cargo.toml              # Dependencies and metadata
└── README.md               # This file
```

---

## ⚙️ Usage

### 🛠️ Build

```bash
cargo build
```

### 🚀 Run

```bash
cargo run -- <folder_path> --report html --quarantine true
```

Example:

```bash
cargo run -- E:/File_Dedup_Rust/input --report json
```

### 📄 CLI Options

| Flag           | Description                     | Default    |
| -------------- | ------------------------------- | ---------- |
| `path`         | Path to folder to scan          | (Required) |
| `--report`     | Report format: `json` or `html` | `json`     |
| `--quarantine` | Whether to move duplicates      | `true`     |

---

## 🧪 Testing

```bash
cargo test
```

Tests include:

* File filtering
* Duplicate grouping
* Quarantine logic
* Report generation (JSON & HTML)

---

## 🛠 Dependencies

The following crates are used:

| Crate        | Purpose                       |
| ------------ | ----------------------------- |
| `rayon`      | Parallel file hashing         |
| `sha2`       | SHA-256 file hashing          |
| `clap`       | Command-line interface        |
| `serde`      | Serialization for reports     |
| `serde_json` | JSON report generation        |
| `tera`       | HTML report templating        |
| `walkdir`    | Recursive directory traversal |
| `tempfile`   | Temp dirs in unit tests       |

---

## 📊 Sample Output (JSON)

```json
{
  "scanned": 10,
  "duplicates_found": 2,
  "space_savings": 2048,
  "space_savings_size": "2.00 KB",
  "duplicate_groups": [
    {
      "hash": "...",
      "files": ["a.txt", "b.txt"]
    }
  ]
}
```

---

## 📌 Notes

* Only SHA-256 is currently used; Blake3 and xxHash can be added.
* Filters are fixed in code (`.txt`, `.rs`, size > 0) — can be made dynamic via CLI.
* HTML report depends on the `templates/report.html` file using Tera.

---

## 💡 Future Improvements

* Configurable hashing algorithm via CLI
* GUI front-end for drag-and-drop usage
* More flexible filtering from CLI
* Real-time progress bar for hashing

---

## 👶 Beginner-Friendly Rust

This project intentionally uses **very simple** Rust patterns and avoids complex abstractions to help new Rustaceans get comfortable with real-world project structure.

---

## 🧑‍💻 Author

* Built as part of a guided Rust learning journey ✨
* Contributions and suggestions welcome!

