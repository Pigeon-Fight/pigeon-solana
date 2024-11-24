import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PigeonBattle } from "../target/types/pigeon_battle";

describe("pigeon-battle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.PigeonBattle as Program<PigeonBattle>;

  // Wallets
  const adminWallet: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const userWallet: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  it("Test!", async () => {});
});
