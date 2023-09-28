use near_sdk::{json_types::U128, serde_json::json, ONE_NEAR};
use tokio::join;
use workspaces::{network::Sandbox, Worker};

const WASM_WITHOUT_NEP145: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/without_nep145.wasm");
const WASM_WITH_NEP145: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/with_nep145.wasm");

#[tokio::main]
pub async fn main() {
    eprintln!("Initializing sandbox worker...");

    let worker: Worker<Sandbox> = workspaces::sandbox().await.unwrap();

    eprintln!("Initializing actors...");

    let (alice, bob, contract) = join!(
        async { worker.dev_create_account().await.unwrap() },
        async { worker.dev_create_account().await.unwrap() },
        async { worker.dev_deploy(WASM_WITHOUT_NEP145).await.unwrap() }
    );

    eprintln!("Calling contract::new()...");

    contract
        .call("new")
        .args_json(json!({}))
        .transact()
        .await
        .unwrap()
        .unwrap();

    eprintln!("Calling contract::mint(10)...");

    alice
        .call(contract.id(), "mint")
        .args_json(json!({
            "amount": "10",
        }))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let balance: U128 = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))
        .await
        .unwrap()
        .json()
        .unwrap();

    eprintln!("Balance of alice: {}", balance.0);

    assert_eq!(balance.0, 10);

    eprintln!("Deploying version with NEP-145...");

    contract
        .batch()
        .deploy(WASM_WITH_NEP145)
        .transact()
        .await
        .unwrap()
        .unwrap();

    let balance: U128 = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))
        .await
        .unwrap()
        .json()
        .unwrap();

    eprintln!("Balance of alice: {}", balance.0);

    assert_eq!(balance.0, 10);

    eprintln!("Registering bob with the contract...");

    bob.call(contract.id(), "storage_deposit")
        .args_json(json!({}))
        .deposit(ONE_NEAR)
        .transact()
        .await
        .unwrap()
        .unwrap();

    eprintln!("Transferring 3 tokens from alice to bob...");

    alice
        .call(contract.id(), "ft_transfer")
        .args_json(json!({
            "receiver_id": bob.id(),
            "amount": "3",
        }))
        .deposit(1)
        .transact()
        .await
        .unwrap()
        .unwrap();

    eprintln!("Finished transfer.");

    let balance: U128 = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": alice.id(),
        }))
        .await
        .unwrap()
        .json()
        .unwrap();

    eprintln!("Balance of alice: {}", balance.0);

    assert_eq!(balance.0, 7);

    let balance: U128 = contract
        .view("ft_balance_of")
        .args_json(json!({
            "account_id": bob.id(),
        }))
        .await
        .unwrap()
        .json()
        .unwrap();

    eprintln!("Balance of bob: {}", balance.0);

    assert_eq!(balance.0, 3);

    eprintln!("Done.");
}
