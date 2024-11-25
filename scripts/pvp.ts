import * as anchor from "@coral-xyz/anchor";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { IDL } from "./utils/idl";
import { PublicKey } from "@solana/web3.js";

const MY_TOKEN = "2qxowa3mGqpqGnMpWdA48cijEKxUNCXg5K3X7QJ28STY";
const OP_TOKEN = "9FJLBFUbDP46DNvpg9Ubphdj7ZRdcZaChd2id232Hgta";

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
    .pvp()
    .accountsPartial({
      mint: new PublicKey(MY_TOKEN),
      opMint: new PublicKey(OP_TOKEN),
    })
    .rpc();

  console.log("Battle PVP successfully!");
};

main().catch((err) => {
  console.error(err);
});
