macro_rules! error_exit {
    ($msg:expr) => {
        {
            error!($msg);
            ::std::process::exit(1);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        {
            error!($fmt, $($arg)*);
            ::std::process::exit(1);
        }
    };
}

macro_rules! unwrap_or_exit {
    ($val:expr, $($arg:tt)*) => {
        match $val {
            Some(value) => value,
            None => error_exit!($($arg)*)
        }
    };
}

macro_rules! import {
    ($name:ident) => {
        mod $name;
        pub use self::$name::*;
    }
}

macro_rules! create_file {
    ($file:expr) => {
        {
            let file = $file;
            match File::create(&file) {
                Ok(value) => value,
                Err(err) => {
                    error!("error regarding file {}: {}", file, err);
                    ::std::process::exit(1);
                }
            }
        }
    };
}
