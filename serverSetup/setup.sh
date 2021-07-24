#!/bin/sh

echo "###########################################"
echo "#      UBUNTU RUST SERVER SETUP           #"
echo "###########################################"

echo "gcc Install";

sudo apt install build-essential

echo "mysql Install"

sudo apt install mysql-server mysql-client

echo "libmysqlclient Install"

sudo apt install libmysqlclient

echo "docer-compose Install"

sudo apt install docker-compose

echo "libssl-dev Install"

sudo apt install libssl-dev

echo "pkg-config Install"

sudo apt install pkg-config

echo "firewall active !!"

sudo ufw enable

echo "port 80 443 22 open"

sudo ufw allow 80

sudo ufw allow 443

sudo ufw allow 22

echo "port status:"

sudo ufw status numbered