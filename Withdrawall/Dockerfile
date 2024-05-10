# Используем официальный образ Node.js 18
FROM node:18

# Установка локальных npm пакетов
WORKDIR /withdrawall
COPY package.json /withdrawall/

# Установка необходимых системных пакетов
RUN apt-get update && apt-get install -y expect

# Установка глобальных npm пакетов
RUN npm install -g @proton/cli

# Копируем исполняемый файл Rust и скрипт
COPY ./target/release/withdrawall /withdrawall/
COPY ./keyadd.sh /withdrawall/keyadd.sh

# Даем права на выполнение скриптам
RUN chmod +x /withdrawall/withdrawall
RUN chmod +x /withdrawall/keyadd.sh

# Запускаем скрипт и исполняемый файл Rust
CMD /withdrawall/keyadd.sh && /withdrawall/withdrawall
