mod date_time;
mod filesystem;
mod hashing;
mod ignore;
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

// String Formatting Utilities
pub use string_format::format_commit_history;

// Safety Check Utilities
pub use safety_checks::checkout_safety_check;
