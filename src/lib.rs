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
use std::path::PathBuf;
use stopwatch::Stopwatch;

/// Archive a file or set of files
///
/// # Arguments
///
/// * `PATH` - Path to a file or directory (required)
pub fn compress_archive(matches: &clap::ArgMatches) {
	let mut timer = Stopwatch::start_new();
	let paths: Vec<&str> = matches
		.values_of("PATH")
		.with_context(|| "No file paths were given".to_string())
		.unwrap()
		.collect();
	let output_path = matches
		.value_of("out")
		.with_context(|| "No output path was given".to_string())
		.unwrap();

	let output_pathbuf = PathBuf::from(output_path);
	let output_file_name = output_pathbuf.file_stem().unwrap().to_str().unwrap();

	let f = File::create(output_path).expect("Unable to create file");
	let compressor = lz4_flex::frame::FrameEncoder::new(f);
	let mut tar = tar::Builder::new(compressor);

	for fs_path in paths {
		let path_buf = PathBuf::from(fs_path);
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
		.expect("Unable to finish with compression");
	timer.stop();

	println!(
		"Wrote archive '{}' to filesystem (path: {}) in {} seconds.",
		output_file_name,
		output_path,
		(timer.elapsed_ms() as f32 / 1000.0)
	);
}

/// Decompress an archive
///
/// # Arguments
///
/// * `PATH` - Path to an archive (required)
pub fn extract_archive(matches: &clap::ArgMatches) {
	let mut timer = Stopwatch::start_new();
	let paths: Vec<&str> = matches
		.values_of("PATH")
		.with_context(|| "No file paths were given".to_string())
		.unwrap()
		.collect();
	let output_path = matches
		.value_of("out")
		.with_context(|| "No output path was given".to_string())
		.unwrap();

	for file_path in paths {
		let f = std::fs::File::open(file_path).expect("Could not read archive file");
		let extractor = lz4_flex::frame::FrameDecoder::new(f);
		let mut tar = tar::Archive::new(extractor);
		tar.unpack(&output_path).expect("Could not extract archive");
	}

	timer.stop();
	println!(
		"Extracted archive(s) to filesystem (path: {}) in {} seconds.",
		output_path,
		(timer.elapsed_ms() as f32 / 1000.0)
	);
}
