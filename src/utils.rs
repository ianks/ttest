use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Test if the pattern is in a Gemfile
pub fn is_in_file(file: &str, name: &str) -> bool {
    let full_path = current_dir().unwrap().join(file);

    if !full_path.exists() {
        return false;
    }

    let reader = BufReader::new(File::open(full_path).unwrap());

    reader
        .lines()
        .any(|line| line.unwrap_or_default().contains(name))
}

/// Check to see if the given file exists, relative to the current directory
pub fn file_exists<T: Into<PathBuf>>(path: T) -> bool {
    current_dir()
        .map(|c| c.join(path.into()).exists())
        .unwrap_or(false)
}

#[macro_export]
#[cfg(not(test))]
macro_rules! memoize_string {
    ($val:expr) => {{
        static INIT: std::sync::Once = std::sync::Once::new();
        static mut VALUE: Option<String> = None;
        unsafe {
            INIT.call_once(|| {
                let s: String = $val.into();
                VALUE = Some(s);
            });
            VALUE.as_ref().unwrap().into()
        }
    }};
}

#[macro_export]
#[cfg(test)]
macro_rules! memoize_string {
    ($val:expr) => {{
        $val.into()
    }};
}

#[macro_export]
macro_rules! none_if_empty {
    ($val:expr) => {{
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    }};
}

#[macro_export]
macro_rules! define_adapters {
    ($($key:expr => $value:ident),*) => {
        impl TryFrom<&str> for $crate::adapter::Adapter {
            type Error = Box<dyn std::error::Error>;
            /// Fetch an adapter for the given name
            fn try_from(name: &str) -> Result<Self, Self::Error> {
                match name {
                    $($key => Ok($crate::adapter::Adapter::$value($value {})),)*
                    _ => Err("Unknown adapter".into()),
                }
            }
        }

        #[enum_dispatch]
        #[derive(Debug, PartialEq)]
        pub enum Adapter { $($value($value),)* }

        /// Get a list of all adapters.
        pub const fn all() -> &'static [$crate::adapter::Adapter] {
            &[$($crate::adapter::Adapter::$value($value {})),*]
        }
    };
}
