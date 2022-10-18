//! # Miette Helper
//! Simple crate providing helper macros for miette in libraries.
//!
//! You will get "random" compiler errors, since I decided against checking the type to ensure compatibility with type aliases.
//! please make sure the error isn't mentioned in the macro doc before reporting.
#[warn(clippy::all)]
#[warn(missing_docs)]
#[warn(rustdoc::all)]
#[macro_export]
/// Turns Error into an Diagnostic and wraps it with the specified error
///
/// ## Arguments
/// 1. Result (Will cause "trait bound not satified" if wrong type)
/// 2. Optional: Message to wrap it with. (format! syntax allowed).
macro_rules! wrap_err {
    ($res: expr, $($msg: tt)*) => (
            ::miette::Context::wrap_err(
                ::miette::IntoDiagnostic::into_diagnostic($res),
                format!($($msg)*)
            )
    );
}

#[macro_export]
/// Unwraps error or prints the error in [error][log::error]
/// You will have to import the error macro yourself (To ensure you can use a re-export)
///
/// Returns a Option wielding `Some(T)` when `Ok(T)` and `None` when `Err(E)`
///
/// ## Arguments
/// 1. Result (Will cause "trait bound not satified" compiler error if wrong type)
/// 2. macro used to print text (like `println`, `log::error`, etc)
/// 3. Optional: Message to wrap it with. (format! syntax allowed).
macro_rules! unwrap_or_print {
    ($res: expr, $printfn: ident, $($msg: tt)*) => {
        match $res {
            Ok(r) => Some(r),
            Err(e) => {
                $printfn!("{}: {e}", $($msg)+);
                None
            }
        }
    };
    ($res: expr, $printfn: ident) => {
        match $res {
            Ok(r) => Some(r),
            Err(e) => {
                $printfn!("{e}");
                None
            }
        }
    };
}

#[macro_export]
/// Unwraps error or prints the error in [error][log::error]
/// You will have to import the error macro yourself (To ensure you can use a re-export)
///
/// Returns a Option wielding `Some(T)` when `Ok(T)` and `None` when `Err(E)`
///
/// ## Arguments
/// 1. Option (Will cause "trait bound not satified" compiler error if wrong type)
/// 2. macro used to print text (like `println`, `log::error`)
/// 3. Optional: Message to wrap it with. (format! syntax allowed).
macro_rules! or_print {
    ($res: expr, $printfn: ident, $($msg: tt)*) => {
        match $res {
            o => o,
            None(e) => {
                $printfn!("{}: {e}", $($msg)+);
                None
            }
        }
    };
    ($res: expr, $printfn: ident) => {
        match $res {
            Ok(r) => Some(r),
            Err(e) => {
                $printfn!("{e}");
                None
            }
        }
    };
}
#[macro_export]
/// Turn an option into an Result with the given error message.
/// If you get an error saying trait bound not satified make sure the first argument is an Option.
/// ## Arguments
/// 1. Option (Will cause "trait bound not satified" compiler error if wrong type)
/// 2. Optional: Message to wrap it with. (format! syntax allowed).
macro_rules! or_wrap_err {
    ($opt: expr, $($msg: tt)*) => (
        $opt.ok_or_else(|| miette::miette!($($msg)*))
    );
}
