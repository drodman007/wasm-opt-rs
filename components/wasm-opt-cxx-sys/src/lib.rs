//! Direct bindings to `wasm-opt`.
//!
//! These are bindings to `wasm-opt`, as built by the [`wasm-opt-sys`] crate.
//! The bindings are created by the [`cxx`] crate, and all go through a custom
//! C++ shim layer that provides a `cxx`-compatible C++ API.
//!
//! Most users will not want to use this crate directly, but instead the [`wasm-opt`] crate.
//!
//! [`wasm-opt-sys`]: https://docs.rs/wasm-opt-sys
//! [`cxx`]: https://docs.rs/cxx
//! [`wasm-opt`]: https://docs.rs/wasm-opt

pub use cxx;

extern crate wasm_opt_sys;

#[cxx::bridge(namespace = "Colors")]
pub mod colors {
    unsafe extern "C++" {
        include!("shims.h");
        fn setEnabled(enabled: bool);
    }
}

#[cxx::bridge(namespace = "wasm_shims")]
pub mod wasm {
    unsafe extern "C++" {
        include!("shims.h");

        type Module;
        fn newModule() -> UniquePtr<Module>;
        fn validateWasm(wasm: Pin<&mut Module>) -> bool;

        type ModuleReader;
        fn newModuleReader() -> UniquePtr<ModuleReader>;

        type ModuleWriter;
        fn newModuleWriter() -> UniquePtr<ModuleWriter>;

        type InliningOptions;
        fn newInliningOptions() -> UniquePtr<InliningOptions>;

        type PassOptions;
        fn newPassOptions() -> UniquePtr<PassOptions>;

        type WasmFeatureSet;
        fn newFeatureSet() -> UniquePtr<WasmFeatureSet>;

        type PassRunner<'wasm>;
        fn newPassRunner<'wasm>(wasm: Pin<&'wasm mut Module>) -> UniquePtr<PassRunner<'wasm>>;
        fn newPassRunnerWithOptions<'wasm>(
            wasm: Pin<&'wasm mut Module>,
            options: UniquePtr<PassOptions>
        ) -> UniquePtr<PassRunner<'wasm>>;
    }
}

// Stub implementations outside the bridge block
#[allow(dead_code)]
pub fn newModule() -> cxx::UniquePtr<wasm::Module> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newModuleReader() -> cxx::UniquePtr<wasm::ModuleReader> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newModuleWriter() -> cxx::UniquePtr<wasm::ModuleWriter> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newInliningOptions() -> cxx::UniquePtr<wasm::InliningOptions> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newPassOptions() -> cxx::UniquePtr<wasm::PassOptions> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newFeatureSet() -> cxx::UniquePtr<wasm::WasmFeatureSet> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newPassRunner<'wasm>(
    _wasm: std::pin::Pin<&'wasm mut wasm::Module>
) -> cxx::UniquePtr<wasm::PassRunner<'wasm>> {
    unimplemented!("Patched to avoid raw_ref_op");
}

#[allow(dead_code)]
pub fn newPassRunnerWithOptions<'wasm>(
    _wasm: std::pin::Pin<&'wasm mut wasm::Module>,
    _options: cxx::UniquePtr<wasm::PassOptions>
) -> cxx::UniquePtr<wasm::PassRunner<'wasm>> {
    unimplemented!("Patched to avoid raw_ref_op");
}
