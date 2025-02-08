use clap::Parser;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use component::greet::greetable::Host;

struct Greet {
    name: String,
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

impl Greet {
    fn new(name: String) -> Greet {
        let wasi_ctx = WasiCtxBuilder::new().build();
        let resource_table = ResourceTable::new();
        Greet {
            name,
            wasi_ctx,
            resource_table,
        }
    }
}

impl WasiView for Greet {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl Host for Greet {
    fn name(&mut self) -> String {
        self.name.clone()
    }

    fn greet(&mut self, name: String) -> String {
        format!("Hello from {name}")
    }
}

bindgen!({
    path:"/Users/tt/Desktop/greet/wit",
    world:"hello-world",
});

#[derive(Parser, Debug)]
struct Cli {
    wasm_file: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = start(cli) {
        print!("{e}")
    }
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    let mut store = Store::new(&engine, Greet::new("Native code".to_string()));

    let component = Component::from_file(&engine, &cli.wasm_file)?;

    HelloWorld::add_to_linker(&mut linker, |greet: &mut Greet| greet)?;

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let hello_world = HelloWorld::instantiate(&mut store, &component, &linker)?;

    let message = hello_world.component_greet_sayable().call_say(&mut store)?;
    print!("{message}");

    Ok(())
}
