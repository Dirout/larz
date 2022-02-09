use std::fs;
/*
	This file is part of larz.
	larz is free software: you can redistribute it and/or modify
	it under the terms of the GNU Affero General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.
	larz is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU Affero General Public License for more details.
	You should have received a copy of the GNU Affero General Public License
	along with larz.  If not, see <https://www.gnu.org/licenses/>.
*/
use anyhow::Context;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use stopwatch::Stopwatch;

/// Archive a file or set of files
///
/// # Arguments
///
/// * `PATH` - Path to a file or directory (required)
pub fn compress_archive_streaming(paths: Vec<String>, output_path: String) {
	let stdout = std::io::stdout();
	let lock = stdout.lock();
	let mut buf_out = BufWriter::new(lock);

	let mut timer = Stopwatch::start_new();

	let output_pathbuf = PathBuf::from(&output_path);
	let output_file_name = output_pathbuf.file_stem().unwrap().to_str().unwrap();

	let f = File::create(&output_path).expect("Unable to create file");
	let buf = BufWriter::new(f);
	let compressor = lz4_flex::frame::FrameEncoder::new(buf);
	let mut tar = tar::Builder::new(compressor);

	for fs_path in paths {
		writeln!(buf_out, "Compressing {} … ", fs_path);
		let path_buf = PathBuf::from(&fs_path);
		match path_buf.is_dir() {
			true => {
				tar.append_dir_all(".", fs_path)
					.expect("Failed to write to archive");
			}
			false => {
				tar.append_path(fs_path)
					.expect("Failed to write to archive");
			}
		}
	}

	let tar_compressor = tar.into_inner().expect("Unable to finish writing archive");
	tar_compressor
		.finish()
		.expect("Unable to finish with compression")
		.flush()
		.unwrap();
	timer.stop();

	writeln!(
		buf_out,
		"\nWrote archive '{}' to filesystem (path: {}) in {} seconds.",
		output_file_name,
		output_path,
		(timer.elapsed_ms() as f32 / 1000.0)
	);
	buf_out.flush().unwrap();
}

/// Decompress an archive
///
/// # Arguments
///
/// * `PATH` - Path to an archive (required)
pub fn extract_archive_streaming(paths: Vec<String>, output_path: String) {
	let stdout = std::io::stdout();
	let lock = stdout.lock();
	let mut buf_out = BufWriter::new(lock);
	let mut timer = Stopwatch::start_new();

	for file_path in paths {
		let f = std::fs::File::open(file_path).expect("Could not read archive file");
		let buf = BufReader::new(f);
		let extractor = lz4_flex::frame::FrameDecoder::new(buf);
		let mut tar = tar::Archive::new(extractor);
		tar.unpack(&output_path).expect("Could not extract archive");
	}

	timer.stop();
	writeln!(
		buf_out,
		"\nExtracted archive(s) to filesystem (path: {}) in {} seconds.",
		output_path,
		(timer.elapsed_ms() as f32 / 1000.0)
	);
	buf_out.flush().unwrap();
}

/// Archive a file or set of files using only memory
///
/// # Arguments
///
/// * `PATH` - Path to a file or directory (required)
pub fn compress_archive_memory(paths: Vec<String>, output_path: String) {
	let stdout = std::io::stdout();
	let lock = stdout.lock();
	let mut buf_out = BufWriter::new(lock);
	let mut timer = Stopwatch::start_new();

	let output_pathbuf = PathBuf::from(&output_path);
	let output_file_name = output_pathbuf.file_stem().unwrap().to_str().unwrap();

	let buf_tar: BufWriter<Vec<u8>> = BufWriter::new(Vec::new());
	let mut tar = tar::Builder::new(buf_tar);

	for fs_path in paths {
		writeln!(buf_out, "Compressing {} … ", fs_path);
		let path_buf = PathBuf::from(&fs_path);
		match path_buf.is_dir() {
			true => {
				tar.append_dir_all(".", fs_path)
					.expect("Failed to write to archive");
			}
			false => {
				tar.append_path(fs_path)
					.expect("Failed to write to archive");
			}
		}
	}

	tar.finish().expect("Unable to finish with compression");

	let mut buf_tar_again = tar.into_inner().unwrap();
	buf_tar_again.flush().unwrap();

	fs::create_dir_all(Path::new(&output_path).parent().unwrap()).unwrap();
	let f = File::create(&output_path).expect("Unable to create file");
	let mut buf = BufWriter::new(f);
	buf.write_all(&lz4_flex::block::compress_prepend_size(
		&buf_tar_again.into_inner().unwrap(),
	))
	.with_context(|| format!("Could not write data to {}", output_path))
	.unwrap(); // Write data to file
	buf.flush().unwrap();

	timer.stop();

	writeln!(
		buf_out,
		"Wrote archive '{}' to filesystem (path: {}) in {} seconds.",
		output_file_name,
		output_path,
		(timer.elapsed_ms() as f32 / 1000.0)
	);
	buf_out.flush().unwrap();
}

/// Decompress an archive created using only memory
///
/// # Arguments
///
/// * `PATH` - Path to an archive (required)
pub fn extract_archive_memory(paths: Vec<String>, output_path: String) {
	let stdout = std::io::stdout();
	let lock = stdout.lock();
	let mut buf_out = BufWriter::new(lock);
	let mut timer = Stopwatch::start_new();

	for file_path in paths {
		let compressed = fs::read(file_path).expect("Could not read archive file");
		let archive = lz4_flex::decompress_size_prepended(&compressed)
			.expect("Could not decompress archive file");
		let archive_bytes = &archive[..];
		let mut tar = tar::Archive::new(archive_bytes);
		tar.unpack(&output_path).expect("Could not extract archive");
	}

	timer.stop();
	writeln!(
		buf_out,
		"Extracted archive(s) to filesystem (path: {}) in {} seconds.",
		output_path,
		(timer.elapsed_ms() as f32 / 1000.0)
	);
	buf_out.flush().unwrap();
}
