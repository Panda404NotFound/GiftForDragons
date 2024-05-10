# Используем официальный образ Node.js 18
FROM node:18

# Установка локальных npm пакетов
WORKDIR /stakelp
COPY package.json /stakelp/

# Установка необходимых системных пакетов
RUN apt-get update && apt-get install -y expect

# Установка глобальных npm пакетов
RUN npm install -g @proton/cli
RUN npm install child_process node-fetch util

# Копируем исполняемый файл Rust и скрипт
COPY ./target/release/stakelp /stakelp/
COPY ./keyadd.sh /stakelp/

# Даем права на выполнение скриптам
RUN chmod +x /stakelp/stakelp
RUN chmod +x /stakelp/keyadd.sh

# Запускаем скрипт и исполняемый файл Rust
CMD /stakelp/keyadd.sh && /stakelp/stakelp
