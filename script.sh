#!/bin/bash

# Устанавливаем необходимые пакеты
sudo apt update
sudo apt upgrade -y
sudo apt-get install -y pkg-config libssl-dev curl git xclip expect cargo
sleep 1

# Устанавливаем Docker
sudo apt-get update
sudo apt-get install -y apt-transport-https ca-certificates software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io
sudo systemctl enable docker
sudo systemctl start docker
sudo docker info | grep -q "Swarm: inactive" && sudo docker swarm init || echo "Swarm already initialized"

# Генерируем SSH-ключи
expect -c '
  spawn ssh-keygen -t rsa -b 4096 -C panda-lavanda
  expect "Enter file in which to save the key*"
  set ssh_key_path $expect_out(buffer)
  send "\r"
  expect "Enter passphrase*"
  send "\r"
  expect "Enter same passphrase again*"
  send "\r"
  expect eof
'

# Получаем путь к SSH-ключам
ssh_key_path=$(cat ~/.ssh/id_rsa)
ssh_key_pub_path=$(cat ~/.ssh/id_rsa.pub)

# Создаем файл Info на рабочем столе
cd
touch Info.txt

echo "SSH public key path: $ssh_key_pub_path" | xclip -selection clipboard && xclip -selection clipboard -o >> Info.txt

cat $ssh_key_pub_path | tee >(xclip -selection clipboard)
cd
echo "YOU BEST BRO!"