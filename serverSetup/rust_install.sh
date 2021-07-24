#!/bin/sh

echo "###########################################"
echo "#          UBUNTU RUST INSTALL            #"
echo "###########################################"


echo "rust Install"

curl https://sh.rustup.rs -sSf | sh

echo "PATH ADD !!"

export PATH="$HOME/.cargo/bin:$PATH"

echo "Please Reboot Now !!"