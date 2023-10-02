//! Example of instantiating a wasm module which uses WASI imports.

use std::time::Instant;

use anyhow::Result;
use std::error::Error;
use wasi_common::pipe::{ReadPipe, WritePipe};

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

#[derive(Debug)]
pub struct FunctionResult {
    pub total_elapsed_time: usize,
    pub startup_time: usize,
    pub result: String,
}

pub fn run_wasi_module(path: &str, input: &str) -> Result<FunctionResult, Box<dyn Error>> {
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

    let startup_time = start.clone().elapsed().as_millis() as usize;

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

    let result = String::from_utf8(contents)?;
    // println!("output: {:?}", result);

    let total_elapsed_time = start.elapsed().as_millis() as usize;

    println!(
        "Done! Elapsed time: {}ms, used {}ms to start up.",
        total_elapsed_time, startup_time
    );

    Ok(FunctionResult {
        total_elapsed_time,
        startup_time,
        result,
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
        match run_wasi_module(&files[0], "2") {
            Ok(_list) => assert_eq!(2, 3),
            Err(_err) => assert_eq!(1, 2),
        }
    }
}
