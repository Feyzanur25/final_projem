
use stellar_sdk::network::Network;
use stellar_sdk::transaction::Transaction;
use stellar_sdk::memo::Memo;
use stellar_sdk::operation::Operation;
use stellar_sdk::asset::Asset;
use stellar_sdk::account::Account;
use stellar_sdk::transaction_builder::TransactionBuilder;
use stellar_sdk::account_id::AccountId;
const HORIZON_URL: &str = "https://horizon-testnet.stellar.org";

#[derive(Serialize, Deserialize)]
struct PaymentRequest {
    from: String,
    to: String,
    amount: String,
    memo: Option<String>,
}

#[derive(Deserialize)]
struct HorizonResponse {
    hash: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let http_client = HttpClient::new();

    // Gönderen hesap bilgileri
    let from_secret = "SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"; // Özel anahtar
    let from_keypair = Keypair::from_secret_key(from_secret);

    // Alıcı hesap bilgileri
    let to_address = "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"; // Genel anahtar

    // Hesap bilgilerini Horizon'dan al
    let account_url = format!("{}/accounts/{}", HORIZON_URL, from_keypair.public_key());
    let response = http_client.get(&account_url).send().await?;
    let account_info: serde_json::Value = response.json().await?;
    let sequence_number = account_info["sequence"].as_str().unwrap().parse::<u64>()?;

    // İşlem oluştur
    let tx = TransactionBuilder::new()
        .with_source_account(Account::new(from_keypair.public_key(), sequence_number))
        .with_operation(Operation::Payment {
            destination: to_address.parse()?,
            asset: StellarAsset::native(),
            amount: "10".parse()?,
        })
        .with_memo(Memo::Text("Bu bir mesajdır".to_string()))
        .build()
        .sign(&from_keypair);

    // İşlemi Horizon'a gönder
    let tx_envelope = tx.to_xdr()?;
    let submit_url = format!("{}/transactions", HORIZON_URL);
    let submit_response = http_client.post(&submit_url)
        .json(&serde_json::json!({ "tx": tx_envelope }))
        .send()
        .await?;

    if submit_response.status().is_success() {
        println!("İşlem başarılı!");
    } else {
        println!("İşlem başarısız oldu: {:?}", submit_response.text().await?);
    }

    Ok(())
}
