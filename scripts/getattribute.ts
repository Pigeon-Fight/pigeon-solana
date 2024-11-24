import { Connection, PublicKey } from "@solana/web3.js";
import * as borsh from "borsh";
import { NftAttributes, NftAttributesSchema } from "./accounts/NftAttributes";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { PROGRAM_ID } from "./utils/constants";

const MY_ACCOUNT = "4u4qsoBkNYGdrU2krEYREMAvwL6thb1WYnpEXxzsi2yJ";
const MY_TOKEN = "BvUjQKSvmNGiHMSh82Thak49Mkf2QAKf89iAGG81wBtL";
const RPC_URL = "http://localhost:8899";

const myToken = getAssociatedTokenAddressSync(
  new PublicKey(MY_TOKEN),
  new PublicKey(MY_ACCOUNT)
);

const [attributePubkey, _bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("attributes"), myToken.toBuffer()],
  new PublicKey(PROGRAM_ID)
);

// Connection to the cluster (e.g., localnet, devnet, or mainnet-beta)
const connection = new Connection(RPC_URL);

// Address of the account

(async () => {
  const accountInfo = await connection.getAccountInfo(attributePubkey);

  if (!accountInfo) {
    console.error("Account not found.");
    return;
  }

  // The raw data (Buffer) from the account
  const rawData = accountInfo.data;

  // Skip the first 8 bytes (discriminator)
  const dataWithoutDiscriminator = rawData.slice(8);

  // Deserialize using Borsh
  const nftAttributes = borsh.deserialize(
    NftAttributesSchema,
    NftAttributes,
    dataWithoutDiscriminator
  );

  console.log("Parsed NFT Attributes:", nftAttributes);
})();
