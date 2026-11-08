# Changelog

> Semua perubahan penting untuk **cosmic-wicara** dicatat di file ini.
> Format mengikuti [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan [Semantic Versioning](https://semver.org/).
> File ini ditulis agar mudah dibaca: setiap entri menjelaskan *apa yang berubah* dan *kenapa itu penting*.

---

## [Unreleased]

> Belum dirilis. Bagian ini berisi perubahan yang sudah selesai di kode tapi belum masuk rilis resmi.

### Added — Fitur baru

- **Semua 15 dialog + `about` sudah terhubung ke widget `libcosmic` asli:**
  - Input teks (`entry` / `password`)
  - File chooser via `xdg-portal` (file picker COSMIC)
  - Kalender (*calendar*), slider angka (*scale*), color picker (*color-selection*)
  - List interaktif (*list*), text viewer dengan checkbox (*text-info*)
  - Progress bar dari `stdin` (*progress*), notifikasi dengan `--listen` (*notification*)
  - Formulir multi-field (*forms*)
- **Opsi baru:**
  - `--extra-button` — tambah tombol ekstra (boleh diulang)
  - `--icon`, `--window-icon` — ikon dialog / ikon jendela
  - `--ellipsize`, `--no-wrap` — kontrol tampilan teks
  - `--switch`, `--default-cancel` — kontrol tombol & fokus
  - `--json` — hasil dialog sebagai JSON (untuk dibaca program)
  - `--lang=en|id` — bahasa teks bawaan dialog (English / Indonesia)
- **Kontrak hasil dialog (`DialogResult`):** data `stdout` dicetak sekali saat dialog selesai, sehingga mudah di-`capture` oleh script.

### Fixed — Perbaikan bug

- **Timeout sekarang konsisten exit `5` di semua jalur** (sebelumnya ada jalur yang salah keluar `1` karena pemetaan pesan).
- **Pembersihan brand:** tidak ada referensi eksternal tersisa di kode, halaman manual (`man/cosmic-wicara.1`), dan entri desktop (`cosmic-wicara.desktop`).
- **Dokumentasi:** folder `docs/` (berisi rencana lama) dihapus; seluruh dokumentasi kini terpusat di `README.md`, `CHANGELOG.md`, dan halaman manual `man/cosmic-wicara.1` supaya lebih mudah ditemukan dan dipahami.

---

## [0.1.0] — Rilis awal

> Rilis pertama: tool dialog native COSMIC (Rust + `libcosmic`, khusus COSMIC, Wayland-first).

### Added

- **15 dialog + `about`:**
  - `calendar`, `entry`, `error`, `info`, `file-selection`, `list`, `notification`, `progress`, `question`, `warning`, `scale`, `text-info`, `color-selection`, `password`, `forms`, plus `about`.
- **Opsi umum:**
  - `--title`, `--width`, `--height`, `--timeout`, `--ok-label`, `--cancel-label`
  - `--extra-button`, `--modal`
  - `--attach` (*deprecated*, no-op — dipertahankan untuk kompatibilitas script)
  - `--json`, `--lang=en|id`, `--window-icon`
- **Selector help per dialog:** `--help-<dialog>` (mis. `--help-question`) dan `--version`.
- **Kontrak exit & aliran:**
  - Exit `0` = OK/help, `1` = Cancel/close, `5` = timeout, `255` = error.
  - `stdout` hanya berisi data, `stderr` berisi diagnostik.
  - Tepat satu tipe dialog per pemanggilan.
- **Perilaku jendela COSMIC:**
  - Jendela dialog mengambang dengan ukuran konten (bukan jendela aplikasi yang di-tile).
  - Terang/gelap mengikuti `cosmic::Theme` (tema sistem COSMIC).
  - Frosted glass via `enable_blur(true)` (blur di belakang jendela).
  - Fokus diminta saat jendela dipetakan (*focus on map*).
- **Kemasan (packaging):**
  - Binary `cosmic-wicara` di `/usr/bin` (Arch `packaging/PKGBUILD`, `cargo build --release`, `[profile.release]` LTO + strip).
  - Halaman manual `man/cosmic-wicara.1`.
  - Entri desktop `cosmic-wicara.desktop` (`Exec=cosmic-wicara`, `Terminal=false`, `Categories=Utility;`).
- **Tooling:**
  - Resep `just`: `build`, `build-release`, `test`, `test-cli`, `install`, `fmt`, `clippy`, `list` (lihat `just --list`).

### Changed — Perubahan perilaku

- Tidak ada perubahan perilaku di rilis pertama (baseline).

### Fixed — Perbaikan bug

- Tidak ada perbaikan bug di rilis pertama (baseline).

---

## Panduan membaca changelog ini

| Judul bagian | Arti |
|---|---|
| `Added` | Fitur baru yang ditambahkan |
| `Changed` | Perubahan pada perilaku yang sudah ada |
| `Fixed` | Bug yang diperbaiki |
| `Unreleased` | Perubahan yang belum dirilis |
| `[0.1.0]` | Versi rilis, mengikuti Semantic Versioning |
