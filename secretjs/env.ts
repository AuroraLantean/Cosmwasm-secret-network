declare module "bun" {
	interface Env {
		SECRET0: string;
		MNEMONIC: string;
	}
}
export const ll = console.log;
export const secret0 = Bun.env.SECRET0;
export const mnemonic = Bun.env.MNEMONIC;
