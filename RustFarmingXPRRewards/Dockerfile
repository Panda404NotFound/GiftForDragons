# Используем официальный образ Node.js 18
FROM node:18

# Установка локальных npm пакетов
WORKDIR /rewards
COPY package.json /rewards/

# Установка необходимых системных пакетов
RUN apt-get update && apt-get install -y expect

# Установка глобальных npm пакетов
RUN npm install -g @proton/cli
RUN npm install @proton/api

# Копируем исполняемый файл Rust и скрипт
COPY ./target/release/rustbashrewards /rewards/
COPY ./keyadd.sh /rewards/

# Даем права на выполнение скриптам
RUN chmod +x /rewards/rustbashrewards
RUN chmod +x /rewards/keyadd.sh

# Запускаем скрипт и исполняемый файл Rust
CMD /rewards/keyadd.sh && /rewards/rustbashrewards
