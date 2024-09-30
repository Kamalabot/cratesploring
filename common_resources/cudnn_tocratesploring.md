If you're unable to locate the cuDNN libraries on your system, it likely means cuDNN is not installed or the installation is incomplete. Here’s how you can install cuDNN and ensure it’s properly set up:

### 1. **Download cuDNN**:

- Visit the [NVIDIA cuDNN download page](https://developer.nvidia.com/cudnn) and log in to download the version compatible with your CUDA installation.
  
  ```
  wget https://developer.download.nvidia.com/compute/cudnn/9.4.0/local_installers/cudnn-local-repo-ubuntu2204-9.4.0_1.0-1_amd64.deb
  sudo dpkg -i cudnn-local-repo-ubuntu2204-9.4.0_1.0-1_amd64.deb
  sudo cp /var/cudnn-local-repo-ubuntu2204-9.4.0/cudnn-*-keyring.gpg /usr/share/keyrings/
  sudo apt-get update
  sudo apt-get -y install cudnn
  sudo apt-get -y install cudnn-cuda-12
  ```
  
  
- Select the appropriate version (typically cuDNN for CUDA) and download the tar file for your system.

### 2. **Install cuDNN**:

   After downloading cuDNN, follow these steps to install it:

1. **Extract the cuDNN tar file**:
   
   ```bash
   tar -xzvf cudnn-linux-x86_64-<version>.tgz
   ```

2. **Copy the extracted files to your CUDA installation**:
   
   ```bash
   sudo cp cudnn-linux-x86_64-<version>/include/* /usr/local/cuda/include/
   sudo cp cudnn-linux-x86_64-<version>/lib64/* /usr/local/cuda/lib64/
   ```

### 3. **Verify cuDNN installation**:

   Check if the cuDNN library files were copied to the correct location:

```bash
ls /usr/local/cuda/lib64/libcudnn*
ls /usr/lib/x86-64-linux-gnu/libcudnn*
```

   You should see files like `libcudnn.so` or `libcudnn.so.<version>` in that directory.

### 4. **Set the `CUDNN_LIB` variable**:

   If cuDNN is now installed, export the `CUDNN_LIB` variable to point to the directory:

```bash
export CUDNN_LIB=/usr/local/cuda/lib64
export CUDNN_LIB=
```

   Add it to your shell configuration file for persistence:

```bash
echo 'export CUDNN_LIB=/usr/local/cuda/lib64' >> ~/.bashrc
source ~/.bashrc
```

### 5. **Verify the environment**:

   Confirm that cuDNN is correctly set up:

```bash
nvcc --version
ldconfig -p | grep libcudnn
```

   If cuDNN is correctly installed, it should show up in the output.

After completing these steps, rebuild your project. Let me know if you need further assistance!
