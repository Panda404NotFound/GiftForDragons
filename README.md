[![Static Badge](https://img.shields.io/badge/Telegram-Channel-Link?style=for-the-badge&logo=Telegram&logoColor=white&logoSize=auto&color=blue)](https://t.me/hidden_coding)


[![Static Badge](https://img.shields.io/badge/Telegram-Chat-yes?style=for-the-badge&logo=Telegram&logoColor=white&logoSize=auto&color=blue)](https://t.me/hidden_codding_chat)

[![Static Badge](https://img.shields.io/badge/Telegram-Bot%20Link-Link?style=for-the-badge&logo=Telegram&logoColor=white&logoSize=auto&color=blue)](https://t.me/PAWSOG_bot/PAWS?startapp=xDZm2M3t)

## [HIDDEN CODE MARKET](https://t.me/hcmarket_bot?start=referral_355876562)

#### - [Paws wallet connector](https://t.me/hcmarket_bot?start=referral_355876562-project_1016)
#### - [Premium notpixel](https://t.me/hcmarket_bot?start=referral_355876562-project_1015)
#### - [Blum wallet connector](https://t.me/hcmarket_bot?start=referral_355876562-project_1002)
#### - [Telegram warning up](https://t.me/hcmarket_bot?start=referral_355876562-project_1001)

# GiftForDragons

### **General description:**

This is an autocompounding bot that uses _XPR/XUSDC SNIPS/XPR_ pools for autostaking. Its yield ranges from _65% to 100%_ APY. Launching and scaling of bots happens through docker containers and two-three virtual machines. The bot is processor-based, interacts with the proton client and works in the command shell, which is why three or more virtual machines are recommended.

### **Requirements:**

1. You must have a [Webauth](https://webauth.com) wallet (preferably verified) or Anchor wallet.
To get the private key of your Webauth wallet:
a) Go to [explorer](https://explorer.xprnetwork.org/) and in the top right corner there will be a "login" button. Log into your wallet, view account, utils, private key.


<img width="1004" alt="Снимок экрана 2024-05-10 в 16 44 35" src="https://github.com/Panda404NotFound/GiftForDragons/assets/148841983/bd0190c7-36c1-4ba1-8fba-190cb1312d93">


To get the private key of your anchor wallet:

b) Go to the desktop version of the wallet on PC, manage wallet, three dots on the right, and the private key at the bottom.


<img width="931" alt="Снимок экрана 2024-05-10 в 16 51 26" src="https://github.com/Panda404NotFound/GiftForDragons/assets/148841983/f19ebfc1-862e-4426-ba06-66fdb9e47935">


2. Linux or Debian distribution, or Windows, macOS with VirtualBox.
3. For a workstation or virtual machine, 30 GB of memory, 1 core is required.
4. A private Github repository where you will place each script module for compilation on a workstation or virtual machine.
5. Keep the working server constantly on or use cloud servers.


**!!! CHANGE IN CODE panda4.gm TO YOUR ACCOUNT !!!**


### **Step-by-step plan:**

Place each script module "Liquidity, RustFarmingXPRRewards, StakeLP, Transfer, Withdrawall" in your private or public github repository. Fulfill all the requirements and _read on_ :)

### **Scripts:**

**script.sh** needs to be run on each new machine where the bot needs to be installed. It downloads all the necessary dependencies and creates an ssh key pair for you to download the script modules via ssh. You can remove this part of the script if you want and download via https.

**ss.sh** downloads 3 script modules (which can be scaled) and installs a script that automatically cleans the docker container cache so that the stack does not overflow. It will automatically compile and run the scripts, restarting them each time the PC is rebooted.

**ss1.sh** downloads 2 script modules that need to be run ONCE. Does the same as the first one.

### **REPLACE EVERYWHERE ""ТВОЙ ЗАКРЫТЫЙ ГИТХАБ ЧЕРЕЗ ССХ"" WITH YOUR METHOD OF DOWNLOADING SCRIPT MODULES (SSH/HTTPS)!!!**

**MAKE THE SCRIPT EXECUTABLE BEFORE RUNNING WITH chmod +x "your_script.sh" !!!**

### **Adding funds and withdrawal:**




All in the PROTON_COMMANDS.md file


### **Pros and Cons:**

Pros:
1) The bot increases pool profitability by 30-40% due to its auto-compounding strategy. The average APY is 75%
2) Personal bot, full control
3) Simple code for creating your own strategy

Cons:
1) Complicated setup for beginners
2) Requires a constant server
3) Integrated with a shell client, instead of the Proton API
4) Resource-intensive scaling

My recommendation is to use pooled capital and a single, constant server.
I wrote this bot from the heart, testing the mechanisms of the Rust language. It can be made faster, easier to scale. If we get 15 stars here, I will create a script through the Proton API.

### **Recommendations:**

For stable operation of the bot, 3-5 virtual machines are recommended. Everything can be easily installed on your working PC. Two modules of the bot need to be installed once, the remaining three up to 5 times, on each unique machine. More copies of the bot do not increase productivity. Each file has auto-scripts that will run everything for you, you only need to press two buttons and add your public key to github to download the project via ssh. Or you can make it public by downloading via https, but you will need to change the autostart scripts.

_If you have any questions, please contact the telegram community where we will help you._

Our website: https://bigroom.site
