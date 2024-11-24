import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PigeonBattle } from "../target/types/pigeon_battle";
import { solToLamports } from "./utils/helpers";
import { PROGRAM_ID } from "./utils/constants";
import { IDL } from "./utils/idl";

const classPrices: {
  [key: number]: {
    solPrice: string;
    boostAttack: number;
    boostDefense: number;
    boostSpeed: number;
  };
} = {
  // Class
  8: { solPrice: "1", boostAttack: 3, boostDefense: 0, boostSpeed: 0 },
  6: { solPrice: "1", boostAttack: 0, boostDefense: 3, boostSpeed: 0 },
  5: { solPrice: "1", boostAttack: 0, boostDefense: 0, boostSpeed: 3 },
  7: { solPrice: "1", boostAttack: 1, boostDefense: 1, boostSpeed: 1 },
  9: { solPrice: "0", boostAttack: 0, boostDefense: 0, boostSpeed: 0 },
  37: { solPrice: "5", boostAttack: 5, boostDefense: 0, boostSpeed: 0 },
  38: { solPrice: "5", boostAttack: 5, boostDefense: 0, boostSpeed: 0 },
  39: { solPrice: "5", boostAttack: 5, boostDefense: 0, boostSpeed: 0 },
  40: { solPrice: "5", boostAttack: 0, boostDefense: 5, boostSpeed: 0 },
  41: { solPrice: "5", boostAttack: 0, boostDefense: 5, boostSpeed: 0 },
  42: { solPrice: "5", boostAttack: 0, boostDefense: 0, boostSpeed: 5 },
  43: { solPrice: "5", boostAttack: 0, boostDefense: 0, boostSpeed: 5 },
  44: { solPrice: "20", boostAttack: 3, boostDefense: 3, boostSpeed: 3 },
};

const itemPrices: {
  [key: number]: {
    solPrice: string;
    healHp: number;
    healEnergy: number;
    boostAttack: number;
    boostDefense: number;
    boostSpeed: number;
  };
} = {
  // Item
  45: {
    solPrice: "0.14",
    healHp: 100,
    healEnergy: 100,
    boostAttack: 0,
    boostDefense: 0,
    boostSpeed: 0,
  },
  46: {
    solPrice: "0.05",
    healHp: 50,
    healEnergy: 0,
    boostAttack: 0,
    boostDefense: 0,
    boostSpeed: 0,
  },
  47: {
    solPrice: "0.08",
    healHp: 100,
    healEnergy: 0,
    boostAttack: 0,
    boostDefense: 0,
    boostSpeed: 0,
  },
  48: {
    solPrice: "0.05",
    healHp: 0,
    healEnergy: 50,
    boostAttack: 0,
    boostDefense: 0,
    boostSpeed: 0,
  },
  49: {
    solPrice: "0.08",
    healHp: 0,
    healEnergy: 100,
    boostAttack: 0,
    boostDefense: 0,
    boostSpeed: 0,
  },
};

const main = async () => {
  // Configure the provider to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("User wallet public key:", provider.wallet.publicKey.toBase58());

  // Load the IDL
  const programId = new PublicKey(PROGRAM_ID);
  const program = new anchor.Program<PigeonBattle>(IDL as any, provider);

  // Items Data
  // Fee: 0.0011 SOL
  for (const [key, value] of Object.entries(itemPrices)) {
    const itemId = parseInt(key);
    const itemClassBytes = Buffer.alloc(1); // Adjust size for u8/u16/u32
    itemClassBytes.writeUint8(itemId, 0);

    const [pda, _bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("item_data"), itemClassBytes],
      programId
    );

    await program.methods
      .setItemData(itemId, {
        price: solToLamports(value.solPrice),
        ...value,
      })
      .accountsPartial({
        itemClassInfo: pda,
      })
      .rpc();
  }

  // NFTs Data
  // Fee: 0.0011 SOL
  for (const [key, value] of Object.entries(classPrices)) {
    const classId = parseInt(key);
    const nftClassBytes = Buffer.alloc(1); // Adjust size for u8/u16/u32
    nftClassBytes.writeUint8(classId, 0);

    const [pda, _bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("nft_data"), nftClassBytes],
      programId
    );

    await program.methods
      .setNftData(classId, {
        price: solToLamports(value.solPrice),
        ...value,
      })
      .accountsPartial({
        nftClassInfo: pda,
      })
      .rpc();
  }

  console.log("Admin update Data success!");
};

main().catch((err) => {
  console.error(err);
});
