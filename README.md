# Support "exec" command in a wasm module

This is a very amateur experiment to use the wasmtime webassembly runtime to support "exec" api to be called from wasm files. This is a first step in exploring how to implement the feature reported in https://github.com/lunatic-solutions/lunatic/issues/149

The idea is that we should be able to call and execute an arbitary application from webassembly module and return the return code of the execution.

There are two packages in this repo. 
- The runtime package, which is a rust binary package which includes the wasmtime and supports the host api "exec"
- The app package, which is a rust webassembly library package, which creates a wasm file to be consumed by the above runtime.
 
## The app package
It exports one function "run" which will be called from the runtime. In turn it expects the runtime to implement the following function

```
pub fn exec(cmd: *const u8, cmd_len: usize, args: *const u8, args_len: usize) -> i32;
```

The run() function calls the exec() function passing different commands and their arguments if any.

#### How to build the app package
- cd to the app directory and execute the below command
- ```$>cargo wasi build```
- copy the created wasmfile to the runtime folder
- ```cp target/wasm32-wasi/debug/app.wasm ../runtime/``` as the runtime app expects the file app.wasm in its root folder.
- 

## The runtime package
It contains one binary crate and the implementation of the exec function to be supported in the wasmtime runtime.

### How to build and run the runtime package
- cd to the runtime directory and execute the below command (before that ensure that the wasm file from app package is copied to the runtime folder)
- ```$>cargo run```
- 

### Make file
You can use the make command to do all the build and run the runtime binary.

