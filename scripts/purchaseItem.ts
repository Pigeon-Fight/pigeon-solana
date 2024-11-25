import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { ADMIN } from "./utils/constants";
import { IDL } from "./utils/idl";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const ITEM_CLASS = 46;
const MY_TOKEN = "6h9g35j3Yyr5h9qH5yfoRjaFMc297hUkAS4iEGD6vcKi";

// Fee: 0.021 SOL
const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const program = new anchor.Program<PigeonBattle>(IDL as any, provider);

  const token = getAssociatedTokenAddressSync(
    new PublicKey(MY_TOKEN),
    provider.wallet.publicKey
  );

  // Invoke
  await program.methods
    .purchaseItem(ITEM_CLASS)
    .accountsPartial({
      admin: new PublicKey(ADMIN),
      token,
    })
    .rpc();

  console.log("purchase item successfully!");
};

main().catch((err) => {
  console.error(err);
});
