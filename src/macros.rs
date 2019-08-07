/// Create an error with a formatted message
#[macro_export]
macro_rules! err {
    ($msg:literal $(,)?) => {
        $crate::Error::msg($msg)
    };
    ($fmt:literal, $($arg:tt)*) => {
        $crate::Error::msg(format!($fmt, $($arg)*))
    };
}

/// Ensure a condition is true, or return an error
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $msg:literal $(,)?) => {
        if !($cond) {
            return Err($crate::err!($msg));
        }
    };
    ($cond:expr, $fmt:literal, $($arg:tt)*) => {
        if !($cond) {
            return Err($crate::err!($fmt, $($arg)*));
        }
    };
    ($cond:expr, $error:expr $(,)?) => {
        if !($cond) {
            return Err($crate::Error::from($error));
        }
    };
}

/// Return early with an error
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::err!($msg));
    };
    ($fmt:literal, $($arg:tt)*) => {
        return Err($crate::err!($fmt, $($arg)*));
    };
    ($error:expr $(,)?) => {
        return Err($crate::Error::from($error));
    };
}
