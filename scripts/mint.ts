import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { PigeonBattle } from "../target/types/pigeon_battle";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { getMasterEdition, getMetadata } from "./utils/helpers";
import { ADMIN, PROGRAM_ID } from "./utils/constants";
import { IDL } from "./utils/idl";

const NFT_CLASS = 8;
const MINT_COLLECTION_ACCOUNT = "FeR2HK3EyDH98HGvVmJopgxDqgsMEqJQnCjnJBXF6dzd";

// Fee: 0.021 SOL
const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const programId = new PublicKey(PROGRAM_ID); // Replace with your program ID
  const program = new anchor.Program<PigeonBattle>(IDL as any, provider);

  // Generate a new mint
  const nftKeypair = Keypair.generate();
  const nftMint = nftKeypair.publicKey;
  const destination = getAssociatedTokenAddressSync(
    nftMint,
    provider.wallet.publicKey
  );

  // Derive metadata and master edition accounts
  const metadata = await getMetadata(nftMint);
  const masterEdition = await getMasterEdition(nftMint);

  const nftClassBytes = Buffer.alloc(1); // Adjust size for u8/u16/u32
  nftClassBytes.writeUint8(NFT_CLASS, 0);
  const [nftInfoAccount, _bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("nft_data"), nftClassBytes],
    programId
  );

  // Invoke
  await program.methods
    .mintNft(NFT_CLASS)
    .accountsPartial({
      admin: new PublicKey(ADMIN),
      mint: nftMint,
      metadata,
      masterEdition,
      destination,
      collectionMint: new PublicKey(MINT_COLLECTION_ACCOUNT),
      nftInfoAccount,
    })
    .signers([nftKeypair])
    .rpc();

  console.log("NFT minted successfully!");
};

main().catch((err) => {
  console.error(err);
});
