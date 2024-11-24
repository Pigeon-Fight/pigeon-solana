import * as anchor from "@coral-xyz/anchor";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { IDL } from "./utils/idl";
import { PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const MY_TOKEN = "BvUjQKSvmNGiHMSh82Thak49Mkf2QAKf89iAGG81wBtL";
const OP_TOKEN = "74MM7M2yAzAPUV7LpTC1b6XeEVnjZbaMvNfsyfmoVWH7";

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
