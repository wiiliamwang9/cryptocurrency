import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@project-serum/anchor';
import idl from './idl.json';

const network = clusterApiUrl('devnet');
const connection = new Connection(network, 'processed');
const wallet = new web3.Keypair(); // Replace with a real wallet

const provider = new AnchorProvider(connection, wallet, {
    commitment: 'processed',
});
const program = new Program(idl, 'YourProgramID', provider);

async function depositFunds(amount: number) {
    const savingsAccount = new PublicKey('11111111111111111111111111111111');
    await program.methods.deposit(new anchor.BN(amount)).accounts({
        savingsAccount,
        user: wallet.publicKey,
    }).rpc();
}

depositFunds(1000).then(() => console.log('Deposit successful'));
