declare module "bun" {
	interface Env {
		SECRET0: string;
		MNEMONIC: string;
		SECRET_NETWORK_URL: string;
		SECRET_NETWORK_ID: string;
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
if (!mnemonic) {
	throw new Error("secretNetworkId invalid");
}
export const contractCodeHash = Bun.env.CONTRACT_CODE_HASH;
export const contractAddress = Bun.env.CONTRACT_ADDRESS;
