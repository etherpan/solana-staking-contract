import { fromTokenAmount } from "../program/utils";
import { loadState } from "./loadState";
import { AppState } from "./state";

export async function render() {
    await loadState();
    renderInternal();
}

function renderInternal(){
    if (!AppState.walletConnected) return showConnectWallet();

    showStakingUi();
    showTokenAData();  
    showTokenBData();  

    if (AppState.stakeEntryData.stakeBalance > 0)  
        showRewardsCalculationWidget();
}

function showConnectWallet() {
    (document.querySelector("#connect-wallet") as HTMLElement).style.display = "block";
    (document.querySelector("#wallet-connected") as HTMLElement).style.display = "none";
}

function showStakingUi() {
    (document.querySelector("#connect-wallet") as HTMLElement).style.display = "none";
    (document.querySelector("#wallet-connected") as HTMLElement).style.display = "block";

    if (AppState.stakeEntryData) {
        (document.querySelector("#stake-amount") as HTMLDataElement).textContent = AppState.stakeEntryData.stakeBalance + " WMP";
        (document.querySelector("#rewards-amount") as HTMLDataElement).textContent = AppState.stakeEntryData.rewards + " xWMP";
        (document.querySelector("#b-escrow") as HTMLDataElement).textContent = AppState.stakePoolData.xWmpEscrow.toBase58();
    }
}

function showTokenAData() {
    if (AppState.tokenABalance) {
        (document.querySelector("#a-amount") as HTMLDataElement).textContent = AppState.tokenABalance.balance + " WMP";
    }
}

function showTokenBData() {
    if (AppState.tokenBBalance) {
        (document.querySelector("#b-amount") as HTMLDataElement).textContent = AppState.tokenBBalance.balance + " xWMP";
    }
}

function showRewardsCalculationWidget() {
    setInterval(async () => {
        let rewards = await calcEstimatedRewards();
        (document.querySelector("#estimated-rewards") as HTMLDataElement).textContent = rewards + " xWMP";
    }, 5000);
}

async function calcEstimatedRewards() {
    let stakePoolData = await AppState.program.account.stakePool.fetchNullable(AppState.stakePoolAddress);
    let stakeEntryData = await AppState.program.account.stakeEntry.fetchNullable(AppState.stakeEntryAddress);

    let interval = (Date.now() / 1e3) - stakePoolData.lastUpdateTimestamp.toNumber();
    let rewardsPerToken = fromTokenAmount(stakePoolData.rewardsPerTokenStored) + 
        (interval * fromTokenAmount(stakePoolData.rewardsPerSecond) / fromTokenAmount(stakePoolData.balance));
    
    let rewards = fromTokenAmount(stakeEntryData.rewards) + (rewardsPerToken - fromTokenAmount(stakeEntryData.rewardsPerTokenPaid)) * fromTokenAmount(stakeEntryData.balance);

    return rewards;
}

