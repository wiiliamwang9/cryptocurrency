use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use savings::program::SavingsAccount;

#[tokio::test]
async fn test_initialize_and_deposit() {
    let program_test = ProgramTest::new(
        "savings",
        id(),
        processor!(savings::processor::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let savings_account = Keypair::new();
    let amount: u64 = 1000;

    let tx = Transaction::new_signed_with_payer(
        &[initialize_savings_account(&payer.pubkey(), &savings_account.pubkey(), amount)],
        Some(&payer.pubkey()),
        &[&payer, &savings_account],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    // Check if balance is correctly initialized
    let account_data = banks_client.get_account_data(&savings_account.pubkey()).await.unwrap();
    let savings_account = SavingsAccount::try_from_slice(&account_data).unwrap();
    assert_eq!(savings_account.balance, amount);
}

fn initialize_savings_account(owner: &Pubkey, savings_account: &Pubkey, amount: u64) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*savings_account, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let data = SavingsInstruction::Initialize { amount }.pack();
    Instruction {
        program_id: savings::ID,
        accounts,
        data,
    }
}
