macro_rules! json_extract {
    ($obj:expr, as_string, $($props:tt)*) => {
        $obj$($props)*.as_str().map(|s| s.to_owned()).context(concat!("`", stringify!($obj$($props)*), "` is missing")).context(Error::InvalidResponse)?
    };
    ($obj:expr, $ty:ident, $($props:tt)*) => {
        $obj$($props)*.$ty().context(concat!("`", stringify!($obj$($props)*), "` is missing")).context(Error::InvalidResponse)?
    };
}
macro_rules! json_extract_optional {
    ($obj:expr, as_string, $($props:tt)*) => {
        $obj$($props)*.as_str().map(|s| s.to_owned())
    };
    ($obj:expr, $ty:ident, $($props:tt)*) => {
        $obj$($props)*.$ty()
    };
}
