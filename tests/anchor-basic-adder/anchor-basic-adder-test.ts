import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorBasicAdder } from "../../target/types/anchor_basic_adder";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { expect } from "chai";

describe("anchor-basic-adder", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorBasicAdder as Program<AnchorBasicAdder>;

  it("Add 5", async () => {
    const tx = await program.methods.add(5).rpc();

    const [counterPubkey] = await findProgramAddressSync([
      Buffer.from('COUNTER_TAG')
    ], program.programId)
    
    const counter = await program.account.counter.fetch(counterPubkey);

    expect(counter.number).to.equal(5);
  });
});
