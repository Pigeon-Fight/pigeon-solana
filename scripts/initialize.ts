import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { PigeonBattle } from "../target/types/pigeon_battle";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { getMasterEdition, getMetadata } from "./helpers";
import { PROGRAM_ID } from "./constants";

// GasFee: 0.02 SOL

const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const programId = new PublicKey(PROGRAM_ID);
  const idl = await anchor.Program.fetchIdl<PigeonBattle>(programId, provider);
  const program = new anchor.Program(idl, provider);

  // Generate a new mint
  const collectionKeypair = Keypair.generate();
  const collectionMint = collectionKeypair.publicKey;
  const destination = getAssociatedTokenAddressSync(
    collectionMint,
    provider.wallet.publicKey
  );

  // Derive metadata and master edition accounts
  const metadata = await getMetadata(collectionMint);
  const masterEdition = await getMasterEdition(collectionMint);

  // Invoke
  await program.methods
    .initialize()
    .accountsPartial({
      user: provider.wallet.publicKey,
      mint: collectionMint,
      metadata,
      masterEdition,
      destination,
    })
    .signers([collectionKeypair])
    .rpc();

  console.log("NFT collection minted successfully!");
};

main().catch((err) => {
  console.error(err);
});
