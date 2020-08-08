macro_rules! json_extract {
    ($obj:expr, as_string, $($props:tt)*) => {
        $obj$($props)*.as_str().map(|s| s.to_owned()).ok_or(Error::InvalidResponse {
            message: Cow::Borrowed(concat!("`", stringify!($obj$($props)*), "` is missing"))
        })?
    };
    ($obj:expr, $ty:ident, $($props:tt)*) => {
        $obj$($props)*.$ty().ok_or(Error::InvalidResponse {
            message: Cow::Borrowed(concat!("`", stringify!($obj$($props)*), "` is missing"))
        })?
    };
}
