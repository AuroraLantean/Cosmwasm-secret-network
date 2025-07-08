import { ll } from "./env";
import {
	secretDeploy,
	secretExecute,
	secretInstantiate,
	secretQuery,
} from "./secretjs";

// To run this script: bun index CASE_NUMBER
const selection: string = Bun.argv[2] || "";
ll("selection =", selection);

//bun run index 999
switch (selection) {
	//mint0 = await makeMint(provider, payer, mintAuth0, tokenDpNum, "mint0");
	case "999":
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
	//execute
	case "1": //bun run index 1
		{
			const verbose = Boolean(Bun.argv[3]);
			const password_key = "key123";
			const password_value = "password456";

			await secretExecute(password_key, password_value, verbose);
		}
		break;
	//query
	case "2": //bun run index 2
		{
			const key = Bun.argv[3];
			const verbose = Boolean(Bun.argv[4]);
			await secretQuery(key, verbose);
		}
		break;
	default:
		ll("unexpected selection");
}
