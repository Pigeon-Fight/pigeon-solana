import * as anchor from "@coral-xyz/anchor";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { IDL } from "./utils/idl";
import { PublicKey } from "@solana/web3.js";

const MY_TOKEN = "2qxowa3mGqpqGnMpWdA48cijEKxUNCXg5K3X7QJ28STY";

// Fee: ? SOL
const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const program = new anchor.Program<PigeonBattle>(IDL as any, provider);

  // Invoke
  await program.methods
    .pve({
      class: 0,
      hp: 20,
      energy: 20,
      exp: 0,
      allocatedPoint: 0,
      attack: 10,
      defense: 5,
      speed: 2,
      maxHp: 20,
      maxEnergy: 20,
    })
    .accountsPartial({
      mint: new PublicKey(MY_TOKEN),
    })
    .rpc();

  console.log("Battle PVE successfully!");
};

main().catch((err) => {
  console.error(err);
});
