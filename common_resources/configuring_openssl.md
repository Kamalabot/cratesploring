- **Step – 1 : Downloading OpenSSL:**
  
  Run the command as below :
  
  `$ wget http://www.openssl.org/source/openssl-1.0.1g.tar.gz (is old version)`
  
  https://github.com/openssl/openssl/releases/download/openssl-3.3.2/openssl-3.3.2.tar.gz **Step – 2 : Extract files from the downloaded package:**
  
  `$ tar -xvzf openssl-3.3.2.tar.gz`
  
  Now, enter the directory where the package is extracted like here is openssl-1.0.1g
  
  `$ cd openssl-3.3.2`
  
  **Step – 3 : Configuration OpenSSL**
  
  Run below command with optional condition to set prefix and directory where you want to copy files and folder.
  
  `$ ./config --prefix=/usr/lib/ssl --openssldir=/usr/lib/ssl
  
  **Step – 4 : Compiling OpenSSL**
  
  To compile openssl you will need to run 2 command : make, make install as below :
  
  `$ make`
  
  Note: check for any error message for verification purpose.
  
  **Step -5 : Installing OpenSSL:**
  
  `$ sudo make install`
  
  Or without sudo,
  
  `$ make install`
  
  `$ /usr/local/openssl/bin/openssl version`

Did install, however needed some additional updates. refer below

updated the .bashrc with the necessary path so the cargo builds can use the openssl libraries

When you install the `reqwest` crate in Rust, it may require OpenSSL for certain features like HTTPS support. During the installation process, the Rust compiler looks for the OpenSSL libraries (`libssl` and `libcrypto`). If it can't find the libraries, you need to tell Rust where OpenSSL is installed by setting some environment variables that point to the correct paths.

### Key Environment Variables for OpenSSL:

1. **`OPENSSL_DIR`:**  
   This is the directory where OpenSSL is installed. It should contain the `lib` and `include` directories for OpenSSL. This is typically `/usr/local/openssl` or `/usr/lib/openssl` if you installed OpenSSL in a non-standard location.
   
   Example:
   
   ```bash
   export OPENSSL_DIR=/usr/local/openssl
   ```

2. **`OPENSSL_LIB_DIR`:**  
   This variable points to the directory containing the OpenSSL libraries (`libssl.so` and `libcrypto.so`). Typically, it's the `lib` folder inside `OPENSSL_DIR`.
   
   Example:
   
   ```bash
   export OPENSSL_LIB_DIR=/usr/local/openssl/lib
   ```

3. **`OPENSSL_INCLUDE_DIR`:**  
   This variable points to the OpenSSL `include` directory, which contains the OpenSSL headers (like `openssl/ssl.h`).
   
   Example:
   
   ```bash
   export OPENSSL_INCLUDE_DIR=/usr/local/openssl/include
   ```

4. **`PKG_CONFIG_PATH`:**  
   If OpenSSL is installed in a non-standard location, the `pkg-config` tool might not be able to find it. Set `PKG_CONFIG_PATH` to the `pkgconfig` folder inside the OpenSSL installation directory.
   
   Example:
   
   ```bash
   export PKG_CONFIG_PATH=/usr/local/openssl/lib/pkgconfig
   ```

### Full Example of Environment Variable Setup:

If you installed OpenSSL in `/usr/local/openssl`, you can set the following environment variables before installing the `reqwest` crate:

```bash
export OPENSSL_DIR=/usr/local/openssl
export OPENSSL_LIB_DIR=/usr/local/openssl/lib
export OPENSSL_INCLUDE_DIR=/usr/local/openssl/include
export PKG_CONFIG_PATH=/usr/local/openssl/lib/pkgconfig
```

Once these variables are set, you can compile `reqwest` or any other crate that depends on OpenSSL:

```bash
cargo build
```

### Verifying OpenSSL Installation:

To ensure that the paths are set correctly, you can verify by checking the installed OpenSSL version:

```bash
$ openssl version
OpenSSL 3.2.0  (Library: /usr/local/openssl/lib)
```

This will confirm that Rust can find and use the correct OpenSSL version during compilation.

##### updating the .bashrc

export OPENSSL_DIR=/usr/local/openssl
export OPENSSL_LIB_DIR=/usr/local/openssl/lib
export OPENSSL_INCLUDE_DIR=/usr/local/openssl/include
export PKG_CONFIG_PATH=/usr/local/openssl/lib/pkgconfig

##### incase above is not sufficient

export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include/openssl
export OPENSSL_DIR=/usr

###### Some commands to find the openssl installation location

find /usr -name "libssl.so" -o -name "libcrypto.so"

pkg-config --libs --cflags openssl

find /usr -name "openssl" -type d

#### Direct installation option

sudo apt install openssl libssl-dev   
