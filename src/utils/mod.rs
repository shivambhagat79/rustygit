mod date_time;
mod filesystem;
mod hashing;
mod ignore;
mod index;
mod parse;
mod safety_checks;
mod status;
mod string_format;

pub use ignore::IgnoreRule;

// Re-exporting utility functions

// Date and Time Utilities
pub use date_time::format_commit_date;
pub use date_time::get_time;

// Filesystem Utilities
pub use filesystem::ensure_repo_exists;

// Hashing Utilities
pub use hashing::bytes_to_hex;
pub use hashing::hash_bytes;
pub use hashing::hex_to_bytes;

// Ignore Utilities
pub use ignore::is_ignored;
pub use ignore::parse_ignore_file;

// Parsing Utilities
pub use parse::parse_blob;
pub use parse::parse_commit;
pub use parse::parse_tree;

// Status Utilities
pub use status::get_current_commit_hash;
pub use status::get_current_tree_hash;
pub use status::get_tree_files_map;
pub use status::get_work_dir_map;

// Index Utilities
pub use index::clear_index;
pub use index::read_index_map;
pub use index::stage_index_entry;
pub use index::write_index_map;

// String Formatting Utilities
pub use string_format::format_commit_history;

// Safety Check Utilities
pub use safety_checks::checkout_safety_check;
