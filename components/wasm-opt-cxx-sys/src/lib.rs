#[cxx::bridge(namespace = "wasm_shims")]
pub mod wasm {
    unsafe extern "C++" {
        include!("shims.h");

        type Module;
        fn newModule() -> UniquePtr<Module>;
        fn validateWasm(wasm: Pin<&mut Module>) -> bool;

        type ModuleReader;
        fn newModuleReader() -> UniquePtr<ModuleReader>;
        // ... other externs ...
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

