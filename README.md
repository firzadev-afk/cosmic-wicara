# cosmic-wicara

> Alat dialog native untuk desktop **COSMIC** — ditulis dalam Rust + `libcosmic`, satu biner `cosmic-wicara` untuk shell script, installer, dan workflow desktop di COSMIC.

**COSMIC-only, Wayland-first.** Cocok untuk sesi COSMIC. Bukan aplikasi GTK, bukan portable ke desktop environment lain (GNOME/KDE/XFCE/dsb), dan tidak untuk X11.

![License](https://img.shields.io/badge/license-GPL--2.0--or--later-blue) ![Rust](https://img.shields.io/badge/rust-1.87%2B-orange) ![COSMIC](https://img.shields.io/badge/COSMIC-native-informational)

---

## Daftar Isi

- [Apa Itu cosmic-wicara?](#apa-itu-cosmic-wicara)
- [Fungsinya Buat Apa?](#fungsinya-buat-apa)
- [Kapan Pakai / Kapan Tidak](#kapan-pakai--kapan-tidak)
- [Fitur Utama](#fitur-utama)
- [Requirements](#requirements)
- [Cara Install](#cara-install)
- [Cara Pakai](#cara-pakai)
- [Contoh Penggunaan](#contoh-penggunaan)
- [Kontrak CLI: stdout, stderr & Exit Code](#kontrak-cli-stdout-stderr--exit-code)
- [Daftar Dialog](#daftar-dialog)
- [Opsi Umum](#opsi-umum)
- [Struktur Project](#struktur-project)
- [Build & Pengembangan](#build--pengembangan)
- [Troubleshooting](#troubleshooting)
- [Kontribusi](#kontribusi)
- [Lisensi](#lisensi)

---

## Apa Itu cosmic-wicara?

**cosmic-wicara** adalah *dialog tool* berbentuk command-line (CLI) yang menampilkan jendela dialog native COSMIC dari terminal / shell script, lalu mengembalikan hasil pilihan pengguna lewat `stdout` dan *exit code*.

Analogi sederhananya: seperti `zenity` / `yad` untuk GTK, tapi versi native untuk COSMIC — memakai `libcosmic` langsung, bukan GTK, bukan web view, bukan wrapper.

Kamu memanggilnya dari script:

```bash
cosmic-wicara --question --title="Konfirmasi" --text="Lanjut?"
```

COSMIC akan menampilkan jendela dialog. Jawaban user (OK / Cancel / teks yang diketik / file yang dipilih / angka slider / dsb.) dikembalikan ke script sebagai *teks di stdout* dan *exit code*.

---

## Fungsinya Buat Apa?

cosmic-wicara dibuat untuk menjembatani **shell script ↔ pengguna desktop COSMIC**. Fungsi utamanya:

| Fungsi | Contoh di dunia nyata |
|---|---|
| **Konfirmasi** | Script installer bertanya *"Install sekarang?"*, script backup bertanya *"Timpa file lama?"* |
| **Input teks** | Minta nama file, nama user, password, atau isian singkat tanpa harus buka terminal interaktif |
| **Pilih file / folder** | Installer atau script utilitas meminta user memilih lokasi file/folder lewat file chooser native COSMIC (via `xdg-portal`) |
| **Pilih tanggal, warna, angka** | Form sederhana: kalender, color picker, slider angka (mis. pilih ukuran / level) |
| **Tampilkan info / error / warning** | Script selesai → tampilkan info; script gagal → tampilkan error dengan ikon yang pas |
| **Progress bar** | Proses panjang (copy, render, download) menampilkan progress yang di-update dari `stdin` |
| **Notifikasi** | Script background memberi tahu user *"Selesai"* lewat notification bubble COSMIC |
| **Tampilkan teks / lisensi** | Menampilkan file teks, README, atau lisensi, dengan opsi checkbox *"I have read and agree"* |
| **Formulir** | Satu dialog berisi beberapa field sekaligus (entry, password, kalender, list, combo) |
| **Otomasi desktop** | Shortcut, launcher, `.desktop`, atau script saat startup yang butuh interaksi visual |

**Intinya:** setiap kali script butuh *tanya, minta input, kasih tahu, atau minta pilih* dari user — tanpa menulis GUI sendiri — pakai cosmic-wicara.

### Yang BUKAN fungsi cosmic-wicara

- Bukan aplikasi GUI mandiri yang dijalankan sendirian (selalu dipanggil dari script/CLI).
- Bukan pengganti GTK/Qt di desktop lain — hanya COSMIC.
- Bukan tool terminal TUI — output dialog-nya berupa jendela COSMIC.
- Bukan library untuk aplikasi lain — ini biner CLI (`cosmic-wicara`).

---

## Kapan Pakai / Kapan Tidak

**Pakai cosmic-wicara kalau:**

- Kamu menulis shell script / installer di COSMIC dan butuh dialog visual.
- Kamu mau hasil dialog kembali ke script (`stdout` + exit code).
- Kamu ingin tampilan benar-benar native COSMIC (tema terang/gelap mengikuti sistem, blur kaca, dekorasi COSMIC).

**Jangan pakai kalau:**

- Kamu di GNOME/KDE/XFCE — pakai `zenity` / `yad` / engine GTK native-nya masing-masing.
- Kamu butuh X11 — cosmic-wicara Wayland-first untuk sesi COSMIC.
- Kamu butuh dialog yang berjalan tanpa display server (headless).

---

## Fitur Utama

- **16 pilihan dialog** — 15 dialog interaktif + 1 dialog `about` (detail di [Daftar Dialog](#daftar-dialog)).
- **Widget nyata per dialog** — bukan gambar statis:
  - File chooser via `xdg-portal` (Cosmic file picker native).
  - Kalender picker, slider, color picker.
  - List dengan checklist / radio / editable.
  - Text viewer dengan `--checkbox`, progress bar yang di-feed dari `stdin`.
- **Opsi umum fleksibel** — judul, ukuran, timeout, label tombol, tombol ekstra, mode modal, ikon, bahasa (`--lang=en|id`), output `--json`, dll.
- **Kontrak CLI bersih:**
  - `stdout` = data hasil dialog saja (teks, path file, angka, dsb).
  - `stderr` = diagnostik / pesan error.
  - Tepat **satu** tipe dialog per pemanggilan (dua atau lebih → exit `255`).
- **Mengikuti tema COSMIC** — terang & gelap mengikuti `cosmic::Theme` (tema sistem COSMIC).
- **Frosted glass** — jendela memakai `enable_blur(true)`, blur di belakang jendela seperti dialog native COSMIC. Tanpa shadow buatan.
- **Jendela mengambang, ukurannya mengikuti konten** — bukan jendela aplikasi yang di-tile compositor, tanpa header ganda, tanpa double title.
- **Exit code jelas** — `0` OK/help/`--version`, `1` Cancel/close, `5` timeout, `255` error (lihat tabel lengkap di bawah).

---

## Requirements

- Sistem dengan sesi **COSMIC** (Wayland).
- Untuk build dari source:
  - `cargo 1.99.1` atau lebih baru, Rust Edition 2024, `rust-version = 1.87`.
  - `just` (opsional, untuk jalanin shortcut di `justfile`).
  - `libcosmic` git-pinned di `dd108aa99d7306110eef06da43eafc109e1ab151` (features `about,wgpu,xdg-portal`) — sudah terkunci di `Cargo.lock`, tidak perlu install manual.
  - `clap 4.6.7` (derive) sebagai batas CLI.

---

## Cara Install

### A. Install lokal (user, tanpa root)

```bash
cargo build --release -p cli      # binary rilis: LTO + strip
just install                      # -> ~/.local/bin/cosmic-wicara
```

atau manual:

```bash
install -Dm755 target/release/cosmic-wicara "$HOME/.local/bin/cosmic-wicara"
```

Pastikan `~/.local/bin` ada di `PATH`.

### B. Install sistem-wide (Arch Linux)

```bash
cd packaging && makepkg -sif      # lihat packaging/PKGBUILD
```

Akan menginstall:

- `/usr/bin/cosmic-wicara`
- `/usr/share/man/man1/cosmic-wicara.1`
- `/usr/share/applications/cosmic-wicara.desktop`

### C. Cek install

```bash
cosmic-wicara --version
cosmic-wicara --help
man cosmic-wicara
```

---

## Cara Pakai

### Sintaks umum

```bash
cosmic-wicara <satu-dialog-type> [opsi-umum] [opsi-spesifik-dialog]
```

Aturan penting:

1. **Wajib tepat satu** `--dialog-type` per pemanggilan (mis. `--question`, `--entry`, `--info`).
2. **Tanpa dialog type** → error, exit `255`, pesan di `stderr`.
3. **Dua dialog type atau lebih** → error, exit `255`.
4. Hasil data → `stdout`. Diagnostik → `stderr`.
5. Gunakan `--help-<dialog>` untuk bantuan spesifik, contoh: `cosmic-wicara --help-question`.
6. Gunakan `--help-general` / `--help` / `--help-all` untuk bantuan umum.

> Jangan jalankan dialog ini dari `cosmic-term -e`. Ini adalah dialog, bukan terminal — file `.desktop`-nya memakai `Terminal=false` dan `Exec=cosmic-wicara`.

### Contoh Penggunaan

```bash
# Tanya konfirmasi — exit 0 kalau OK, 1 kalau Cancel
cosmic-wicara --question --title="Konfirmasi" --text="Lanjut?"

# Minta input teks — teks yang diketik dicetak ke stdout
cosmic-wicara --entry --title="Login" --text="Masukkan nama:"

# Pilih folder (boleh banyak) — path dicetak, satu per baris
cosmic-wicara --file-selection --directory --multiple

# Progress bar — baca persen dari stdin
cosmic-wicara --progress --pulsate --title="Bekerja"

# Notifikasi selesai
cosmic-wicara --notification --text="Selesai" --icon=dialog-information

# Bantuan per dialog
cosmic-wicara --help-question

# Manual lengkap
man cosmic-wicara
```

### Pola umum di shell script

```bash
#!/usr/bin/env bash

if cosmic-wicara --question --title="Backup" --text="Backup sekarang?"; then
  echo "OK ditekan"
else
  code=$?
  case $code in
    1) echo "User membatalkan" ;;
    5) echo "Timeout" ;;
    255) echo "Error: dialog tidak muncul" ;;
  esac
fi

# Ambil input user dari stdout
name=$(cosmic-wicara --entry --title="Nama" --text="Nama kamu:")
echo "Halo, $name"
```

Gunakan flag yang sama di semua script; jaga `stdout` tetap bersih untuk data, dan tulis diagnostik ke `stderr`.

---

## Kontrak CLI: stdout, stderr & Exit Code

### stdout vs stderr

| Aliran | Isi | Untuk apa |
|---|---|---|
| `stdout` | **Data hasil dialog saja** — teks input, path file, angka slider, tanggal, dsb. | Di-`capture` script (`$(...)`, pipe, dsb.) |
| `stderr` | Diagnostik, pesan error, help error | Dibaca manusia, tidak mengotori data |

### Exit code

| Code | Arti | Kapan terjadi |
|---|---|---|
| `0` | OK / help / `--version` | User menekan OK, atau kamu memanggil `--help` / `--version` |
| `1` | Cancel / tutup jendela | User menekan Cancel, menutup jendela, atau dialog di-close |
| `5` | Timeout | `--timeout` tercapai sebelum user menjawab |
| `255` | Error | Opsi dialog tidak dikenal / hilang / tidak valid, atau dua dialog type sekaligus |

### Opsi output & bahasa

| Opsi | Efek |
|---|---|
| `--json` | Hasil dialog dicetak sebagai JSON (machine-readable) |
| `--lang=en\|id` | Bahasa label / teks bawaan dialog (`en` = English, `id` = Indonesia) |

---

## Daftar Dialog

Tepat satu dari daftar ini per pemanggilan. Gunakan `--help-<nama-dialog>` untuk opsi lengkapnya, atau `man cosmic-wicara`.

| # | Flag | Dialog | Kapan dipakai |
|---|---|---|---|
| 1 | `--calendar` | Kalender | Pilih tanggal (day/month/year, format output via `--date-format`) |
| 2 | `--entry` | Input teks | Minta teks singkat dari user (teks awal via `--entry-text`, sembunyikan via `--hide-text`) |
| 3 | `--error` | Error | Tampilkan pesan error (ikon, wrap, markup, ellipsize) |
| 4 | `--info` | Info | Tampilkan pesan informasi |
| 5 | `--file-selection` | Pilih file/folder | File chooser native COSMIC via `xdg-portal` (multiple, directory, save, separator, filter) |
| 6 | `--list` | Daftar | Pilih dari daftar (checklist, radiolist, imagelist, editable, print-column) |
| 7 | `--notification` | Notifikasi | Notification bubble COSMIC (ikon, `--listen` baca perintah dari stdin) |
| 8 | `--progress` | Progress bar | Progress bar yang di-update dari stdin (`--pulsate`, `--auto-close`, `--no-cancel`, dll.) |
| 9 | `--question` | Pertanyaan | Tanya OK/Cancel (ikon, default-cancel, switch) |
| 10 | `--warning` | Peringatan | Tampilkan peringatan |
| 11 | `--scale` | Slider angka | Pilih angka (value, min/max, step, hide-value) |
| 12 | `--text-info` | Teks info | Tampilkan file/teks (editable, font, checkbox persetujuan) |
| 13 | `--color-selection` | Color picker | Pilih warna (warna awal, palette) |
| 14 | `--password` | Password | Input password (opsi `--username` untuk field nama user) |
| 15 | `--forms` | Formulir | Satu dialog berisi banyak field (entry, password, kalender, list, combo, dsb.) |
| 16 | `--about` | About | Tampilkan info tentang cosmic-wicara |

**Contoh per kelompok:**

```bash
# Pesan: info / error / warning / question
cosmic-wicara --info --title="Info" --text="Backup selesai."
cosmic-wicara --error --text="Gagal menulis file."
cosmic-wicara --question --text="Lanjutkan?" --default-cancel

# Input: entry / password / scale / calendar / color
cosmic-wicara --entry --text="Nama:" --entry-text="Budi"
cosmic-wicara --password --username
cosmic-wicara --scale --min-value=0 --max-value=100 --value=50
cosmic-wicara --calendar --date-format="%Y-%m-%d"
cosmic-wicara --color-selection --show-palette

# Pilih: file / list / forms
cosmic-wicara --file-selection --filename="$HOME" --directory
cosmic-wicara --list --column="Nama" --column="Ukuran" --checklist
cosmic-wicara --forms --add-entry="Nama" --add-password="Password"

# Tampil: text-info / progress / notification
cosmic-wicara --text-info --filename=LICENSE --checkbox="Saya sudah membaca"
echo 50 | cosmic-wicara --progress --title="Copying"
cosmic-wicara --notification --text="Done" --icon=dialog-information
```

---

## Opsi Umum

Opsi ini berlaku untuk semua dialog di mana relevan. Lihat `cosmic-wicara --help-general` untuk versi lengkapnya.

| Opsi | Arti |
|---|---|
| `--title` | Judul jendela dialog |
| `--width` | Lebar jendela (`0` = default 400px) |
| `--height` | Tinggi jendela (`0` = mengikuti konten) |
| `--timeout` | Timeout detik (exit `5` saat timeout) |
| `--ok-label` | Label tombol OK (default: `OK`) |
| `--cancel-label` | Label tombol Cancel (default: `Cancel`) |
| `--extra-button` | Tambah tombol ekstra (boleh diulang) |
| `--modal` | Hint modal |
| `--attach` | *Deprecated, no-op* — dipertahankan untuk kompatibilitas script |
| `--json` | Output hasil sebagai JSON |
| `--lang=en\|id` | Bahasa dialog (`en` / `id`) |
| `--window-icon` | Ikon jendela |
| `--icon` | Ikon dialog per-item (per-dialog) |
| `--ellipsize` | Aktifkan ellipsize teks |
| `--no-wrap` | Matikan text wrapping |
| `--switch` | Mode switch (menekan OK/Cancel) |
| `--default-cancel` | Fokus awal di tombol Cancel |

**Help & versi:**

| Opsi | Arti |
|---|---|
| `--help`, `-h` | Tampilkan help semua tipe dialog |
| `--help-general` | Help opsi umum |
| `--help-all` | Semua opsi help |
| `--help-<dialog>` | Help satu dialog, mis. `--help-question`, `--help-file-selection` |
| `--version` | Tampilkan versi |

---

## Struktur Project

```text
cosmic-wicara/
├── Cargo.toml              # [workspace] + [profile.release] lto/strip
├── README.md               # dokumen utama (file ini)
├── CHANGELOG.md            # riwayat perubahan
├── LICENSE                 # GPL-2.0-or-later
├── justfile                # shortcut build/test/clippy/fmt
├── cosmic-wicara.desktop   # Exec=cosmic-wicara, Terminal=false, Categories=Utility
├── man/
│   └── cosmic-wicara.1     # halaman manual
├── packaging/
│   └── PKGBUILD            # kemasan Arch (cargo build --release)
└── crates/
    ├── cli/                # paket `cli`, binary `cosmic-wicara`
    │   ├── src/main.rs     # entry tipis: parse -> hitung dialog -> dispatch
    │   ├── src/args/       # flag CLI (umum + per-dialog, via flatten)
    │   ├── src/builders/   # builder per-dialog (Cli -> DialogRequest)
    │   └── tests/          # test CLI (headless + exit code)
    ├── core/               # Dialog/Exit/theme/icons + notify (bus/listen)
    ├── app/                # shell jendela libcosmic (semua dialog)
    │   ├── src/runner/     # config/state/output/views (+file_task): runtime dialog
    │   ├── src/window/     # state shell + geometri + blur
    │   └── src/dialogs/    # satu file per dialog
    └── applet/             # applet tray (poll + tray)
```

### Cara kerja (alur data)

```text
Script (bash/installer)
        │
        ▼   cosmic-wicara --question --text="Lanjut?"
crates/cli (main.rs)
        │   parse clap -> hitung dialog (0/1/>1) -> dispatch
        ▼
crates/cli builders/          # ubah flag CLI menjadi DialogRequest
        │
        ▼
crates/app runner/            # jalankan jendela libcosmic COSMIC
        │   (theme, blur, geometry, fokus, timeout)
        ▼
Dialog COSMIC muncul ──► user klik / ketik / pilih
        │
        ▼
stdout = data hasil      stderr = diagnostik
exit 0 / 1 / 5 / 255
        │
        ▼
Script baca hasil & exit code
```

---

## Build & Pengembangan

| Perintah | Fungsi |
|---|---|
| `cargo build --release -p cli` | Build rilis (`LTO + strip` dari `[profile.release]`) |
| `just build` | Build debug seluruh workspace |
| `just build-release` | Sama dengan `cargo build --release -p cli` |
| `just test` | `cargo test --workspace` |
| `cargo test -p cli` | Test CLI saja (scaffold proof, harus tetap hijau) |
| `just test-cli` | Sama dengan di atas |
| `just install` | Install rilis ke `~/.local/bin/cosmic-wicara` |
| `just fmt` | `cargo fmt --check` |
| `just clippy` | `cargo clippy --workspace -- -D warnings` |
| `just clean` | `cargo clean` |
| `just list` | Daftar semua resep `just` |

### Status saat ini

- Builds dengan `cargo build --release -p cli` (LTO + strip).
- Test workspace, clippy, dan fmt hijau di CI.
- Jendela dialog mengambang dan ukurannya mengikuti konten: bukan jendela aplikasi yang di-tile, tanpa header compositor, tanpa double title.
- **Saran:** lakukan pengecekan visual di sesi COSMIC live sebelum rilis. Validasi headless (exit `255` untuk input buruk) sudah tercover test.

### Catatan desain

- `.desktop`: `Exec=cosmic-wicara`, `Terminal=false`, `Categories=Utility;`, `Keywords=dialog;wicara;cosmic;`, `Icon=dialog-information`.
- Terang/gelap mengikuti tema sistem COSMIC.
- Frosted glass adalah hint jendela `enable_blur(true)` di setiap dialog.
- Jendela dialog adalah surface mengambang dengan ukuran konten: `min == max` fixed, dekorasi client-side, dan fokus diminta saat map.

---

## Troubleshooting

| Gejala | Penyebab umum | Solusi |
|---|---|---|
| `You must specify a dialog type` (exit `255`) | Tidak ada `--dialog-type` | Tambahkan satu, mis. `--question` |
| `Two or more dialog options specified` (exit `255`) | Lebih dari satu dialog type | Pilih tepat satu dialog type |
| Opsi tidak dikenal (exit `255`) | Flag salah ketik / tidak didukung | Cek `--help` atau `--help-<dialog>` |
| Dialog tidak muncul sama sekali | Bukan di sesi COSMIC / tanpa display | Jalankan di sesi COSMIC (Wayland) |
| Timeout terus (exit `5`) | `--timeout` terlalu pendek | Naikkan `--timeout` atau hapus jika tidak perlu |
| File picker kosong / gagal | `xdg-portal` / COSMIC portal tidak tersedia | Pastikan portal COSMIC aktif di sesi |
| Output script "kotor" | Diagnostik tercampur ke `stdout` | Pisahkan: data di `stdout`, diagnostik di `stderr` |
| Binary tidak ketemu setelah `just install` | `~/.local/bin` tidak di `PATH` | Tambahkan ke `PATH` atau pakai path absolut |
| Ingin lihat semua opsi | — | `cosmic-wicara --help-all` atau `man cosmic-wicara` |

---

## Kontribusi

Kontribusi, laporan bug, dan feedback UX COSMIC sangat diterima. Saat membuka issue atau PR, sertakan:

- Tipe dialog dan pemanggilan CLI yang persis.
- Versi COSMIC/`libcosmic` dan tipe sesi (Wayland).
- Exit code / `stdout` / `stderr` yang diharapkan vs. yang sebenarnya.

Untuk pertanyaan desain se-COSMIC secara umum, lebih baik ke proyek upstream COSMIC/`libcosmic`; repo ini fokus pada perilaku dialog.

---

## Lisensi

GPL-2.0-or-later. Lihat [LICENSE](LICENSE).

---

*Dibangun untuk desktop COSMIC — native, ringan, dan mudah dipanggil dari shell script.*
