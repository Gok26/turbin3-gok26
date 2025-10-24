import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VisitorCounter } from "../target/types/visitor_counter";
import { assert } from "chai";


describe("visitor_counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);


  const program = anchor.workspace.visitorCounter as Program<VisitorCounter>;
   const authority = provider.wallet;
   let counterAccount = anchor.web3.Keypair.generate();
  it("Is initialized!", async () => {
    // Add your test here.
          await program.provider.connection.confirmTransaction(await program.provider.connection.requestAirdrop(counterAccount.publicKey, 100*anchor.web3.LAMPORTS_PER_SOL),"confirmed");

    const tx =     await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
        authority: authority.publicKey,
      })
      .signers([counterAccount])
      .rpc();
      const counterState = await program.account.counter.fetch(
      counterAccount.publicKey
    );
    assert.equal(counterState.count.toNumber(), 0);
    console.log("Your transaction signature", tx);
  });
   
    it("Increment the counter multiple times", async () => {
    for (let i = 0; i < 3; i++) {
      await program.methods
        .increment()
        .accounts({
          counter: counterAccount.publicKey,
          visitor: authority.publicKey,
        })
        .rpc();
    }

    const counterState = await program.account.counter.fetch(
      counterAccount.publicKey
    );
    assert.equal(counterState.count.toNumber(), 3);
  });

});
