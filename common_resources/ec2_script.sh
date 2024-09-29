sudo apt update -y && sudo apt upgrade -y
sudo apt install -y build-essential 
sudo apt install - y unzip npm libfuse2 python3-pip fontconfig python3.12-venv xclip xsel
wget https://github.com/neovim/neovim-releases/releases/download/v0.10.1/nvim.appimage
sudo mkdir /opt/nvim
sudo mv nvim.appimage /opt/nvim/nvim
sudo chmod u+x /opt/nvim/nvim
sudo apt install -y openssl libssl-dev libavformat-dev libavfilter-dev libavdevice-dev ffmpeg tesseract-ocr libtesseract-dev libasound2-dev cmake libxcb1-dev
wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-ubuntu2204.pin
sudo mv cuda-ubuntu2204.pin /etc/apt/preferences.d/cuda-repository-pin-600
wget https://developer.download.nvidia.com/compute/cuda/12.2.0/local_installers/cuda-repo-ubuntu2204-12-2-local_12.2.0-535.54.03-1_amd64.deb
sudo dpkg -i cuda-repo-ubuntu2204-12-2-local_12.2.0-535.54.03-1_amd64.deb
sudo cp /var/cuda-repo-ubuntu2204-12-2-local/cuda-*-keyring.gpg /usr/share/keyrings/
sudo apt-get update -y
# but this errors out, stating that nvidia driver is not accepting
sudo apt-get -y install cuda-toolkit-12-2 # goes thru
sudo apt-get install -y nvidia-kernel-open-535
sudo apt-get install -y cuda-drivers-535
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
