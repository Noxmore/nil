#![doc = include_str!("../README.md")]

pub use smart_default::*;
pub use once_cell;
pub use once_cell::sync::Lazy;
pub use parking_lot;
pub use parking_lot::{Mutex, MutexGuard, MappedMutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, MappedRwLockReadGuard};

/// Extra std imports that i use a lot.
pub mod std_prelude {
	pub use std::path::{Path, PathBuf};
	pub use std::fs;
	pub use std::io;
	pub use std::thread;
	pub use std::collections::{HashMap, BTreeMap};
	pub use std::sync::Arc;
	pub use std::sync::atomic::*;
	pub use std::error::Error;
	pub use std::borrow::Cow;
	pub use std::time::{Duration, Instant};
	pub use std::mem;
	pub use std::fmt;
	pub use std::env;
	pub use std::process;
}

/// Makes defining a flat module (e.g. foo::Baz instead of foo::bar::Baz) easier.
///
/// Instead of
/// ```ignore
/// pub mod foo;
/// pub use foo::*;
/// pub mod bar;
/// pub use bar::*;
/// ```
/// You could use
/// ```ignore
/// use keystone::*;
///
/// flat! {
/// 	foo;
/// 	bar;
/// }
/// ```
#[macro_export]
macro_rules! flat {
	{$($(#[$attr:meta])* $name:ident ;)*} => {
		$( $(#[$attr])* pub mod $name; $(#[$attr])* pub use self::$name::*; )*
	};
}

/// Reads a directory without having to do tons of error-checking boilerplate.
/// 
/// Only executes if everything goes well
/// 
/// # Examples
/// ```ignore
/// use keystone::*;
/// 
/// read_dir!(path, |entry|
/// {
///		// (Do something with entry)
/// });
/// ```
#[macro_export]
macro_rules! read_dir {
	($path:expr, |$entry:ident| $body:block) => {
		if let Ok(read_dir) = $path.read_dir() {
			for entry in read_dir {
				if let Ok($entry) = entry
					$body
			}
		}
	};
}

/// Extension trait that shortens `.to_owned()` or `.to_string_lossy().to_string()` into just `.s()` to get a [String].
/// 
/// # Examples
/// ```
/// use keystone::*;
/// 
/// let string: String = "foo".s();
/// let owned_str: String = "foo".to_owned();
/// 
/// assert_eq!(string, owned_str);
/// ```
pub trait ShortToString {
	/// Shorthand for getting a string representation
	fn s(&self) -> String;
}

impl ShortToString for str {
	fn s(&self) -> String {
		self.to_owned()
	}
}

impl ShortToString for std::ffi::OsStr {
	fn s(&self) -> String {
		self.to_string_lossy().to_string()
	}
}

impl ShortToString for std::path::Path {
	fn s(&self) -> String {
		self.to_string_lossy().to_string()
	}
}

impl ShortToString for std::ffi::CStr {
	fn s(&self) -> String {
		self.to_string_lossy().to_string()
	}
}