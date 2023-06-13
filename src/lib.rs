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

/// An easier way to make a serializable struct with defaults that doesn't lose data if new fields are added.
/// 
/// This is useful for things like settings that might change in future updates of your software.
/// 
/// # Examples
/// ```
/// use keystone::serde_defaulted_struct;
/// 
/// serde_defaulted_struct! (pub Settings, settings_defaults
/// {
/// 	pub thing: i32 => (1 => "settings_defaults::thing"),
/// 	pub foo: bool => (false => "settings_defaults::foo"),
/// });
/// 
/// let settings = Settings::default();
/// 
/// assert_eq!(settings.thing, 1);
/// assert_eq!(settings.foo, false);
/// ```
/// Unfortunately, the `"settings_defaults::thing"` is necessary due to compiler limitations.
/// 
/// `#[derive(Serialize, Deserialize)]` is added automatically, but if you want to use other derive macros, you can add `(macro1, macro2)` after your type name.
/// ```
/// use keystone::serde_defaulted_struct;
/// 
/// serde_defaulted_struct! (pub Settings(PartialEq, Clone), settings_defaults
/// {
/// 	pub thing: i32 => (1 => "settings_defaults::thing"),
/// 	pub foo: bool => (false => "settings_defaults::foo"),
/// });
/// 
/// let settings = Settings::default();
/// 
/// assert!(settings == settings.clone());
/// ```
#[macro_export]
macro_rules! serde_defaulted_struct {
	($struct_vis:vis $struct_name:ident, $defaults_vis:vis $defaults_name:ident {$($vis:vis $name:ident : $type:ty => ($default:expr => $default_path:literal)),* $(,)?}) =>
	{
		#[derive(::serde::Serialize, ::serde::Deserialize)]
		$struct_vis struct $struct_name {
			$(#[serde(default = $default_path)] $vis $name: $type,)*
		}

		$defaults_vis mod $defaults_name {
			$(pub fn $name() -> $type {$default})*
		}

		impl Default for $struct_name {
			fn default() -> Self {
				Self {
					$($name: $default,)*
				}
			}
		}
	};
	($struct_vis:vis $struct_name:ident($($macro: ident),* $(,)?), $defaults_vis:vis $defaults_name:ident {$($vis:vis $name:ident : $type:ty => ($default:expr => $default_path:literal)),* $(,)?}) =>
	{
		#[derive(::serde::Serialize, ::serde::Deserialize, $($macro,)*)]
		$struct_vis struct $struct_name {
			$(#[serde(default = $default_path)] $vis $name: $type,)*
		}

		$defaults_vis mod $defaults_name {
			$(pub fn $name() -> $type {$default})*
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
/// You could instead use
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