use chrono::Local;
use std::path::{Path, PathBuf};

pub(super) fn format_filename(path: &PathBuf) -> PathBuf {
	let dir = path.parent().unwrap_or_else(|| Path::new(""));
	let file_stem = path.file_stem().unwrap_or_default().to_str().unwrap();
	let extension = path.extension().map(|e| e.to_str().unwrap()).unwrap_or("txt");

	let timestamp = Local::now().format("%Y%m%d%H%M%S");
	let filename = if extension.is_empty() {
		format!("{file_stem}_{timestamp}")
	} else {
		format!("{file_stem}_{timestamp}.{extension}")
	};
	dir.join(filename)
}
