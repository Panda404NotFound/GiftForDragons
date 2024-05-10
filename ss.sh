#!/bin/bash

cd

mkdir Project
sleep 1
cd Project

# Клонирование репозиториев
sleep 1
git clone ""ТВОЙ ЗАКРЫТЫЙ ГИТХАБ ЧЕРЕЗ ССХ""
sleep 1
git clone ""ТВОЙ ЗАКРЫТЫЙ ГИТХАБ ЧЕРЕЗ ССХ""
sleep 1
git clone ""ТВОЙ ЗАКРЫТЫЙ ГИТХАБ ЧЕРЕЗ ССХ""
sleep 1

# Сборка и создание Docker образа для Transfer
cd Transfer
cargo build --release
sleep 1
sudo docker build -t transfer .
sleep 1
cd ..

# Сборка и создание Docker образа для Liquidity
cd Liquidity
cargo build --release
sleep 1
sudo docker build -t liquidity .
sleep 1
cd ..

# Сборка и создание Docker образа для StakeLP
cd StakeLP
cargo build --release
sleep 1
sudo docker build -t stakelp .
sleep 1
cd

# Получаем путь текущей директории
current_dir=$(pwd)

# Создаем скрипт docker_prune.sh
cat > docker_prune.sh <<EOL
#!/bin/bash

# Ждем 30 секунд перед первым запуском
sleep 30

while true
do
# Выполняем docker system prune
sudo docker system prune -f

# Ждем 3 суток (259200 секунд) перед следующим запуском
sleep 259200
done
EOL

# Делаем скрипт исполняемым
chmod +x docker_prune.sh

# Создаем systemd unit файл для запуска скрипта в фоновом режиме
sudo tee /etc/systemd/system/docker_prune.service > /dev/null <<EOL
[Unit]
Description=Docker Prune Service

[Service]
ExecStart=$current_dir/docker_prune.sh
Restart=always

[Install]
WantedBy=multi-user.target
EOL

# Перезагружаем systemd для применения изменений
sudo systemctl daemon-reload

# Включаем и запускаем сервис
sudo systemctl enable docker_prune.service
sudo systemctl start docker_prune.service

# Создание Docker сервисов
sudo docker service create --name transfer --replicas 1 --restart-condition any transfer
sudo docker service create --name stakelp --replicas 1 --restart-condition any stakelp
sudo docker service create --name liquidity --replicas 1 --restart-condition any liquidity
