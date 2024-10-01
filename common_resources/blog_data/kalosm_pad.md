<img src="file:///media/uberdev/ddrv/gitFolders/cratesploring/common_resources/blog_data/RustLang_tutorials01.png" title="" alt="kalosm" width="540">

### Objective

Load Llama2 model on Cuda enabled GPU, and do the inference using Kalosm Crate

### Demo On the Terminal

![image](/media/uberdev/ddrv/gitFolders/cratesploring/common_resources/blog_pics/redo-right.png)

### Wait a Minute.. Model is loading...

##### Repo & Crates References

- [floneum/interfaces/kalosm at main · floneum/floneum · GitHub](https://github.com/floneum/floneum/tree/main/interfaces/kalosm)

### Pre-Reqs

- Ubuntu Linux with Cuda Libraries & Nvidia Drivers
  
  - Refer [how to setup here](https://medium.com/@kamaljp/list/rust-in-linux-with-cuda-llm-703a8526d1fe)

- Following Linux libraries to be installed
  
  ```shell
  sudo apt install libclang-dev
  * export LIBCLANG_PATH=/usr/lib/llvm-<version>/lib
  
  sudo apt-get install librocksdb-dev
  * export ROCKSDB_LIB_DIR=/path/to/rocksdb/lib
  * export ROCKSDB_INCLUDE_DIR=/path/to/rocksdb/include
  ```

- Update the Cargo.toml file with kalosm with "full" and "cuda" features. Refer the [toml file here](https://github.com/Kamalabot/cratesploring/tree/main/floneum_explorer/next-gen-ai)

### Code WalkThrough

1) Imports & Flags

2) Setting up Model

3) Taking User input

4) Generating to Standard Output

### Build Compilation

1. cargo clean 

2. cargo build --bin next-gen-ai
   
   - Creates the binary in /target/debug

### Execution

1. cargo run --bin ../target/debug/next-gen-ai

```mermaid
flowchart TD
a[user execute binary file]
b[binary file loads model in GPU]
c[prompt fed to model]
d[inference output]
a -->b-->c-->d
```
