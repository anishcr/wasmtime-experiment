use std::error::Error;
use wasmtime::*;

pub fn get_memory<T>(caller: &mut wasmtime::Caller<T>) -> Option<wasmtime::Memory> {
    caller.get_export("memory").unwrap().into_memory()
}

fn get_string_arg<T>(
    caller: &mut Caller<T>,
    name_str_ptr: u32,
    name_str_len: u32,
) -> Option<String> {
    if name_str_len == 0 {
        return Some("".to_string());
    }

    let memory = match get_memory(caller) {
        Some(mem) => mem,
        None => return None,
    };

    let memory_slice = memory.data(caller);

    let name = match memory_slice.get(name_str_ptr as usize..(name_str_ptr + name_str_len) as usize)
    {
        Some(x) => x,
        None => return None,
    };

    let name = String::from_utf8(name.to_vec());

    Some(name.unwrap())
}

fn exec<T>(
    mut caller: Caller<'_, T>,
    cmd_str_ptr: u32,
    cmd_str_len: u32,
    args_str_ptr: u32,
    args_str_len: u32,
) -> i32 {
    let cmd_name = match get_string_arg(&mut caller, cmd_str_ptr, cmd_str_len) {
        Some(n) => n,
        _ => return i32::MIN,
    };

    let args_name = match get_string_arg(&mut caller, args_str_ptr, args_str_len) {
        Some(n) => n,
        _ => return i32::MIN,
    };

    println!("--------------------------------------------------------");
    println!("exec called with : cmd : {cmd_name} and args : {args_name}");

    let x = if args_name.eq("") {
        std::process::Command::new(cmd_name).status()
    } else {
        std::process::Command::new(cmd_name).arg(args_name).status()
    };

    let result = match x {
        Ok(val) => val.code().unwrap_or(i32::MIN),
        _ => i32::MIN,
    };

    println!("exec returning value : {result}");

    result
}

struct Log {}

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "app.wasm")?;

    let mut linker = Linker::new(&engine);

    linker.func_wrap("lunatic::process", "exec", exec)?;

    let data = Log {};
    let mut store = Store::new(&engine, data);
    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;

    run.call(&mut store, ())?;

    Ok(())
}
