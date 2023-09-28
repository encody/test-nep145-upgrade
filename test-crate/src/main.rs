use near_sdk::serde_json::json;
use tokio::join;
use workspaces::{network::Sandbox, Worker};

const WASM: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/without_nep145.wasm");

#[tokio::main]
pub async fn main() {
    eprintln!("Initializing sandbox worker...");

    let worker: Worker<Sandbox> = workspaces::sandbox().await.unwrap();

    eprintln!("Initializing actors...");

    let (alice, _bob, contract) = join!(
        async { worker.dev_create_account().await.unwrap() },
        async { worker.dev_create_account().await.unwrap() },
        async { worker.dev_deploy(WASM).await.unwrap() }
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
            "amount": "10"
        }))
        .transact()
        .await
        .unwrap()
        .unwrap();

    eprintln!("Done.");
}
