# Используем официальный образ Node.js 18
FROM node:18

# Установка локальных npm пакетов
WORKDIR /transfer
COPY package.json /transfer/

# Установка необходимых системных пакетов
RUN apt-get update && apt-get install -y expect

# Установка глобальных npm пакетов
RUN npm install -g @proton/cli
RUN npm install child_process util

# Копируем исполняемый файл Rust и скрипт
COPY ./target/release/transfer /transfer/
COPY ./keyadd.sh /transfer/

# Даем права на выполнение скриптам
RUN chmod +x /transfer/transfer
RUN chmod +x /transfer/keyadd.sh

# Запускаем скрипт и исполняемый файл Rust
CMD /transfer/keyadd.sh && /transfer/transfer
