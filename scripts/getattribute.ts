import { Connection, PublicKey } from "@solana/web3.js";
import * as borsh from "borsh";
import { NftAttributes, NftAttributesSchema } from "./accounts/NftAttributes";
import { PROGRAM_ID } from "./utils/constants";

const MY_TOKEN = "H43GgR5uuxYjwziQFst8Ri3S4bZX9tDNdkh795rg1pHZ";
const RPC_URL = "http://localhost:8899";

const [attributePubkey, _bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("attributes"), new PublicKey(MY_TOKEN).toBuffer()],
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
