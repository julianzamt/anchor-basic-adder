import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorBasicAdder } from "../../target/types/anchor_basic_adder";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { expect } from "chai";

describe("anchor-basic-adder", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorBasicAdder as Program<AnchorBasicAdder>;

  it("Add & Double Test", async () => {
    const [counterPubkey] = await findProgramAddressSync([
      Buffer.from('counter')
    ], program.programId)
    
    await program.methods.add(500).accounts({
      counter: counterPubkey
    }).rpc();

    const counterAfterAdd = await program.account.counter.fetch(counterPubkey);

    expect(counterAfterAdd.number).to.equal(500);

    await program.methods.double().accounts({
      counter: counterPubkey
    }).rpc();

    const counterAfterDouble = await program.account.counter.fetch(counterPubkey);

    expect(counterAfterDouble.number).to.equal(1000);
  });
});
