import * as anchor from "@coral-xyz/anchor";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { IDL } from "./utils/idl";
import { PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const MY_TOKEN = "6h9g35j3Yyr5h9qH5yfoRjaFMc297hUkAS4iEGD6vcKi";
const OP_TOKEN = "2Axa3x4B6sMQRLdSb7pK33AHM18H84F3c3DsMzsQbC6t";

// Fee: ? SOL
const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const program = new anchor.Program<PigeonBattle>(IDL as any, provider);

  const myToken = getAssociatedTokenAddressSync(
    new PublicKey(MY_TOKEN),
    provider.wallet.publicKey
  );
  const opToken = getAssociatedTokenAddressSync(
    new PublicKey(OP_TOKEN),
    provider.wallet.publicKey
  );

  // Invoke
  await program.methods
    .battle()
    .accountsPartial({
      myToken,
      opToken,
    })
    .rpc();

  console.log("Battle successfully!");
};

main().catch((err) => {
  console.error(err);
});
