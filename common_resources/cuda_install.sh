#!/bin/bash

echo "starting cuda12-2 install"
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-ubuntu2204.pin
sudo mv cuda-ubuntu2204.pin /etc/apt/preferences.d/cuda-repository-pin-600
wget https://developer.download.nvidia.com/compute/cuda/12.2.0/local_installers/cuda-repo-ubuntu2204-12-2-local_12.2.0-535.54.03-1_amd64.deb
sudo dpkg -i cuda-repo-ubuntu2204-12-2-local_12.2.0-535.54.03-1_amd64.deb
sudo cp /var/cuda-repo-ubuntu2204-12-2-local/cuda-*-keyring.gpg /usr/share/keyrings/
sudo apt-get update -y
sudo apt-get -y install cuda-toolkit-12-2 
echo 'export LD_LIBRARY_PATH="/usr/local/cuda-12.2/lib64${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"' >> ~/.bashrc
echo 'export PATH="/usr/local/cuda-12.2/bin${PATH:+:${PATH}}"' >> ~/.bashrc
source ~/.bashrc 
echo "Completed cuda installation"

echo "Starting cudnn install"
wget https://developer.download.nvidia.com/compute/cudnn/9.4.0/local_installers/cudnn-local-repo-ubuntu2204-9.4.0_1.0-1_amd64.deb
sudo dpkg -i cudnn-local-repo-ubuntu2204-9.4.0_1.0-1_amd64.deb
sudo cp /var/cudnn-local-repo-ubuntu2204-9.4.0/cudnn-*-keyring.gpg /usr/share/keyrings/
sudo apt-get update -y
sudo apt-get -y install cudnn-cuda-12
echo 'export CUDNN_LIB=/usr/lib/x86-64-linux-gnu' >> ~/.bashrc
echo "Completed cudnn installation"

echo "Ensure AWS Credentials are updated in .bashrc"
source ~/.bashrc
echo "Installing Nvidia Driver"
aws s3 cp s3://ec2-linux-nvidia-drivers/grid-16.6/NVIDIA-Linux-x86_64-535.183.01-grid-aws.run .
sudo chmod +x NVIDIA-Linux-x86_64*.run
sudo /bin/sh ./NVIDIA-Linux-x86_64*.run
echo "Completed driver install"


