use allocator;
use anchor_client::anchor_lang::{InstructionData, ToAccountMetas};
use anchor_client::solana_sdk::entrypoint::ProgramResult;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::transaction::Transaction;
use anchor_client::solana_sdk::{signature::Keypair, signer::Signer};
use anchor_lang::prelude::{AccountInfo, Pubkey};
use solana_program_test::*;

fn entry(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
	let accounts = Box::leak(Box::new(accounts.to_vec()));
	allocator::entry(program_id, accounts, instruction_data)
}


#[tokio::test]
pub async fn test_heap_allocate() {
    let program_test = ProgramTest::new("allocator", allocator::ID, processor!(entry));

    let (mut banks_client, payer, _) = program_test.start().await;

    // Heap Allocate = 1KB
    heap_allocate_test(&mut banks_client, &payer, 1024).await;

    // Heap Allocate = 10KB
    heap_allocate_test(&mut banks_client, &payer, 10 * 1024).await;

    // Heap Allocate = 100KB
    heap_allocate_test(&mut banks_client, &payer, 100 * 1024).await;
}

#[tokio::test]
async fn test_bump_allocate() {
    let program_test = ProgramTest::new("allocator", allocator::ID, processor!(entry));

    let (mut banks_client, payer, _) = program_test.start().await;

    // Bump Allocate = 1KB
    bump_allocate_test(&mut banks_client, &payer, 1024).await;

    // Bump Allocate = 10KB
    bump_allocate_test(&mut banks_client, &payer, 10 * 1024).await;

    // Bump Allocate = 100KB
    bump_allocate_test(&mut banks_client, &payer, 100 * 1024).await;
}

pub async fn heap_allocate_test(banks_client: &mut BanksClient, payer: &Keypair, size: u64) {
    let ins: Instruction = Instruction {
        program_id: allocator::ID,
        accounts: allocator::accounts::HeapAllocateCtx {}.to_account_metas(None),
        data: allocator::instruction::HeapAllocate { size }.data(),
    };

    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[ins],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let bank_tx_result = banks_client.simulate_transaction(tx).await.unwrap();
    // Extract compute units
    let compute_units = bank_tx_result.simulation_details.unwrap().units_consumed;
    println!(
        "Heap Allocate {} bytes consumed {} compute units",
        size, compute_units
    );
}

pub async fn bump_allocate_test(banks_client: &mut BanksClient, payer: &Keypair, size: u64) {
    let ins: Instruction = Instruction {
        program_id: allocator::ID,
        accounts: allocator::accounts::CustomizeAllocateCtx {}.to_account_metas(None),
        data: allocator::instruction::CustomizeAllocate { size }.data(),
    };

    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[ins],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let bank_tx_result = banks_client.simulate_transaction(tx).await.unwrap();

    // Extract compute units
    let compute_units = bank_tx_result.simulation_details.unwrap().units_consumed;
    println!(
        "Bump Allocate {} bytes consumed {} compute units",
        size, compute_units
    );
}
