import dotenv from "dotenv";
import path from "path";
import { z } from "zod";

dotenv.config({ path: path.resolve(__dirname, "../.env"), override: true });

const envVarsSchema = z.object({
  RPC_URL: z.string(),
  WS_URL: z.string(),
});

const envVars = envVarsSchema.parse(process.env);

const envConfig = {
  rpcUrl: envVars.RPC_URL,
  wsUrl: envVars.WS_URL,
};

export default envConfig;
