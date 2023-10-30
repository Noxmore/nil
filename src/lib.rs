#![doc = include_str!("../README.md")]

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

/// Creates a `struct` with defaults values specified next to fields to remove boilerplate.
/// 
/// I mostly use this with `#[serde(default)]` to make things like settings that won't break if the struct is
/// changed in a newer version of my application.
/// 
/// # Examples
/// ```
/// use keystone::*;
/// 
/// defaulted_struct!
/// {
/// 	/// Doc comments work here, as well as attributes.
/// 	#[derive(Clone, Debug, PartialEq, Eq)]
/// 	#[serde(default)]
/// 	pub struct Settings
/// 	{
/// 		pub thing: i32 => 1,
/// 		pub foo: bool => false,
/// 	}
/// }
/// 
/// let settings = Settings::default();
/// 
/// assert_eq!(settings.thing, 1);
/// assert_eq!(settings.foo, false);
/// assert_eq!(settings, settings.clone());
#[macro_export]
macro_rules! defaulted_struct {
	{
		$(#[$attr:meta])*
		$struct_vis:vis struct $struct_name:ident
		{
			$($(#[$field_attr:meta])* $vis:vis $name:ident : $type:ty => $default:expr),* $(,)?
		}
	} =>
	{
		$(#[$attr])*
		$struct_vis struct $struct_name {
			$($(#[$field_attr])* $vis $name: $type,)*
		}

		impl Default for $struct_name {
			fn default() -> Self {
				Self {
					$($name: $default,)*
				}
			}
		}
	};
}

/// Makes defining a flat module (e.g. foo::Baz instead of foo::bar::Baz) easier.
///
/// Instead of
/// ```ignore
/// pub mod foo;
/// pub use foo::*;
/// mod bar;
/// use bar::*;
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
		$( pub mod $name; $vis use self::$name::*; )*
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