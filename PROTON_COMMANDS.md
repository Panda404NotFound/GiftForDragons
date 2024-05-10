I recommend using two wallets, one working, the other for receiving and sending funds.

IF YOU USE ONE WALLET:
If the main wallet is your working one, then the script will use XPR SNIPS XUSDC tokens to interact with the bot, which can send your tokens straight to work. I recommend:
a) receive tokens in XETH or other tokens and exchange them in equal proportion between working pools. For example, $100 of which $50 XPR, $25 SNIPS, $25 XUSDC, the bot automatically uses these tokens and will deposit into the liquidity pool.
b) use manually through the terminal staking input commands, which I will attach below, but this is a difficult way for a beginner.

IF YOU USE TWO WALLETS:
If the main wallet is for receiving and sending, and the second for work, then you can convert tokens and send to the wallet according to the previous example or use commands manually.

STAKING INPUT COMMANDS. REPLACE "panda4.gm" WITH YOUR WALLET :

Deposit Preparation:

proton action proton.swaps depositprep "{\"owner\": \"panda4.gm\", \"symbols\": [{\"sym\": \"4,SNIPS\", \"contract\": \"snipcoins\"}, {\"sym\": \"4,XPR\", \"contract\": \"eosio.token\"}]}" panda4.gm; 

proton action proton.swaps depositprep "{\"owner\": \"panda4.gm\", \"symbols\": [{\"sym\": \"6,XUSDC\", \"contract\": \"xtokens\"}, {\"sym\": \"4,XPR\", \"contract\": \"eosio.token\"}]}" panda4.gm; 

Sending tokens to Staking. Replace the number of tokens with the desired number of tokens:

proton action snipcoins transfer "{\"from\": \"panda4.gm\", \"to\": \"proton.swaps\", \"quantity\": \"21019499.2055 SNIPS\", \"memo\": \"deposit\"}" panda4.gm; 

proton action eosio.token transfer "{\"from\": \"panda4.gm\", \"to\": \"proton.swaps\", \"quantity\": \"277972.7364 XPR\", \"memo\": \"deposit\"}" panda4.gm; 

proton action xtokens transfer "{\"from\": \"panda4.gm\", \"to\": \"proton.swaps\", \"quantity\": \"2.007798 XUSDC\", \"memo\": \"deposit\"}" panda4.gm; 

Entering tokens into staking according to your deposit from “proton.swaps”:

proton action proton.swaps liquidityadd "{\"owner\": \"panda4.gm\", \"lt_symbol\": \"8,SNIPSXP\", \"add_token1\": {\"quantity\": \"21019499.2055 SNIPS\", \"contract\": \"snipcoins\"}, \"add_token2\": {\"quantity\": \"210187.3932 XPR\", \"contract\": \"eosio.token\"}, \"add_token1_min\": {\"quantity\": \"21019499.2054 SNIPS\", \"contract\": \"snipcoins\"}, \"add_token2_min\": {\"quantity\": \"210187.3932 XPR\", \"contract\": \"eosio.token\"}}" panda4.gm

proton action proton.swaps liquidityadd "{\"owner\": \"panda4.gm\", \"lt_symbol\": \"8,XPRUSDC\", \"add_token1\": {\"quantity\": \"63290.1784 XPR\", \"contract\": \"eosio.token\"}, \"add_token2\": {\"quantity\": \"63.027798 XUSDC\", \"contract\": \"xtokens\"}, \"add_token1_min\": {\"quantity\": \"63290.1783 XPR\", \"contract\": \"eosio.token\"}, \"add_token2_min\": {\"quantity\": \"63.027797 XUSDC\", \"contract\": \"xtokens\"}}" panda4.gm

COMMANDS TO WITHDRAW TOKENS FROM STAKING:

Output all tokens from proton.swaps:

proton action proton.swaps withdrawall "{\"owner\":\"panda4.gm\"}" panda4.gm

Withdrawal from staking and into an account:

proton action yield.farms withdraw "{\"withdrawer\": \"panda4.gm\", \"token\": {\"quantity\": \"38.08314311 XPRUSDC\", \"contract\": \"proton.swaps\"}}" panda4.gm

proton action proton.swaps liquidityrmv "{\"owner\":\"panda4.gm\", \"lt\": \"38.08314311 XPRUSDC\"}" panda4.gm

proton action proton.swaps withdrawall "{\"owner\":\"panda4.gm\"}" panda4.gm
