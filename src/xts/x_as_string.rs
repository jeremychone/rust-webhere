///! as_string  trait/implementations
///! ----
use std::fs::DirEntry;
use std::path::PathBuf;

pub trait AsString {
	fn x_as_string(&self) -> Option<String>;
}

impl AsString for PathBuf {
	fn x_as_string(&self) -> Option<String> {
		self.to_str().map(|v| v.to_string())
	}
}

impl AsString for Option<PathBuf> {
	fn x_as_string(&self) -> Option<String> {
		match self {
			Some(path) => AsString::x_as_string(path),
			None => None,
		}
	}
}

impl AsString for DirEntry {
	fn x_as_string(&self) -> Option<String> {
		self.path().to_str().map(|s| s.to_string())
	}
}

impl AsString for Option<DirEntry> {
	fn x_as_string(&self) -> Option<String> {
		self.as_ref().and_then(|v| DirEntry::x_as_string(v))
	}
}
