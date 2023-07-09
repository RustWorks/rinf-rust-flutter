#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.78.0.

use crate::bridge::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_prepare_rust_signal_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "prepare_rust_signal_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| Ok(prepare_rust_signal_stream(task_callback.stream_sink())),
    )
}
fn wire_prepare_rust_response_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "prepare_rust_response_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| Ok(prepare_rust_response_stream(task_callback.stream_sink())),
    )
}
fn wire_prepare_channels_impl() -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "prepare_channels",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(prepare_channels()),
    )
}
fn wire_start_rust_logic_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start_rust_logic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(start_rust_logic()),
    )
}
fn wire_request_to_rust_impl(
    request_unique: impl Wire2Api<RustRequestUnique> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "request_to_rust",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_request_unique = request_unique.wire2api();
            Ok(request_to_rust(api_request_unique))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<Operation> for i32 {
    fn wire2api(self) -> Operation {
        match self {
            0 => Operation::Create,
            1 => Operation::Read,
            2 => Operation::Update,
            3 => Operation::Delete,
            _ => unreachable!("Invalid variant for Operation: {}", self),
        }
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for RustResponse {
    fn into_dart(self) -> support::DartAbi {
        vec![self.successful.into_dart(), self.bytes.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RustResponse {}

impl support::IntoDart for RustResponseUnique {
    fn into_dart(self) -> support::DartAbi {
        vec![self.id.into_dart(), self.response.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RustResponseUnique {}

impl support::IntoDart for RustSignal {
    fn into_dart(self) -> support::DartAbi {
        vec![self.address.into_dart(), self.bytes.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for RustSignal {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;