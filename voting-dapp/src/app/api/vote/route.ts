import { ActionGetResponse, ActionPostRequest, ACTIONS_CORS_HEADERS, createPostResponse } from '@solana/actions';
import { ACTION } from 'next/dist/client/components/app-router-headers';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { Voting } from 'anchor/target/types/voting';
import { Program } from '@coral-xyz/anchor';
import BN from 'bn.js';

const IDL = require('@/../anchor/target/idl/voting.json');

export const OPTIONS = GET
export async function GET(request: Request) {
  const actionMetadata: ActionGetResponse = {
    title: 'Peanut Butter Voting',
    icon: 'https://t3.ftcdn.net/jpg/01/71/54/62/360_F_171546265_DV8E3F9ObHCvPjOWN6Mhr08DXwKrybR1.jpg',
    description: 'Vote between Crunchy and Smooth peanut butter',
    label: 'Vote',
    links: {
      actions: [
        {
          label: 'Vote Crunchy',
          href: '/api/vote?candidate=crunchy',
          type:'external-link',
        },
        {
          label: 'Vote Smooth',
          href: '/api/vote?candidate=smooth',
          type:'external-link',
          
        }
      ]
    }
  };
  
  return Response.json(actionMetadata, {headers: ACTIONS_CORS_HEADERS});
}


export async function POST(request: Request) {

  const url = new URL(request.url);
  const candidate = url.searchParams.get('candidate');
  if (!candidate) {
    return new Response('Missing candidate parameter', {status: 400});
  }
  if(candidate !== 'crunchy' && candidate !== 'smooth') {
    return new Response('Invalid candidate parameter', {status: 400});
   }
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  const program : Program<Voting> = new Program(IDL, {connection});
  const body : ActionPostRequest = await request.json();
  let voter = new PublicKey(body.account);

    try{
      voter = new PublicKey(body.account);

    }
    catch(e){   
      return new Response('Invalid account parameter', {status: 400,headers: ACTIONS_CORS_HEADERS});
    }

    const pollId = new BN(1); // Replace with actual pollId
    const instruction = await program.methods
    .vote(candidate, pollId)
    .accounts({
      signer: voter,
    })  
    .instruction();

    const blockhash = await connection.getLatestBlockhash();
    const transaction = new Transaction(
      {
        feePayer: voter,
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight,
      }).add(instruction);

    const response = await createPostResponse({
     fields: {
        type: 'transaction',
        transaction: transaction
      }
 } );
    
  return Response.json(response, {headers: ACTIONS_CORS_HEADERS});
}
