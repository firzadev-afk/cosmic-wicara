//! File chooser task.

use cosmic::app::Task;

use super::config::{DialogConfig, DialogRequest, FileOutcome, RunnerMessage};

/// File chooser task.
pub(crate) fn file_task(cfg: DialogConfig) -> Task<RunnerMessage> {
    cosmic::task::future(async move {
        use cosmic::dialog::file_chooser::{self, open, save};
        let DialogRequest::File(sel) = cfg.request else {
            return RunnerMessage::FileChosen(FileOutcome::Cancelled);
        };
        if sel.save {
            let dialog = save::Dialog::new().title(cfg.title.clone());
            return match dialog.save_file().await {
                Ok(response) => match response.url() {
                    Some(url) => match url.to_file_path() {
                        Ok(path) => RunnerMessage::FileChosen(FileOutcome::Chosen(vec![path])),
                        Err(_) => RunnerMessage::FileChosen(FileOutcome::Error(
                            "not a file path".to_owned(),
                        )),
                    },
                    None => RunnerMessage::FileChosen(FileOutcome::Cancelled),
                },
                Err(file_chooser::Error::Cancelled) => {
                    RunnerMessage::FileChosen(FileOutcome::Cancelled)
                }
                Err(err) => RunnerMessage::FileChosen(FileOutcome::Error(format!("{err:?}"))),
            };
        }
        let dialog = open::Dialog::new().title(cfg.title.clone());
        let result = if sel.directory {
            if sel.multiple {
                dialog.open_folders().await.map(|r| r.urls().to_vec())
            } else {
                dialog
                    .open_folder()
                    .await
                    .map(|r| r.url().clone())
                    .map(|u| vec![u])
            }
        } else if sel.multiple {
            dialog.open_files().await.map(|r| r.urls().to_vec())
        } else {
            dialog
                .open_file()
                .await
                .map(|r| r.url().clone())
                .map(|u| vec![u])
        };
        match result {
            Ok(urls) => {
                let mut paths = Vec::new();
                for url in urls {
                    match url.to_file_path() {
                        Ok(path) => paths.push(path),
                        Err(_) => {
                            return RunnerMessage::FileChosen(FileOutcome::Error(
                                "not a file path".to_owned(),
                            ));
                        }
                    }
                }
                if paths.iter().any(|p| !sel.matches_filter(p)) {
                    return RunnerMessage::FileChosen(FileOutcome::Error(
                        "file does not match filter".to_owned(),
                    ));
                }
                RunnerMessage::FileChosen(FileOutcome::Chosen(paths))
            }
            Err(file_chooser::Error::Cancelled) => {
                RunnerMessage::FileChosen(FileOutcome::Cancelled)
            }
            Err(err) => RunnerMessage::FileChosen(FileOutcome::Error(format!("{err:?}"))),
        }
    })
}
