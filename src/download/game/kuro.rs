use std::path::Path;
use std::sync::{Arc, Mutex};
use crate::download::game::{Game, Kuro};
use crate::utils::downloader::Downloader;
use crate::utils::game::{list_kuro_integrity_files};
use crate::utils::KuroFile;

impl Kuro for Game {
    fn download(urls: Vec<KuroFile>, game_path: String, progress: impl Fn(u64, u64) + Send + 'static) -> bool {
        if urls.is_empty() || game_path.is_empty() {
            return false;
        }

        let progress = Arc::new(Mutex::new(progress));
        for url in urls {
            let p = progress.clone();
            let mut downloader = Downloader::new(url.url).unwrap();
            let file = url.path.to_string();
            downloader.download(Path::new(game_path.as_str()).to_path_buf().join(&file), move |current, total| {
                let pl = p.lock().unwrap();
                pl(current, total);
            }).unwrap();

        }
        true
    }

    fn patch(url: String, game_path: String, progress: impl Fn(u64, u64) + Send + 'static) -> bool {
        if url.is_empty() || game_path.is_empty() {
            return false;
        }

        let mut downloader = Downloader::new(url).unwrap();
        let file = downloader.get_filename().to_string();
        let dl = downloader.download(Path::new(game_path.as_str()).to_path_buf().join(&file), progress);

        if dl.is_ok() {
            true
        } else {
            false
        }
    }

    fn repair_game(index_url: String, res_list: String, game_path: String, is_fast: bool, progress: impl Fn(u64, u64) + Send + 'static) -> bool {
        let files = list_kuro_integrity_files(index_url, res_list);

        if files.is_some() {
            let f = files.unwrap();
            let progress = Arc::new(Mutex::new(progress));

            f.iter().for_each(|file| {
                let p = progress.clone();
                let path = Path::new(game_path.as_str());

                if is_fast {
                    let rslt= file.fast_verify(path.to_path_buf().clone());
                    if !rslt {
                        file.repair(path.to_path_buf(), move |current, total| {
                            let pl = p.lock().unwrap();
                            pl(current, total);
                        });
                    }
                } else {
                    let rslt = file.verify(path.to_path_buf().clone());
                    if !rslt {
                        file.repair(path.to_path_buf(), move |current, total| {
                            let pl = p.lock().unwrap();
                            pl(current, total);
                        });
                    }
                }
            });
            true
        } else {
            false
        }
    }
}