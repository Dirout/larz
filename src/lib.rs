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

//! # larz
//! A simple, fast, and efficient file archiver and compressor.
//! larz creates archives in the [Unix Standard TAR](https://en.wikipedia.org/wiki/Tar_(computing)#UStar_format) format, and compresses them using [LZ4](https://en.wikipedia.org/wiki/LZ4_(compression_algorithm)).
//!
//! ## Usage
//!
//! ### Compression
//!
//! ```rust
//! use larz::compress_archive_memory;
//! use std::path::PathBuf;
//! use std::io::StdoutLock;
//!
//! let paths = vec![PathBuf::from("path/to/file"), PathBuf::from("path/to/directory")];
//! let output_path = PathBuf::from("path/to/output.larz");
//!
//! compress_archive_memory::<StdoutLock>(paths, output_path, None);
//! ```
//!
//! ### Decompression
//!
//! ```rust
//! use larz::extract_archive_memory;
//! use std::path::PathBuf;
//!
//! let paths = vec![PathBuf::from("path/to/archive.larz")];
//! let output_path = PathBuf::from("path/to/output");
//!
//! extract_archive_memory(paths, output_path);
//! ```
//!
//! ## Features
//! - `safe` - Ensures that compression and decompression are performed in a memory-safe manner. This is enabled by default.
//! - `streaming` - larz supports streaming compression and decompression using the LZ4 frame format. This means that larz can compress and decompress files with larger sizes, without having to load the entire file into memory.\
//! To enable this feature, use the `streaming` feature flag. This is enabled by default.
//!
//! ## Installation
//! Run `cargo add larz` to add larz to your `Cargo.toml` file.
//! If you intend on using `larz` as a tool, run `cargo install larz`.
//!
//! ## License
//! larz is licensed under the [GNU Affero General Public License](https://www.gnu.org/licenses/agpl-3.0.en.html).
//!
//! ## Contributing
//! Contributions are welcome! Please see [`CONTRIBUTING.md`](https://github.com/Dirout/larz/blob/master/CONTRIBUTING.md)) for more information.
//!
//! ## Authors
//! - [Emil Sayahi](https://github.com/emmyoh)
//!
//! ## Acknowledgements
//! - [`lz4_flex`](https://crates.io/crates/lz4_flex) - The LZ4 compression library used by larz.
//! - [`tar`](https://crates.io/crates/tar) - The TAR archiving library used by larz.

#![warn(missing_docs)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

/// Archive & compress a file or set of files
///
/// # Arguments
///
/// * `paths` - A list of paths pointing to files or directories intended to be archived
///
/// * `output_path` - Path to write the archive to
///
/// * `optional_logger` - An optional `BufWriter` to log information to
///
/// # Panics
///
/// This function will panic if any of the input paths are invalid or cannot be read, if the output path is invalid, or if the archive cannot be written to.
///
/// # Examples
///
/// ```rust
/// use larz::compress_archive_streaming;
/// use std::path::PathBuf;
/// use std::io::StdoutLock;
///
/// let paths = vec![PathBuf::from("path/to/file"), PathBuf::from("path/to/directory")];
/// let output_path = PathBuf::from("path/to/output.larz");
///
/// compress_archive_streaming::<StdoutLock>(paths, output_path, None);
/// ```
#[cfg(feature = "streaming")]
pub fn compress_archive_streaming<W: Write>(
	paths: Vec<PathBuf>,
	output_path: PathBuf,
	mut optional_logger: Option<&mut BufWriter<W>>,
) {
	std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();

	let f = File::create(&output_path).expect("Unable to create file");
	let buf = BufWriter::new(f);
	let compressor = lz4_flex::frame::FrameEncoder::new(buf);
	let mut tar = tar::Builder::new(compressor);

	for fs_path in paths {
		if let Some(ref mut logger) = optional_logger {
			writeln!(logger, "Compressing '{}' … ", fs_path.to_string_lossy()).unwrap();
		}
		match fs_path.is_dir() {
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
}

/// Extract & decompress an existing archive
///
/// # Arguments
///
/// * `paths` - A list of paths pointing to `larz` archives
///
/// * `output_path` - Path to write the extracted files to
///
/// # Panics
///
/// This function will panic if any of the input paths are invalid or cannot be read, or if the output path is invalid or cannot be written to.
///
/// # Examples
///
/// ```rust
/// use larz::extract_archive_streaming;
/// use std::path::PathBuf;
///
/// let paths = vec![PathBuf::from("path/to/archive.larz")];
/// let output_path = PathBuf::from("path/to/output");
///
/// extract_archive_streaming(paths, output_path);
/// ```
#[cfg(feature = "streaming")]
pub fn extract_archive_streaming(paths: Vec<PathBuf>, output_path: PathBuf) {
	std::fs::create_dir_all(&output_path).unwrap();

	for file_path in paths {
		let f = std::fs::File::open(file_path).expect("Could not read archive file");
		let buf = BufReader::new(f);
		let extractor = lz4_flex::frame::FrameDecoder::new(buf);
		let mut tar = tar::Archive::new(extractor);
		tar.unpack(&output_path).expect("Could not extract archive");
	}
}

/// Archive & compress a file or set of files, in memory
///
/// # Arguments
///
/// * `paths` - A list of paths pointing to files or directories intended to be archived
///
/// * `output_path` - Path to write the archive to
///
/// * `optional_logger` - An optional `BufWriter` to log information to
///
/// # Panics
///
/// This function will panic if any of the input paths are invalid or cannot be read, if the output path is invalid, or if the archive cannot be written to.
///
/// # Examples
///
/// ```rust
/// use larz::compress_archive_memory;
/// use std::path::PathBuf;
/// use std::io::StdoutLock;
///
/// let paths = vec![PathBuf::from("path/to/file"), PathBuf::from("path/to/directory")];
/// let output_path = PathBuf::from("path/to/output.larz");
///
/// compress_archive_memory::<StdoutLock>(paths, output_path, None);
/// ```
pub fn compress_archive_memory<W: Write>(
	paths: Vec<PathBuf>,
	output_path: PathBuf,
	mut optional_logger: Option<&mut BufWriter<W>>,
) {
	std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();

	let buf_tar: BufWriter<Vec<u8>> = BufWriter::new(Vec::new());
	let mut tar = tar::Builder::new(buf_tar);

	for fs_path in paths {
		if let Some(ref mut logger) = optional_logger {
			writeln!(logger, "Compressing '{}' … ", fs_path.to_string_lossy()).unwrap();
		}
		match fs_path.is_dir() {
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

	std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
	let f = File::create(&output_path).expect("Unable to create file");
	let mut buf = BufWriter::new(f);
	buf.write_all(&lz4_flex::block::compress_prepend_size(
		&buf_tar_again.into_inner().unwrap(),
	))
	.unwrap_or_else(|_| panic!("Could not write data to {}",
		output_path.to_string_lossy())); // Write data to file
	buf.flush().unwrap();
}

/// Extract & decompress an existing archive, in memory
///
/// # Arguments
///
/// * `paths` - A list of paths pointing to `larz` archives
///
/// * `output_path` - Path to write the extracted files to
///
/// # Panics
///
/// This function will panic if any of the input paths are invalid or cannot be read, or if the output path is invalid or cannot be written to.
///
/// # Examples
///
/// ```rust
/// use larz::extract_archive_memory;
/// use std::path::PathBuf;
///
/// let paths = vec![PathBuf::from("path/to/archive.larz")];
/// let output_path = PathBuf::from("path/to/output");
///
/// extract_archive_memory(paths, output_path);
/// ```
pub fn extract_archive_memory(paths: Vec<PathBuf>, output_path: PathBuf) {
	std::fs::create_dir_all(&output_path).unwrap();

	for file_path in paths {
		let compressed = std::fs::read(file_path).expect("Could not read archive file");
		let archive = lz4_flex::decompress_size_prepended(&compressed)
			.expect("Could not decompress archive file");
		let archive_bytes = &archive[..];
		let mut tar = tar::Archive::new(archive_bytes);
		tar.unpack(&output_path).expect("Could not extract archive");
	}
}
