import { contractCodeHash, contractCodeId, ll } from "./env";
import {
	secretDeploy,
	secretExecute,
	secretInstantiate,
	secretQuery,
} from "./secretjs";

// To run this script: bun index caseId
const caseId: string = Bun.argv[2] || "";
ll("caseId =", caseId);

switch (caseId) {
	case "999": //bun run index 999
		{
			ll("new function");
		}
		break;
	//deploy
	case "0": //bun run index 0
		{
			const verbose = Boolean(Bun.argv[3]);
			const { codeId, contractCodeHash } = await secretDeploy(verbose);

			await secretInstantiate(codeId, contractCodeHash, verbose);
		}
		break;
	//secretInstantiate
	case "1": //bun run index 1
		{
			const verbose = Boolean(Bun.argv[3]);
			await secretInstantiate(contractCodeId, contractCodeHash, verbose);
		}
		break;
	//execute
	case "2": //bun run index 2 funcName arg1 arg2
		{
			const funcName = Bun.argv[3] || "flip";
			const arg1 = Bun.argv[4] || "key123";
			const arg2 = Bun.argv[5] || "pw456";
			await secretExecute(funcName, arg1, arg2);
		}
		break;
	//query
	case "3": //bun run index 3 funcName arg1
		{
			const funcName = Bun.argv[3] || "flip";
			const arg1 = Bun.argv[4] || "key123";
			await secretQuery(funcName, arg1);
		}
		break;
	default:
		ll("unexpected caseId");
}
