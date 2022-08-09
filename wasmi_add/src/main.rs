extern crate wasmi;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

fn main() -> Result<(), Box<dyn Error>>{
    let mut buffer = Vec::new();
    {
        let f = File::open("./add.wasm");
        f?.read_to_end(&mut buffer);
    }
    let module = wasmi::Module::from_buffer(buffer)?;

    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
                    .expect("Failed to instantiate WASM module")
                    .assert_no_start();
    
    let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(42));
    args.push(RuntimeValue::from(1));

    let result: Option<RuntimeValue> = instance.invoke_export("add", &args, &mut NopExternals)?;

    match result {
        Some(RuntimeValue::I32(v)) => {
            println!("The answer to your addition was {}", v);
        }
        Some(_) => {
            println!("Failed to get from wasm invocation");
        }
        None => {
            println!("Unexpected None")
        }
    }
    Ok(())
}
