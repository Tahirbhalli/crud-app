import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CrudApp } from "../target/types/crud_app";
import { assert } from "chai";

describe("crud-app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CrudApp as Program<CrudApp>;

  // Generate a test keypair for the journal owner
  const owner = anchor.web3.Keypair.generate();
  const title = "My First Journal";
  const message = "This is my first journal entry";

  // Derive the PDA for the journal entry
  const [journalEntryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(title), owner.publicKey.toBuffer()],
    program.programId
  );

  it("Creates a journal entry", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(owner.publicKey, 1_000_000_000),
      "confirmed"
    );

    const tx = await program.methods
      .createJournalEntry(title, message)
      .accounts({
        owner: owner.publicKey,
        journalEntry: journalEntryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log("✅ Created Journal Entry. Tx:", tx);

    const entry = await program.account.journalEntryState.fetch(journalEntryPda);
    assert.equal(entry.title, title, "Title should match");
    assert.equal(entry.message, message, "Message should match");
    assert.equal(entry.owner.toBase58(), owner.publicKey.toBase58(), "Owner should match");
  });

  it("Searches for a journal entry", async () => {
    const entry = await program.account.journalEntryState.fetch(journalEntryPda);
    console.log("📖 Found Journal Entry:", entry);
    assert.exists(entry, "Entry should exist");
  });

  it("Updates a journal entry", async () => {
    const updatedMessage = "Updated journal content";

    const tx = await program.methods
      .updateJournalEntry(title, updatedMessage)
      .accounts({
        owner: owner.publicKey,
        journalEntry: journalEntryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log("✏️ Updated Journal Entry. Tx:", tx);

    const entry = await program.account.journalEntryState.fetch(journalEntryPda);
    assert.equal(entry.message, updatedMessage, "Message should be updated");
  });

  it("Deletes a journal entry", async () => {
    const tx = await program.methods
      .deleteJournalEntry(title)
      .accounts({
        owner: owner.publicKey,
        journalEntry: journalEntryPda,
      })
      .signers([owner])
      .rpc();

    console.log("🗑️ Deleted Journal Entry. Tx:", tx);

    try {
      await program.account.journalEntryState.fetch(journalEntryPda);
      assert.fail("Entry should not exist after deletion");
    } catch (error) {
      assert.include(error.message, "Account does not exist", "Entry should be deleted");
    }
  });
});
