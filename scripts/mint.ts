import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PigeonBattle } from "../target/types/pigeon_battle";
import { generateSigner } from "@metaplex-foundation/umi";
import {
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { getAssociatedTokenAddress } from "@solana/spl-token";

const NFT_CLASS = 1;

const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const programId = new PublicKey(
    "GtJQXa6CQ4qX4GuM92v7DbissAbVwrQJVcx5a4YESiCS"
  ); // Replace with your program ID
  const idl = await anchor.Program.fetchIdl<PigeonBattle>(programId, provider);
  const program = new anchor.Program(idl, provider);

  // Generate a new mint
  const mint = generateSigner(umi);
  const userTokenAccount = await getAssociatedTokenAddress(
    new PublicKey(mint.publicKey),
    provider.wallet.publicKey
  );

  // Derive metadata and master edition accounts
  const [metadataPDA, _metadataBump] = findMetadataPda(umi, {
    mint: mint.publicKey,
  });
  const [masterEditionPDA, _masterEditionBump] = findMasterEditionPda(umi, {
    mint: mint.publicKey,
  });

  // Invoke
  await program.methods
    .mintNft(NFT_CLASS)
    .accounts({
      user: provider.wallet.publicKey,
      mint: mint.publicKey,
      token: userTokenAccount,
      metadataAccount: metadataPDA,
      masterEditionAccount: masterEditionPDA,
    })
    .rpc();

  console.log("NFT minted successfully!");
};

main().catch((err) => {
  console.error(err);
});
