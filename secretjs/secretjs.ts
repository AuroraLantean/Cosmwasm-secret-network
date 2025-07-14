import { type ArrayLog, SecretNetworkClient, Wallet } from "secretjs";
import {
	ll,
	mnemonic,
	secretNetworkId,
	secretNetworkUrl,
	wallet1vk,
	wallet2vk,
} from "./env";

const wallet = new Wallet(mnemonic);
//https://github.com/scrtlabs/secret.js?tab=readme-ov-file#wallet
ll("loading wallet:", wallet.address);
//"publicKey:", wallet.publicKey);

//SecretJs on signer client: https://github.com/scrtlabs/secret.js?tab=readme-ov-file#secretnetworkclient
//URL: https://docs.scrt.network/secret-network-documentation/development/resources-api-contract-addresses/connecting-to-the-network
export const client = new SecretNetworkClient({
	chainId: secretNetworkId,
	url: secretNetworkUrl,
	wallet: wallet,
	walletAddress: wallet.address,
});

//echo "testnets: Osmosis, Juno, Terra, or others"
//https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/
export const secretDeploy = async (secretCtrtPath: string, verbose = false) => {
	ll("secretDeploy()... secretCtrtPath:", secretCtrtPath);
	if (!secretCtrtPath) {
		throw new Error("secretCtrtPath invalid");
	}
	const file = Bun.file(secretCtrtPath); //absolute or relative path
	const contract_wasm = await file.bytes();
	//const contract_wasm = fs.readFileSync("./contract.wasm.gz") as Uint8Array;
	ll("after reading contract file");

	const tx = await client.tx.compute.storeCode(
		{
			sender: wallet.address,
			wasm_byte_code: contract_wasm,
			source: "",
			builder: "",
		},
		{
			gasLimit: 4_000_000,
		},
	); ////https://github.com/scrtlabs/secret.js?tab=readme-ov-file#secretnetworkclient

	if (verbose) ll(tx);
	const codeId = findTxnData(tx.arrayLog, "codeId");
	ll(`SECRET_CONTRACT_CODE_ID= ${codeId}`);
	if (!codeId) {
		throw new Error("codeId invalid");
	}

	//contractCodeHash will change if you chnage your contract. Then codeId will increment by 1
	const contractCodeHash = (
		await client.query.compute.codeHashByCodeId({ code_id: codeId })
	).code_hash;
	ll(`SECRET_CONTRACT_CODE_HASH= ${contractCodeHash}`);
	if (!contractCodeHash) {
		throw new Error("contractCodeHash invalid");
	}
	return { codeId, contractCodeHash };
};

export const findTxnData = (
	txArrayLog: ArrayLog | undefined,
	target: string,
) => {
	let logkey = "code_id";
	if (target === "addr") logkey = "contract_address";

	if (!txArrayLog) {
		console.error("txArrayLog invalid");
		return;
	}
	const codeIdObj = txArrayLog?.find(
		(log) => log.type === "message" && log.key === logkey,
	);
	return codeIdObj?.value;
};

//--------------------==
//https://github.com/scrtlabs/secret.js?tab=readme-ov-file#secretnetworkclient
export const secretInstantiate = async (
	codeId: string | undefined,
	codeHash: string | undefined,
	verbose = false,
) => {
	ll("secretInstantiate()");
	if (!codeId || !codeHash) {
		ll("codeId:", codeId);
		ll("codeHash:", codeHash);
		throw new Error("input invalid");
	}
	const tx = await client.tx.compute.instantiateContract(
		{
			sender: wallet.address,
			admin: wallet.address, // optional admin address that can perform code migrations
			code_id: codeId,
			code_hash: codeHash,
			init_msg: { count: 0, flip: [42] }, //according to your InstantiateMsg
			label: `demo contract ${Math.ceil(Math.random() * 10000)}`, //something unique
			//init_funds: [], // optional
		},
		{
			gasLimit: 400_000,
		},
	);
	if (verbose) ll(tx);

	const contractAddress = findTxnData(tx.arrayLog, "addr");
	ll(`CONTRACT_ADDRESS= ${contractAddress}`);
	return contractAddress;
};

//https://github.com/SecretFoundation/snip20-reference-impl/tree/master/node
export const secretInstantiateSNIP20 = async (
	codeId: string | undefined,
	codeHash: string | undefined,
	verbose = false,
) => {
	ll("secretInstantiateSNIP20()");
	if (!codeId || !codeHash) {
		ll("codeId:", codeId);
		ll("codeHash:", codeHash);
		throw new Error("input invalid");
	}

	const config = {
		//public_total_supply: true,
		/// Indicates whether deposit functionality should be enabled
		/// default: False
		enable_deposit: true,
		/// Indicates whether redeem functionality should be enabled
		/// default: False
		enable_redeem: true,
		/// Indicates whether mint functionality should be enabled
		/// default: False
		enable_mint: true,
		/// Indicates whether burn functionality should be enabled
		/// default: False
		enable_burn: true,
		/// Indicated whether an admin can modify supported denoms
		/// default: False
		can_modify_denoms: true,
	};
	const init_msg = {
		name: "Secret Dragon",
		symbol: "sDRAG",
		decimals: 6, //SCRT has 6 decimals
		prng_seed: Buffer.from("Something really random").toString("base64"),
		admin: wallet.address,
		initial_balances: [
			{
				address: wallet.address,
				amount: "1000000000",
			},
		],
		config,
		// supported_denoms: ["uscrt"],
	};

	const tx = await client.tx.compute.instantiateContract(
		{
			sender: wallet.address,
			admin: wallet.address, // optional admin address that can perform code migrations
			code_id: codeId,
			code_hash: codeHash,
			init_msg, //according to the InstantiateMsg
			label: `secret Dragon coin ${Math.ceil(Math.random() * 10000)}`, //something unique
		},
		{
			gasLimit: 400_000,
		},
	);
	if (verbose) ll(tx);

	const secretCoinAddr = findTxnData(tx.arrayLog, "addr");
	ll(`SECRET_COIN_ADDRESS= ${secretCoinAddr}`);
	return secretCoinAddr;
};

export const secretExecute = async (
	ctrtAddr: string,
	codeHash: string,
	funcName: string,
	arg1: string,
	arg2: string,
) => {
	ll(`secretExecute: 
funcName=${funcName}, arg1: ${arg1}, arg2: ${arg2}`);
	ll("ctrtAddr:", ctrtAddr);
	if (!ctrtAddr) {
		console.error("ctrtAddr invalid");
		return;
	}
	ll("codeHash:", codeHash);
	if (!codeHash) {
		console.error("codeHash invalid");
		return;
	}
	let msg = {}; //all snake_case!
	if (funcName === "flip") {
		msg = { flip: {} };
	} else if (funcName === "password") {
		if (!arg1) {
			console.error("password_key invalid");
			return;
		}
		if (!arg2) {
			console.error("password_value invalid");
			return;
		}
		msg = {
			store_password: {
				password_key: arg1,
				password_value: arg2,
			},
		};
	} else {
		console.error("funcName not valid");
		return;
	}

	const tx = await client.tx.compute.executeContract(
		{
			sender: wallet.address,
			contract_address: ctrtAddr,
			msg, //all snake_case!
			code_hash: codeHash,
		},
		{ gasLimit: 100_000 },
	);
	ll(tx);
};

export const secretQuery = async (
	ctrtAddr: string,
	codeHash: string,
	funcName: string,
	arg1: string | undefined,
) => {
	ll(`secretQuery: 
funcName=${funcName}, arg1: ${arg1}`);

	let query = {}; //all snake_case!
	if (funcName === "flip") {
		query = {
			get_flip: {},
		};
	} else if (funcName === "password") {
		if (!arg1) {
			console.error("key invalid");
			return;
		}
		query = {
			get_password: {
				password_key: arg1,
			},
		};
	} else if (funcName === "token_info") {
		query = {
			token_info: {},
		};
	} else if (funcName === "balance") {
		let viewingkey = "";
		if (!arg1) {
			console.error("arg1 invalid");
			return;
		} else if (arg1 === "vk1") {
			viewingkey = wallet1vk;
		} else if (arg1 === "vk2") {
			viewingkey = wallet2vk;
		} else {
			console.error("viewing key invalid");
			return;
		}
		ll("viewingkey:", viewingkey);
		if (!viewingkey) {
			console.error("viewingkey invalid");
			return;
		}
		query = {
			balance: {
				address: wallet.address,
				key: viewingkey,
			},
		};
	} else {
		console.error("funcName not valid");
		return;
	}

	const query_result = await client.query.compute.queryContract({
		contract_address: ctrtAddr,
		code_hash: codeHash,
		query, //all snake_case in query!
	});
	ll("query_result: ", query_result);
};
