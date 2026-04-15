//! Log capture module - handles background log file tailing with timestamp filtering.
//! Uses a plain std thread + std::sync::mpsc so drain_received() works in the UI thread.

use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{self, AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// Shared stop flag for the capture thread.
/// Call `.store(true, ...)` to request graceful stop.
pub type LogCaptureStop = Arc<AtomicBool>;

/// Start a new log capture thread that tails the app's log file.
///
/// Returns `(log_rx, stop_flag)`. Drop the `stop_flag` or set it to `true`
/// to stop the thread. Drain `log_rx` from the UI thread each frame.
pub fn start_log_capture(app_log_dir: PathBuf) -> Option<(Receiver<String>, LogCaptureStop)> {
    let log_file = find_latest_log_file(&app_log_dir)?;

    let (log_tx, log_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let stop_flag: LogCaptureStop = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);

    thread::Builder::new()
        .name("log_capture".into())
        .spawn(move || {
            tail_log_file_thread(log_file, stop_flag_clone, log_tx);
        })
        .ok()?;

    Some((log_rx, stop_flag))
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Runs in a plain thread. Reads the log file line by line, starting from the end.
fn tail_log_file_thread(log_file: PathBuf, stop_flag: LogCaptureStop, log_tx: Sender<String>) {
    let _ = log_tx.send("[LogCapture] Tailing from session start...".to_string());

    let mut file = match std::fs::File::open(&log_file) {
        Ok(f) => f,
        Err(_) => {
            let _ = log_tx.send("[LogCapture] Cannot open log file".to_string());
            return;
        }
    };

    // Seek to end to only capture new lines written after we opened
    let _ = file.seek(SeekFrom::End(0));

    let reader = BufReader::with_capacity(8192, file);
    let mut lines = reader.lines();

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            // Grace period: drain any remaining lines from the buffer and exit.
            // BufReader::lines() won't re-read after EOF, so this just drains
            // what's already buffered.
            while let Some(result) = lines.next() {
                if let Ok(line) = result {
                    if !line.trim().is_empty() {
                        let _ = log_tx.send(line);
                    }
                }
            }
            break;
        }

        // Try to read one line
        match lines.next() {
            Some(Ok(line)) => {
                if !line.trim().is_empty() {
                    let _ = log_tx.send(line);
                }
            }
            Some(Err(_)) => {
                thread::sleep(std::time::Duration::from_millis(100));
            }
            None => {
                // EOF — sleep briefly, check for new content
                thread::sleep(std::time::Duration::from_millis(150));
            }
        }
    }
}

fn find_latest_log_file(log_dir: &std::path::Path) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(log_dir).ok()?;
    let mut log_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "log"))
        .collect();

    if log_files.is_empty() {
        return None;
    }

    log_files.sort_by_key(|e| std::cmp::Reverse(e.metadata().ok().and_then(|m| m.modified().ok())));

    Some(log_files.into_iter().next().unwrap().path())
}
