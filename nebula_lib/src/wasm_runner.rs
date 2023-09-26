//! Example of instantiating a wasm module which uses WASI imports.

use std::time::Instant;

use anyhow::Result;
use std::error::Error;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub fn run_wasi_module(path: &str) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, path)?;

    // let instance =
    //     Instance::new(&mut store, &module, &[]).expect("There to be an instance available");
    //
    // let fibonacci = instance
    //     .get_func(&mut store, "fibonacci")
    //     .expect("Fibonacci was not an exported function");
    //
    // match instance.get_func(&mut store, "fibonacci") {
    //     Some(_) => println!("found fibonacci!"),
    //     None => println!("fibonacci not found :(!!!!!!!!!!!!!"),
    // };
    //

    linker.module(&mut store, "", &module)?;

    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    let duration = start.elapsed().as_nanos();

    println!("Done! Elapsed time: {}µs", duration);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::list_files::list_files;

    use super::*;

    #[test]
    fn it_works() {
        let files = list_files("../faas_user/modules").unwrap();
        println!("{:?}", files);
        match run_wasi_module(&files[0]) {
            Ok(_list) => assert_eq!(2, 3),
            Err(_err) => assert_eq!(1, 2),
        }
    }
}
