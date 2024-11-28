import { BankrunProvider, startAnchor } from "anchor-bankrun";
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { Votingdapp } from '../target/types/votingdapp';

const IDL = require('../target/idl/votingdapp.json');
const votingAddress = new PublicKey("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

describe('votingdapp', () => {
  it('Initialize Poll', async () => {
    // 1. Updated 'programID' to 'programId' to match the expected property name.
    const context = await startAnchor("", [{ name: "votingdapp", programId: votingAddress }], []);

    // 2. Using BankrunProvider with the correct context.
    const provider = new BankrunProvider(context);
    const votingProgram = new Program<Votingdapp>(IDL, provider);

    // 3. Setting an extended timeout for the test to avoid timeout issues (10 seconds).
    await votingProgram.methods.initializePoll(
      new anchor.BN(1),
      "is this working",
      new anchor.BN(0),
      new anchor.BN(1730860890),
    ).rpc();
    const [pollAddress] = PublicKey.findProgramAddressSync(new anchor.BN(1))
  }, 10000); // Custom timeout set to 10 seconds
});
