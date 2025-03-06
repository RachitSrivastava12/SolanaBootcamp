import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { describe, it, before } from "node:test";
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Voting } from '../target/types/voting';
import { Keypair, PublicKey } from "@solana/web3.js";
import expect from "expect";

const IDL = require('../target/idl/voting.json');
const votingAddress = new PublicKey("96EH7YWE6M6jusxeVnGdRsQGJMYaZZsYZuxfjj6tmqs9");

  describe('Voting', () => {
    let provider ;
    let context;
    // anchor.setProvider(anchor.AnchorProvider.env());
    // let VotingProgram  =  anchor.workspace.Voting as Program<Voting>;
    let VotingProgram:any;
    
    before(async () => {
      context = await startAnchor("", [{name:"voting", programId: votingAddress}], []);
      provider = new BankrunProvider(context);
      VotingProgram = new Program<Voting>(IDL, provider);
      console.log("Program initialized with address:", votingAddress.toString());
    });
  
  it('Initialize the poll', async () => {
    console.log("\n--- INITIALIZING POLL ---");
    const tx = await VotingProgram.methods.initializePoll(
      new anchor.BN(1),
      "What is your favorite Peanut Butter?",
      new anchor.BN(0),
      new anchor.BN(1740663311),
    ).rpc();
    console.log("Poll initialized with transaction:", tx);
    
    const [pollAddress] = await PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8)],
      votingAddress,
    );
    console.log("Poll address:", pollAddress.toString());
    
    const poll = await VotingProgram.account.poll.fetch(pollAddress);
    console.log("Poll data:", {
      pollId: poll.pollId.toNumber(),
      description: poll.description,
      pollStart: poll.pollStart.toNumber(),
      pollEnd: poll.pollEnd.toNumber(),
      candidateAmount: poll.candidateAmount.toNumber()
    });
    
    expect(poll.pollId.toNumber()).toEqual(1);
    expect(poll.description).toEqual("What is your favorite Peanut Butter?");
    expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber());
  });
  
  it('Initialize the candidate', async () => {
    console.log("\n--- INITIALIZING CANDIDATE ---");
    // Note: Parameters are now in the correct order - candidate_name first, then poll_id
    const tx = await VotingProgram.methods.addCandidate(
      "Smooth Peanut Butter",
      new anchor.BN(1)
    ).rpc();

    const  bx = await VotingProgram.methods.addCandidate(
      "Crunchy Peanut Butter",
      new anchor.BN(1)
    ).rpc();
    console.log("Candidate1 initialized with transaction:", tx);
    console.log("Candidate2 initialized with transaction:", bx);
    
    // Add assertions for candidate verification
    const [smoothAddress] = await PublicKey.findProgramAddressSync(
      [
        new anchor.BN(1).toArrayLike(Buffer, 'le', 8),
        Buffer.from("Smooth Peanut Butter")
      ],
      votingAddress
    );
    const [crunchyAddress] = await PublicKey.findProgramAddressSync(
      [
        new anchor.BN(1).toArrayLike(Buffer, 'le', 8),
        Buffer.from("Crunchy Peanut Butter")
      ],
      votingAddress
    );
    console.log("Smooth Candidate address:", smoothAddress.toString());
    console.log("Crunchy Candidate address:", crunchyAddress.toString());
    
    const candidate1 = await VotingProgram.account.candidate.fetch(smoothAddress);
    
    expect(candidate1.candidateName).toEqual("Smooth Peanut Butter");
    expect(candidate1.candidateVotes.toNumber()).toEqual(new anchor.BN(0).toNumber());
    const candidate2 = await VotingProgram.account.candidate.fetch(crunchyAddress);
    
    expect(candidate2.candidateName).toEqual("Crunchy Peanut Butter");
    expect(candidate2.candidateVotes.toNumber()).toEqual(0);
    
    console.log(" Crunchy Peanut Butter:", {
      candidateName: candidate2.candidateName,
      candidateVotes: candidate2.candidateVotes.toNumber()
    });
    console.log(" Smooth Peanut Butter:", {
      candidateName: candidate1.candidateName,
      candidateVotes: candidate1.candidateVotes.toNumber()
    });

  });

  it('Vote', async () => {
    await VotingProgram.methods
    .vote(
       "Smooth Peanut Butter",
        new anchor.BN(1))
        .rpc();
        const [smoothAddress] = await PublicKey.findProgramAddressSync(
          [
            new anchor.BN(1).toArrayLike(Buffer, 'le', 8),
            Buffer.from("Smooth Peanut Butter")
          ],
          votingAddress
        );
        const candidate1 = await VotingProgram.account.candidate.fetch(smoothAddress);
        console.log(candidate1); 
        expect(candidate1.candidateVotes.toNumber()).toEqual(1);
  });
});