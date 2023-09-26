use std::time::Instant;

use wasmtime::*;

use super::wasm_module::FunctionResult;

struct MyState {
    name: String,
    count: usize,
}

pub fn run_modules(file: String) -> FunctionResult {
    let start = Instant::now();

    // First the wasm module needs to be compiled. This is done with a global
    // "compilation environment" within an `Engine`. Note that engines can be
    // further configured through `Config` if desired instead of using the
    // default like this is here.
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, &file).expect("To find file");

    let mut imports = Vec::new();

    for import in module.imports() {
        match import.ty() {
            ExternType::Func(func_ty) => {
                imports.push(func_ty);
            }
            // Handle other ExternTypes like Memories, Globals, and Tables similarly
            _ => unimplemented!(),
        }
    }

    println!("{:?}", imports);

    // After a module is compiled we create a `Store` which will contain
    // instantiated modules and other items like host functions. A Store
    // contains an arbitrary piece of host information, and we use `MyState`
    // here.
    println!("Initializing...");
    let mut store = Store::new(
        &engine,
        MyState {
            name: "hello, worlds!".to_string(),
            count: 0,
        },
    );

    // Our wasm module we'll be instantiating requires one imported function.
    // the function takes no parameters and returns no results. We create a host
    // implementation of that function here, and the `caller` parameter here is
    // used to get access to our original `MyState` value.
    println!("Creating callback...");
    let the_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
        println!("Calling back...");
        println!("> {}", caller.data().name);
        caller.data_mut().count += 1;
    });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    let imports = [the_func.into()];
    let instance = Instance::new(&mut store, &module, &imports).expect("instance to be spawned");

    // Next we poke around a bit to extract the `run` function from the module.
    println!("Extracting export...");
    let run = instance
        .get_typed_func::<(), ()>(&mut store, "run")
        .expect("run to work");

    // And last but not least we can call it!
    println!("Calling export...");
    run.call(&mut store, ()).expect("export to work");

    //    println!("Result: {:?}", result);

    let duration = start.elapsed().as_micros();
    println!("Done. time: {:?}", duration);

    FunctionResult {
        function_name: file,
        total_runtime: duration,
    }
}
