import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Crudapp } from '../target/types/Crudapp'

describe('Crudapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Crudapp as Program<Crudapp>

  const CrudappKeypair = Keypair.generate()

  it('Initialize Crudapp', async () => {
    await program.methods
      .initialize()
      .accounts({
        Crudapp: CrudappKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([CrudappKeypair])
      .rpc()

    const currentCount = await program.account.Crudapp.fetch(CrudappKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Crudapp', async () => {
    await program.methods.increment().accounts({ Crudapp: CrudappKeypair.publicKey }).rpc()

    const currentCount = await program.account.Crudapp.fetch(CrudappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Crudapp Again', async () => {
    await program.methods.increment().accounts({ Crudapp: CrudappKeypair.publicKey }).rpc()

    const currentCount = await program.account.Crudapp.fetch(CrudappKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Crudapp', async () => {
    await program.methods.decrement().accounts({ Crudapp: CrudappKeypair.publicKey }).rpc()

    const currentCount = await program.account.Crudapp.fetch(CrudappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set Crudapp value', async () => {
    await program.methods.set(42).accounts({ Crudapp: CrudappKeypair.publicKey }).rpc()

    const currentCount = await program.account.Crudapp.fetch(CrudappKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the Crudapp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        Crudapp: CrudappKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.Crudapp.fetchNullable(CrudappKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
