#!/bin/bash

sudo apt update -y && sudo apt upgrade -y
sudo apt install -y build-essential pkg-config \
  unzip npm python3-pip fontconfig python3.12-venv xclip xsel \
  openssl libssl-dev libavformat-dev libavfilter-dev libavdevice-dev \
  ffmpeg tesseract-ocr libtesseract-dev libasound2-dev cmake libxcb1-dev
  libclang-dev librocksdb-dev awscli python3-pip libfuse2 docker.io \
  python3.10-venv
echo "adding additional libraries including awscli"
sudo apt-get upgrade -y linux-aws
sudo apt-get install -y linux-headers-$(uname -r)
echo "installing nvim"
wget https://github.com/neovim/neovim-releases/releases/download/v0.10.1/nvim.appimage
sudo mkdir /opt/nvim
sudo mv nvim.appimage /opt/nvim/nvim
sudo chmod u+x /opt/nvim/nvim
sudo echo 'export PATH="$PATH:/opt/nvim"' >> ~/.bashrc
echo "added nvim to path. Installing rust"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
rustup component add rust-analyzer 
rustup component add clippy
rustup component add rustfmt 
echo "installed rust and components"
cat << EOF | sudo tee --append /etc/modprobe.d/blacklist.conf
blacklist vga16fb
blacklist nouveau
blacklist rivafb
blacklist nvidiafb
blacklist rivatv
EOF
echo "blacklist entries added"
sudo echo 'GRUB_CMDLINE_LINUX="rdblacklist=nouveau"' >> /etc/default/grub
sudo update-grub
echo "updated grub"
echo "Update .bashrc file"
echo 'export AWS_ACCESS_KEY_ID="your-access-key-id"' >> ~/.bashrc
echo 'export AWS_SECRET_ACCESS_KEY="your-secret-access-key"' >> ~/.bashrc
echo 'export AWS_DEFAULT_REGION="your-region"' >> ~/.bashrc
echo 'export DATABASE_URL="postgres url"' >> ~/.bashrc
echo 'export OPENAI_API_KEY="YOUR API KEY"' >> ~/.bashrc
echo "installing lvim"
wget https://raw.githubusercontent.com/LunarVim/LunarVim/release-1.4/neovim-0.9/utils/installer/install.sh
sudo chmod +x install.sh
source ~/.bashrc
./install.sh -y
sudo echo 'export PATH="$PATH:/home/ubuntu/.local/bin"' >> ~/.bashrc
sudo echo 'alias cls="clear"' >> ~/.bashrc
source ~/.bashrc
echo "Lvim Install completed"
# echo "Installing Surreal DB completed"
# curl -sSf https://install.surrealdb.com | sh
# echo "Installing Vnc & Ubuntu Desktop"
# sudo apt-get install -y lightdm ubuntu-desktop
# sudo apt install -y tightvncserver
# sudo apt install -y gnome-panel gnome-settings-daemon metacity nautilus gnome-terminal
# echo "Desktop installed, now Configure the vnc server xstartup"
# echo "visit https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/install-nvidia-driver.html for next steps"
# vncserver :1
