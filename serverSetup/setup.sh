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