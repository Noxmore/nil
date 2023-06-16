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
			$($vis:vis $name:ident : $type:ty => $default:expr),* $(,)?
		}
	} =>
	{
		$(#[$attr])*
		$struct_vis struct $struct_name {
			$($vis $name: $type,)*
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

/// Could make defining many modules a bit easier, where instead of using
/// ```
/// pub mod foo;
/// pub use foo::*;
/// mod bar;
/// use bar::*;
/// ```
/// You could use
/// ```
/// use keystone::mod_use;
/// 
/// mod_use! {
/// 	pub foo;
/// 	bar;
/// }
/// ```
/// Not sure how useful this actually is, but here it is.

// Doctest is disabled because `foo` and `bar` aren't actual files, so it would break, this isn't the best, but i don't know of another solution.
#[cfg(not(doctest))]
#[macro_export]
macro_rules! mod_use {
	{$($vis:vis $name:ident ;)*} => {
		$( $vis mod $name; $vis use self::$name::*; )*
	};
}

/// Used for Results where the value isn't used, and them being [Err] shouldn't stop the process, and instead should just be printed out to console.
pub trait PrintResult: Sized
{
	/// Consumes the [Result] and if it's an [Err], prints it out.
	fn print_err(self) {
		self.print_err_msg("Error");
	}
	/// Consumes the [Result] and if it's an [Err], prints it out with the specified error message.
	fn print_err_msg(self, msg: &str);
}

impl<T, E: std::error::Error> PrintResult for Result<T, E>
{
	fn print_err_msg(self, msg: &str) {
		if let Err(err) = self { println!("{msg}: {err}") }
	}
}

/// Extension trait for `&str` that shortens the `.to_owned()` function into just `.s()` to get a [String].
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
	/// Shortened version of `.to_owned()`
	fn s(&self) -> String;
}

impl ShortToString for str
{
	fn s(&self) -> String {
		self.to_owned()
	}
}