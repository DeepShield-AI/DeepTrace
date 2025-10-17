/// [`core_read_kernel`] macro can be used to access structure fields.
/// Rust field accessors must be defined and must return Option<T>. This macro
/// returns [`Result<T, Error>`]
///
/// # Example
///
///```
/// let pid = core_read_kernel!(task_struct, pid);
///```
macro_rules! __core_read_kernel {
    ($struct:expr, $field:ident) => {
        $struct
            .$field()
    };

    ($struct:expr, $first:ident, $($rest: ident),*) => {
        $struct
            .$first()
            $(
            .and_then(|r| r.$rest())
            )*
    };
}

pub(crate) use __core_read_kernel;

#[macro_export]
macro_rules! core_read_kernel {
    ($struct:expr, $field:ident) => {
        $struct
            .$field()
            .ok_or($crate::error::code::CORE_READ_FAILED)
    };

    ($struct:expr, $first:ident, $($rest: ident),*) => {
        $struct
            .$first()
            $(
            .and_then(|r| r.$rest())
            )*
            .ok_or($crate::error::code::CORE_READ_FAILED)
    };
}

/// core_read_user macro can be used to access structure fields
#[macro_export]
macro_rules! core_read_user {
    ($struct:expr, $field:ident) => {
        paste::item!{
        $struct
            .[<$field _user>]()
        }
    };

    ($struct:expr, $first:ident, $($rest: ident),*) => {
        paste::item!{
        $struct
            .[<$first _user>]()
            $(
            .and_then(|r| r.[<$rest _user>]())
            )*
        }
    };
}
