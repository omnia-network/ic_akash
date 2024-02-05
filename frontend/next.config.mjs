import dotenv from "dotenv";
import * as fs from "fs";

const envList = dotenv.config({ path: "./.env" }).parsed || {};
const dfxEnvList = dotenv.config({ path: "../.env" }).parsed || {};

envList.NEXT_PUBLIC_IC_HOST =
  dfxEnvList.DFX_NETWORK === "ic" ? "https://icp-api.io" : "http://127.0.0.1:4943";

const { version } = JSON.parse(fs.readFileSync("./package.json"));

envList.NEXT_PUBLIC_VERSION = version;

/** @type {import('next').NextConfig} */
const nextConfig = {
  env: {
    DFX_NETWORK: dfxEnvList.DFX_NETWORK,
    CANISTER_ID_BACKEND: dfxEnvList.CANISTER_ID_BACKEND,
    ...envList,
  },
  output: "export",
  images: {
    unoptimized: true,
  },
  staticPageGenerationTimeout: 10000
};

export default nextConfig;
