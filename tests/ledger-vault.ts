import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { LedgerVault } from "../target/types/ledger_vault";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("ledger-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as NodeWallet;

  const program = anchor.workspace.ledgerVault as Program<LedgerVault>;

  let mint: anchor.web3.PublicKey;
  let userATA: anchor.web3.PublicKey;
  let vaultState: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;

  it("Creates a mint and initializes user ATA", async () => {
    // Create a new mint
    mint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      9 // Decimals
    );
    console.log("\nMint created:", mint.toBase58());

    // Initialize user's associated token account (ATA)
    let ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint,
      payer.publicKey
    );
    userATA = ATA.address;

    console.log("User ATA initialized:", userATA.toBase58());

    vaultState = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("vault"),
      provider.publicKey.toBuffer(),
      mint.toBuffer(),
    ], program.programId)[0];
    console.log("\nVault State Address:", vaultState.toBase58());
  
    vault = getAssociatedTokenAddressSync(
      mint,
      vaultState,
      true, // allow owner off curve
    );
  });


  it("Is initialized!", async () => {
  
    const tx = await program.methods.initialize().accountsPartial({
      user: provider.publicKey,
      mint,
      vaultState,
      vault,
      systemProgram: SYSTEM_PROGRAM_ID,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Mints tokens to user ATA", async () => {
    // Mint tokens to user's ATA
    const mintAmount = 100000000; // 100 token with 9 decimals
    await mintTo(
      provider.connection,
      payer.payer,
      mint,
      userATA,
      payer.publicKey,
      mintAmount
    );
    console.log(`\nMinted ${mintAmount} tokens to user ATA: ${userATA.toBase58()}`);
  });

  it("Deposits tokens into the vault", async () => {

    let initalBalance = await provider.connection.getTokenAccountBalance(vault);
    console.log("\nVault balance before deposit:", initalBalance.value.amount);

    const tx = await program.methods.deposit(new anchor.BN(10000000)) // Deposit 10 tokens
      .accountsPartial({
        user: provider.publicKey,
        mint,
        vaultState,
        vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("\nDeposit transaction signature", tx);
    
    let finalBalance = await provider.connection.getTokenAccountBalance(vault);
    console.log("\nVault balance after deposit:", finalBalance.value.amount);
  });

  it("Withdraws tokens from the vault", async () => {
    let initialVaultBalance = await provider.connection.getTokenAccountBalance(vault);
    console.log("\nVault balance before withdrawal:", initialVaultBalance.value.amount);

    const tx = await program.methods.withdraw()
      .accountsPartial({
        user: provider.publicKey,
        mint,
        vaultState,
        vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("\nWithdrawal transaction signature", tx);

    try {
      let finalVaultBalance = await provider.connection.getTokenAccountBalance(vault);
      console.log("\nVault balance after withdrawal:", finalVaultBalance.value.amount);
    } catch (error) {
      console.error(error.message);
    }
  });
});
