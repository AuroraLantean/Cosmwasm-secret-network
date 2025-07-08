import { type ArrayLog, SecretNetworkClient, Wallet } from "secretjs";
import {
	contractAddress,
	contractCodeHash,
	ll,
	mnemonic,
	secretNetworkId,
	secretNetworkUrl,
} from "./env";

const wasmFilePath = "./contract.wasm.gz";
const wallet = new Wallet(mnemonic);
ll("loading wallet:", wallet.address, wallet.publicKey);

const file = Bun.file(wasmFilePath); //absolute or relative path
const contract_wasm = await file.bytes();
//const contract_wasm = fs.readFileSync("./contract.wasm.gz");

//https://docs.scrt.network/secret-network-documentation/development/resources-api-contract-addresses/connecting-to-the-network
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
	);
	if (verbose) ll(tx);
	const codeId = findTxnData(tx.arrayLog, "codeId");
	ll("codeId: ", codeId);

	const contractCodeHash = (
		await client.query.compute.codeHashByCodeId({ code_id: codeId })
	).code_hash;
	ll(`CONTRACT_CODE_HASH: ${contractCodeHash}`);
	if (!contractCodeHash) {
		throw new Error("contractCodeHash invalid");
	}
	return { codeId, contractCodeHash };
};

export const findTxnData = (
	txArrayLog: ArrayLog | undefined,
	target: string,
) => {
	let targetKey = "code_id";
	if (target === "addr") targetKey = "contract_address";

	if (!txArrayLog) {
		throw new Error("txArrayLog invalid");
	}
	const codeIdObj = txArrayLog?.find(
		(log) => log.type === "message" && log.key === targetKey,
	);
	return codeIdObj?.value;
};

//--------------------==
export const secretInstantiate = async (
	codeId: string | undefined,
	contractCodeHash: string | undefined,
	verbose = false,
) => {
	ll("secretInstantiate()");
	if (!codeId || !contractCodeHash) {
		ll("codeId:", codeId);
		ll("contractCodeHash:", contractCodeHash);
		throw new Error("input invalid");
	}
	const tx = await client.tx.compute.instantiateContract(
		{
			code_id: codeId,
			sender: wallet.address,
			code_hash: contractCodeHash,
			init_msg: {}, //according to your InstantiateMsg
			label: `example contract ${Math.ceil(Math.random() * 10000)}`, //something unique
			admin: wallet.address,
		},
		{
			gasLimit: 400_000,
		},
	);
	if (verbose) ll(tx);

	const contractAddress = findTxnData(tx.arrayLog, "addr");
	ll("contractAddress: ", contractAddress);
	return contractAddress;
};

export const secretExecute = async (
	password_key: string,
	password_value: string,
	_verbose = false,
) => {
	ll("secretExecute");
	if (!password_key) throw new Error("password_key invalid");
	if (!password_value) throw new Error("password_value invalid");

	const tx = await client.tx.compute.executeContract(
		{
			sender: wallet.address,
			contract_address: contractAddress,
			msg: {
				store_password: {
					password_key,
					password_value,
				},
			}, //all snake_case in msg!
			code_hash: contractCodeHash,
		},
		{ gasLimit: 100_000 },
	);
	ll(tx);
};

export const secretQuery = async (
	key: string | undefined,
	_verbose = false,
) => {
	ll("secretQuery");
	if (!key) throw new Error("key invalid");

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
