# Используем официальный образ Node.js 18
FROM node:18

# Установка локальных npm пакетов
WORKDIR /liquidity
COPY package.json /liquidity/

# Установка необходимых системных пакетов
RUN apt-get update && apt-get install -y expect

# Установка глобальных npm пакетов
RUN npm install -g @proton/cli
RUN npm install @proton/api @proton/js axios child_process node-fetch util

# Копируем исполняемый файл Rust и скрипт
COPY ./target/release/liquidity /liquidity/
COPY ./keyadd.sh /liquidity/

# Даем права на выполнение скриптам
RUN chmod +x /liquidity/liquidity
RUN chmod +x /liquidity/keyadd.sh

# Запускаем скрипт и исполняемый файл Rust
CMD /liquidity/keyadd.sh && /liquidity/liquidity
