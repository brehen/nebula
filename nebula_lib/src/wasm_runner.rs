//! Example of instantiating a wasm module which uses WASI imports.

use std::{path::PathBuf, time::Instant};

use anyhow::Result;
use std::error::Error;
use wasi_common::pipe::{ReadPipe, WritePipe};

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

use crate::models::{FunctionResult, Metrics};

pub fn run_wasi_module(path: &PathBuf, input: &str) -> Result<FunctionResult, Box<dyn Error>> {
    let start = Instant::now();
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let stdin = ReadPipe::from(input);
    let stdout = WritePipe::new_in_memory();

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(stdin.clone()))
        .stdout(Box::new(stdout.clone()))
        .build();

    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, path)?;

    let startup_time = start.clone().elapsed().as_millis();

    linker
        .module(&mut store, "", &module)
        .expect("the function to be linked");

    linker
        .get_default(&mut store, "")
        .expect("Should get the wasi runtime")
        .typed::<(), ()>(&store)
        .expect("should type the function")
        .call(&mut store, ())
        .expect("should call the function");

    drop(store);

    let contents: Vec<u8> = stdout
        .try_into_inner()
        .map_err(|_err| anyhow::Error::msg("sole remaining reference"))?
        .into_inner();

    let result = String::from_utf8(contents)?.trim().to_string();

    let total_runtime = start.elapsed().as_millis();

    println!(
        "Done! Elapsed time: {}ms, used {}ms to start up.",
        total_runtime, startup_time
    );

    Ok(FunctionResult {
        result,
        metrics: Some(Metrics {
            total_runtime,
            startup_time,
        }),
    })
}

#[cfg(test)]
mod tests {
    use crate::list_files::list_files;

    use super::*;

    #[test]
    fn it_works() {
        let files = list_files("../faas_user/modules").unwrap();
        println!("{:?}", files);
        match run_wasi_module(files.get(0).expect("to exist"), "2") {
            Ok(_list) => assert_eq!(2, 3),
            Err(_err) => assert_eq!(1, 2),
        }
    }
}
