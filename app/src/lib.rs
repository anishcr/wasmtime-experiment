#[link(wasm_import_module = "lunatic::process")]
extern "C" {
    pub fn exec(cmd: *const u8, cmd_len: usize, args: *const u8, args_len: usize) -> i32;
}

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        let mut cmd = "echo";
        let mut args = "hello";
        exec(
            cmd.as_bytes().as_ptr(),
            cmd.len(),
            args.as_bytes().as_ptr(),
            args.len(),
        );

        cmd = "ls";
        args = "-l";
        exec(
            cmd.as_bytes().as_ptr(),
            cmd.len(),
            args.as_bytes().as_ptr(),
            args.len(),
        );
        cmd = "ls";
        args = "";
        exec(
            cmd.as_bytes().as_ptr(),
            cmd.len(),
            args.as_bytes().as_ptr(),
            args.len(),
        );
        cmd = "lsldtr";
        args = "-l";
        exec(
            cmd.as_bytes().as_ptr(),
            cmd.len(),
            args.as_bytes().as_ptr(),
            args.len(),
        );
    }
}
