//! Per-dialog builders: Cli -> DialogRequest.

pub mod calendar;
pub mod color;
pub mod entry;
pub mod file_selection;
pub mod forms;
pub mod list;
pub mod message;
pub mod progress;
pub mod scale;
pub mod text_info;

use cli::args::DialogKind;

/// Default per-kind title.
pub fn default_title(kind: DialogKind, lang: &str) -> &'static str {
    let id = lang.eq_ignore_ascii_case("id");
    match kind {
        DialogKind::Calendar => {
            if id {
                "Kalender"
            } else {
                "Calendar"
            }
        }
        DialogKind::Entry => {
            if id {
                "Entri"
            } else {
                "Entry"
            }
        }
        DialogKind::Error => {
            if id {
                "Kesalahan"
            } else {
                "Error"
            }
        }
        DialogKind::Info => {
            if id {
                "Informasi"
            } else {
                "Information"
            }
        }
        DialogKind::FileSelection => {
            if id {
                "Pilih Berkas"
            } else {
                "File Selection"
            }
        }
        DialogKind::List => {
            if id {
                "Daftar"
            } else {
                "List"
            }
        }
        DialogKind::Notification => {
            if id {
                "Pemberitahuan"
            } else {
                "Notification"
            }
        }
        DialogKind::Progress => {
            if id {
                "Kemajuan"
            } else {
                "Progress"
            }
        }
        DialogKind::Question => {
            if id {
                "Pertanyaan"
            } else {
                "Question"
            }
        }
        DialogKind::Warning => {
            if id {
                "Peringatan"
            } else {
                "Warning"
            }
        }
        DialogKind::Scale => {
            if id {
                "Skala"
            } else {
                "Scale"
            }
        }
        DialogKind::TextInfo => {
            if id {
                "Info Teks"
            } else {
                "Text Info"
            }
        }
        DialogKind::ColorSelection => {
            if id {
                "Pemilihan Warna"
            } else {
                "Color Selection"
            }
        }
        DialogKind::Password => {
            if id {
                "Kata Sandi"
            } else {
                "Password"
            }
        }
        DialogKind::Forms => {
            if id {
                "Formulir"
            } else {
                "Forms"
            }
        }
        DialogKind::About => {
            if id {
                "Tentang cosmic-wicara"
            } else {
                "About cosmic-wicara"
            }
        }
    }
}

/// Today's date triple.
pub fn today() -> (u32, u32, u32) {
    let date = jiff::Zoned::now().date();
    (date.day() as u32, date.month() as u32, date.year() as u32)
}

/// Read piped stdin.
pub fn read_stdin() -> String {
    use std::io::Read;
    let mut text = String::new();
    let _ = std::io::stdin().read_to_string(&mut text);
    text
}
