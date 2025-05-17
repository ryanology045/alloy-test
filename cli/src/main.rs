/*
----- Right now not a CLI at all, maybe later -----
*/

use alloy::primitives::{Address, U256}; 
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;

use std::error::Error;
use std::env; 
use dotenv::dotenv;
use tokio::time::{sleep, Duration};

sol! { 
    #[sol(rpc)] 
    contract Pinger { 
        function ping() public; 
        function poke(uint256 how_much_to_annoy) public; 
        function relieveStress() public; 
        function getAnnoyance() public view returns (uint256); 
    } 
} 
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();

    // Read the variables
    let rpc_url = env::var("RPC_URL").expect("No RPC? NGMI");
    let private_key = env::var("PRIVATE_KEY").expect("Where's your key?");

    let contract_addr: Address = env::var("CONTRACT_ADDR").expect("Go deploy something bruh").parse()?;

    // Create a provider with the variables
    let signer: PrivateKeySigner = private_key.parse()?;
    let provider = ProviderBuilder::new().wallet(signer).connect_http(rpc_url.parse()?); 

    // Setup contract instance.
    let contract = Pinger::new(contract_addr, provider.clone());

    // ----- READ INITIAL ANNOYANCE -----
    let val0 = contract.getAnnoyance().call().await?;
    println!("Annoyance value at: {}", val0);
    sleep(Duration::from_millis(1000)).await;

    // ----- PING -----
    let tx0 = contract.ping().send().await?;
    let receipt0 = tx0.get_receipt().await?;
    println!("ping() mined at block {}", receipt0.block_number.unwrap());

    // ----- READ ANNOYANCE AGAIN -----
    let val1 = contract.getAnnoyance().call().await?;
    println!("Annoyance value at: {}", val1);
    sleep(Duration::from_millis(1000)).await;

    // ----- POKE 5 TIMES -----
    let tx1 = contract.poke(U256::from(5)).send().await?;
    let receipt1 = tx1.get_receipt().await?;
    println!("poke(5) mined at block {}", receipt1.block_number.unwrap());

    // ----- READ ANNOYANCE AGAIN -----
    let val2 = contract.getAnnoyance().call().await?;
    println!("Annoyance value at: {}", val2);
    sleep(Duration::from_millis(1000)).await;

    // ----- PING -----
    let tx2 = contract.ping().send().await?;
    let receipt2 = tx2.get_receipt().await?;
    println!("ping() mined at block {}", receipt2.block_number.unwrap());

    // ----- READ ANNOYANCE AGAIN -----
    let val3 = contract.getAnnoyance().call().await?;
    println!("Annoyance value at: {}", val3);
    sleep(Duration::from_millis(1000)).await;

    // ----- RELIEVE STRESS -----
    let tx3 = contract.relieveStress().send().await?;
    let receipt3 = tx3.get_receipt().await?;
    println!("relieveStress() mined at block {}", receipt3.block_number.unwrap());

    // ----- READ FINAL ANNOYANCE -----
    let val4 = contract.getAnnoyance().call().await?;
    println!("Annoyance value at: {}", val4);
    sleep(Duration::from_millis(1000)).await;

    // ----- JUST ONE MORE PING FOR FUN -----
    let tx4 = contract.ping().send().await?;
    let receipt4 = tx4.get_receipt().await?;
    println!("ping() mined at block {}", receipt4.block_number.unwrap());

    // ----- READ FINAL FINAL ANNOYANCE -----
    let val5 = contract.getAnnoyance().call().await?;
    println!("Annoyance value at: {}", val5);

    // maybe will add event listening and proper transaction ordering later idk

    Ok(())
}