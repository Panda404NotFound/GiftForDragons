# GiftForDragons
General description:

This is an autocompounding bot that uses XPR/XUSDC SNIPS/XPR pools for autostaking. Its yield ranges from 65% to 100% APY. Launching and scaling of bots happens through docker containers and two-three virtual machines. The bot is processor-based, interacts with the proton client and works in the command shell, which is why three or more virtual machines are recommended.

Requirements:

1. You must have a Webauth wallet (https://webauth.com/) (preferably verified) or Anchor wallet.
To get the private key of your Webauth wallet:
a) Go to https://explorer.xprnetwork.org/ and in the top right corner there will be a "login" button. Log into your wallet, view account, utils, private key.
(screenshot)
To get the private key of your anchor wallet:
b) Go to the desktop version of the wallet on PC, manage wallet, three dots on the right, and the private key at the bottom.
(screenshots)
2. Linux or Debian distribution, or Windows, macOS with VirtualBox.
3. For a workstation or virtual machine, 30 GB of memory, 1 core is required.
4. A private Github repository where you will place each script module for compilation on a workstation or virtual machine.
5. Keep the working server constantly on or use cloud servers.

!!! CHANGE IN CODE panda4.gm TO YOUR ACCOUNT !!!

Step-by-step plan:

Place each script module "Liquidity, RustFarmingXPRRewards, StakeLP, Transfer, Withdrawall" in your private or public github repository. Fulfill all the requirements and read on :)

Scripts:

script.sh needs to be run on each new machine where the bot needs to be installed. It downloads all the necessary dependencies and creates an ssh key pair for you to download the script modules via ssh. You can remove this part of the script if you want and download via https.

ss.sh downloads 3 script modules (which can be scaled) and installs a script that automatically cleans the docker container cache so that the stack does not overflow. It will automatically compile and run the scripts, restarting them each time the PC is rebooted.

ss1.sh downloads 2 script modules that need to be run ONCE. Does the same as the first one.

REPLACE EVERYWHERE ""ТВОЙ ЗАКРЫТЫЙ ГИТХАБ ЧЕРЕЗ ССХ"" WITH YOUR METHOD OF DOWNLOADING SCRIPT MODULES (SSH/HTTPS)!!!

MAKE THE SCRIPT EXECUTABLE BEFORE RUNNING WITH chmod +x "your_script.sh" !!!

Adding funds and withdrawal:

All in the PROTON_COMMANDS.md file

Recommendations:

For stable operation of the bot, 3-5 virtual machines are recommended. Everything can be easily installed on your working PC. Two modules of the bot need to be installed once, the remaining three up to 5 times, on each unique machine. More copies of the bot do not increase productivity. Each file has auto-scripts that will run everything for you, you only need to press two buttons and add your public key to github to download the project via ssh. Or you can make it public by downloading via https, but you will need to change the autostart scripts.

If you have any questions, please contact the telegram community where we will help you.

Our website: https://bigroom.site
