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
        #[macro_use] mod $name;
        pub use self::$name::*;
    }
}

macro_rules! create_file {
    ($file:expr) => {
        {
            use std::io::BufWriter;
            use std::fs::File;

            let file = $file;
            let file = match File::create(&file) {
                Ok(value) => value,
                Err(err) => {
                    error!("error regarding file {:?}: {}", file, err);
                    ::std::process::exit(1);
                }
            };

            BufWriter::new(file)
        }
    };
}

macro_rules! clamp_error {
    ($expr:expr, $($arg:tt)*) => {
        {
            match $expr {
                Ok(value) => value,
                Err(err) => error_exit!($($arg)*)
            }
        }
    };
}
