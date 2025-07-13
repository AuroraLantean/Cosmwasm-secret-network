declare module "bun" {
	interface Env {
		SECRET0: string;
		MNEMONIC: string;
		SECRET_NETWORK_URL: string;
		SECRET_NETWORK_ID: string;
		CONTRACT_PATH: string;
		CONTRACT_CODE_ID: string;
		CONTRACT_CODE_HASH: string;
		CONTRACT_ADDRESS: string;
	}
}
export const ll = console.log;
export const secret0 = Bun.env.SECRET0;
export const mnemonic = Bun.env.MNEMONIC;
if (!mnemonic) {
	throw new Error("mnemonic invalid");
}
export const secretNetworkUrl = Bun.env.SECRET_NETWORK_URL;
if (!secretNetworkUrl) {
	throw new Error("secretNetworkUrl invalid");
}
export const secretNetworkId = Bun.env.SECRET_NETWORK_ID;
if (!secretNetworkId) {
	throw new Error("secretNetworkId invalid");
}
export const sectetDemoCtrtPath = Bun.env.SECRET_DEMO_CTRT_PATH || "";
export const secretDragonCoinPath = Bun.env.SECRET_DRAGON_COIN_PATH || "";

export const secretCtrtCodeId = Bun.env.SECRET_CONTRACT_CODE_ID || "";
export const secretCtrtCodeHash = Bun.env.SECRET_CONTRACT_CODE_HASH || "";
export const secretCtrtAddress = Bun.env.SECRET_CONTRACT_ADDRESS || "";

export const secretCoinCodeHash = Bun.env.SECRET_COIN_CODE_HASH || "";
export const secretCoinAddress = Bun.env.SECTET_COIN_ADDR || "";
