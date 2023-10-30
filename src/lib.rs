#![doc = include_str!("../README.md")]

pub use smart_default::*;

/// Ternary operator macro to condense code a bit
/// 
/// # Examples
/// The simplest usage is:
/// ```
/// use keystone::tr;
/// 
/// assert_eq!(tr!(true, 0, 1), 0);
/// assert_eq!(tr!(false, 0, 1), 1);
/// ```
/// But if you want to do more complex stuff:
/// ```
/// use keystone::tr;
/// 
/// assert_eq!(tr!(false => 0, false => 1, true => 2, false => 3 ; 4), 2);
/// ```
#[macro_export]
macro_rules! tr {
	($($condition: expr => $a: expr $(,)?)+ ; $b: expr) => {
		$(if $condition {$a} else)+ {$b}
	};
	($condition: expr, $a: expr, $b: expr) => {
		if $condition {$a} else {$b}
	};
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
	{$($name:ident ;)*} => {
		$( pub mod $name; pub use self::$name::*; )*
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
	($path:expr, |$entry:ident| $body:block) =>
	{
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
pub trait ShortToString
{
	/// Shorthand for getting a string representation
	fn s(&self) -> String;
}

impl ShortToString for str
{
	fn s(&self) -> String {
		self.to_owned()
	}
}

impl ShortToString for std::ffi::OsStr
{
	fn s(&self) -> String {
		self.to_string_lossy().to_string()
	}
}

impl ShortToString for std::path::Path
{
	fn s(&self) -> String {
		self.to_string_lossy().to_string()
	}
}