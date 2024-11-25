import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { IDL } from "./utils/idl";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const MY_TOKEN = "H43GgR5uuxYjwziQFst8Ri3S4bZX9tDNdkh795rg1pHZ";

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
    .upgradeNft({
      attack: 1,
      defense: 1,
      speed: 2,
      maxHp: 0,
      maxEnergy: 0,
    })
    .accountsPartial({
      token,
    })
    .rpc();

  console.log("Upgrade NFT successfully!");
};

main().catch((err) => {
  console.error(err);
});
