import {
	ll,
	secretCoinAddress,
	secretCoinCodeHash,
	secretCtrtAddress,
	secretCtrtCodeHash,
	secretCtrtCodeId,
	secretDragonCoinPath,
	sectetDemoCtrtPath,
} from "./env";
import {
	secretDeploy,
	secretExecute,
	secretInstantiate,
	secretInstantiateSNIP20,
	secretQuery,
} from "./secretjs";

// To run this script: bun index caseId
const caseId: string = Bun.argv[2] || "";
ll("caseId =", caseId);

switch (caseId) {
	case "0": //bun run index 0
		{
			ll("new function");
		}
		break;
	//deploy
	case "1": //bun run index 1
		{
			const verbose = Boolean(Bun.argv[3]);
			const secretCtrtPath = sectetDemoCtrtPath;
			const { codeId, contractCodeHash } = await secretDeploy(
				secretCtrtPath,
				verbose,
			);
			await secretInstantiate(codeId, contractCodeHash, verbose);
		}
		break;
	//secretInstantiate
	case "2": //bun run index 2
		{
			const verbose = Boolean(Bun.argv[3]);
			await secretInstantiate(secretCtrtCodeId, secretCtrtCodeHash, verbose);
		}
		break;
	//execute
	case "3": //bun run index 3 funcName arg1 arg2
		{
			const funcName = Bun.argv[3] || "flip";
			const arg1 = Bun.argv[4] || "key123";
			const arg2 = Bun.argv[5] || "pw456";
			await secretExecute(
				secretCtrtAddress,
				secretCtrtCodeHash,
				funcName,
				arg1,
				arg2,
			);
		}
		break;
	//query
	case "4": //bun run index 4 funcName arg1
		{
			const funcName = Bun.argv[3] || "flip";
			const arg1 = Bun.argv[4] || "key123";
			await secretQuery(secretCtrtAddress, secretCtrtCodeHash, funcName, arg1);
		}
		break;

	//deploy secret token
	case "21": //bun run index 21
		{
			const verbose = Boolean(Bun.argv[3]);
			const secretCtrtPath = secretDragonCoinPath;
			const { codeId, contractCodeHash } = await secretDeploy(
				secretCtrtPath,
				verbose,
			);
			await secretInstantiateSNIP20(codeId, contractCodeHash, verbose);
		}
		break;
	//query secret coin
	case "22": //bun run index 22 funcName arg1
		{
			const funcName = Bun.argv[3] || ""; //token_info, balance + vk1;
			const arg1 = Bun.argv[4] || "";
			await secretQuery(secretCoinAddress, secretCoinCodeHash, funcName, arg1);
		}
		break;
	// send SCRT
	case "31": //bun run index 31
		{
			const verbose = Boolean(Bun.argv[3]);
			const secretCtrtPath = secretDragonCoinPath;
			const { codeId, contractCodeHash } = await secretDeploy(
				secretCtrtPath,
				verbose,
			);
			await secretInstantiateSNIP20(codeId, contractCodeHash, verbose);
		}
		break;
	default:
		ll("unexpected caseId");
}
