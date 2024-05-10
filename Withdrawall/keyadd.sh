#!/usr/bin/expect -f

# Запуск команды proton key:add 

spawn proton key:add "YOUR_KEY"

# Ожидание запроса ввода ответа

expect "Would you like to encrypt your stored keys with a password?"

# Отправка ответа "no"

send "no\r"

# Ожидание 5 секунд

sleep 5

# Ожидание завершения команды

expect eof
