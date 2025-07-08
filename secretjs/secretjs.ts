import { type ArrayLog, SecretNetworkClient, Wallet } from "secretjs";
import { ll, mnemonic } from "./env";

const wasmFilePath = "./contract.wasm.gz";
const wallet = new Wallet(mnemonic);

const file = Bun.file(wasmFilePath); //absolute or relative path
const contract_wasm = await file.bytes();
//const contract_wasm = fs.readFileSync("./contract.wasm.gz");

//https://docs.scrt.network/secret-network-documentation/development/resources-api-contract-addresses/connecting-to-the-network
export const client = new SecretNetworkClient({
	chainId: "pulsar-3",
	url: "https://pulsar.lcd.secretnodes.com",
	wallet: wallet,
	walletAddress: wallet.address,
});

//echo "testnets: Osmosis, Juno, Terra, or others"
//https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/
export const deploy = async (verbose = false) => {
	ll("deploy()");
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
	ll(`Contract hash: ${contractCodeHash}`);
	if (!contractCodeHash) {
		throw new Error("contractCodeHash invalid");
	}
	return { codeId, contractCodeHash };
};

export const findTxnData = (
	txArrayLog: ArrayLog | undefined,
	target: string,
) => {
	let targetTxt = "code_id";
	if (target === "addr") targetTxt = "contract_address";

	if (!txArrayLog) {
		throw new Error("txArrayLog invalid");
	}
	const codeIdObj = txArrayLog?.find(
		(log) => log.type === "message" && log.key === targetTxt,
	);
	return codeIdObj?.value;
};

//--------------------==
export const instantiate = async (
	codeId: string,
	contractCodeHash: string,
	verbose = false,
) => {
	ll("instantiate()");

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
