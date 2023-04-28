//! The main library for Chickenchovy.
//!
//! The application is defined in a library, rather than directly in the binary source,
//! in part so that tests can be more easily run against it. Writing it as a library
//! also gives us the ability to easily create other binaries that run parts of it
//! here.

#![allow(clippy::nonminimal_bool)]
#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    non_ascii_idents,
    noop_method_call,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::suspicious_operation_groupings,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

use std::fmt;

use bevy as _;
use bevy_egui as _;

#[macro_use]
mod macros;

mod modules;
pub use modules::Ui;

/// The main error that is returned for this application, rather than generic Err().
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum CCError {
    /// Something is not implemented completely. Raise this error when in
    /// development/testing.
    NotImplemented(String),

    // /// Unknown error. Used for testing.
    // Unknown(String),
}

/// Returns a string representing the particular `ChuiError` variant.
impl fmt::Display for CCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CCError::NotImplemented(reason) => {
                write!(f, "Error (Not Implemented): {}.", reason)
            }

            // CCError::Unknown(reason) => {
            //     write!(f, "Error (Unknown): {}", reason)
            // }
        }
    }
}

/// The main result type that is returned in this application, rather than the
/// generic Ok().
pub type CCResult<T> = std::result::Result<T, CCError>;
