import { ll } from "./env";
import { deploy, instantiate } from "./secretjs";

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
	case "deploy": //bun run index deploy
		{
			const verbose = Boolean(Bun.argv[3]);
			const { codeId, contractCodeHash } = await deploy(verbose);
			if (!codeId || !contractCodeHash) {
				throw new Error("codeId or contractCodeHash invalid");
			}
			await instantiate(codeId, contractCodeHash, verbose);
		}
		break;
	default:
		ll("unpected selection");
}
