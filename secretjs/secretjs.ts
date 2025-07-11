import { type ArrayLog, SecretNetworkClient, Wallet } from "secretjs";
import {
	contractAddress,
	contractCodeHash,
	contractPath,
	ll,
	mnemonic,
	secretNetworkId,
	secretNetworkUrl,
} from "./env";

const wasmFilePath = contractPath;
const wallet = new Wallet(mnemonic);
//https://github.com/scrtlabs/secret.js?tab=readme-ov-file#wallet
ll("loading wallet:", wallet.address, ", publicKey:", wallet.publicKey);

const file = Bun.file(wasmFilePath); //absolute or relative path
const contract_wasm = await file.bytes();
//const contract_wasm = fs.readFileSync("./contract.wasm.gz") as Uint8Array;
ll("after reading contract file");

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
export const secretDeploy = async (verbose = false) => {
	ll("secretDeploy()");
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
	ll(`CONTRACT_CODE_ID=${codeId}`);
	if (!codeId) {
		throw new Error("codeId invalid");
	}

	//contractCodeHash will change if you chnage your contract. Then codeId will increment by 1
	const contractCodeHash = (
		await client.query.compute.codeHashByCodeId({ code_id: codeId })
	).code_hash;
	ll(`CONTRACT_CODE_HASH=${contractCodeHash}`);
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
		throw new Error("txArrayLog invalid");
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
			init_msg: { count: 0 }, //according to your InstantiateMsg
			label: `demo contract ${Math.ceil(Math.random() * 10000)}`, //something unique
			//init_funds: [], // optional
		},
		{
			gasLimit: 400_000,
		},
	);
	if (verbose) ll(tx);

	const contractAddress = findTxnData(tx.arrayLog, "addr");
	ll(`CONTRACT_ADDRESS=${contractAddress}`);
	return contractAddress;
};
/*    init_msg: {
      name: "Secret SCRT",
      admin: myAddress,
      symbol: "SSCRT",
      decimals: 6,
      initial_balances: [{ address: myAddress, amount: "1" }],
      prng_seed: "eW8=",
      config: {
        public_total_supply: true,
        enable_deposit: true,
        enable_redeem: true,
        enable_mint: false,
        enable_burn: false,
      },
      supported_denoms: ["uscrt"],
    },
 */

export const secretExecute = async (
	password_key: string,
	password_value: string,
) => {
	ll("secretExecute");
	if (!password_key) throw new Error("password_key invalid");
	if (!password_value) throw new Error("password_value invalid");

	ll("contractAddress:", contractAddress);
	if (!contractAddress) {
		throw new Error("contractAddress invalid");
	}
	ll("contractCodeHash:", contractCodeHash);
	if (!contractCodeHash) {
		throw new Error("contractCodeHash invalid");
	}

	const tx = await client.tx.compute.executeContract(
		{
			sender: wallet.address,
			contract_address: contractAddress,
			msg: {
				store_password: {
					password_key,
					password_value,
				},
			}, //all snake_case!
			code_hash: contractCodeHash,
		},
		{ gasLimit: 100_000 },
	);
	ll(tx);
};

export const secretQuery = async (key: string | undefined) => {
	ll("secretQuery");
	if (!key) {
		ll("key:", key);
		throw new Error("key invalid");
	}
	const my_query = await client.query.compute.queryContract({
		contract_address: contractAddress,
		code_hash: contractCodeHash,
		query: {
			get_password: {
				password_key: key,
			},
		}, //all snake_case in query!
	});
	ll("value: ", my_query);
};
