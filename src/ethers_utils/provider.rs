use std::{str::FromStr, sync::Arc, thread, time::Duration};
use ethers::{prelude::*, core::k256::ecdsa::SigningKey};
use async_recursion::async_recursion;

pub async fn get_ws_provider(url: &str) -> Arc<Provider<Ws>> {
  let provider = _get_ws_provider_helper(url).await;
  provider
}

pub fn get_signer_middleware<M: Middleware>(provider: M, key: &str, chain_id: u64) -> SignerMiddleware<M, Wallet<SigningKey>> {
  let wallet = Wallet::from_str(key).unwrap().with_chain_id(chain_id);
  SignerMiddleware::new(provider, wallet)
}

#[async_recursion]
async fn _get_ws_provider_helper(url: &str) -> Arc<Provider<Ws>> {
  let result = Provider::<Ws>::connect(url).await;
  let provider = match result {
    Ok(provider) => {
      Arc::new(provider)
    },
    Err(_) => {
      println!("Error: Retrying to connect provider after 5 seconds...");
      thread::sleep(Duration::from_millis(5000));
      get_ws_provider(url).await
    }
  };
  provider
}

pub fn get_http_provider(url: &str) -> Arc<Provider<Http>> {
  Arc::new(Provider::<Http>::try_from(url).unwrap())
}

pub async fn get_latest_block<T: JsonRpcClient>(provider: Arc<Provider<T>>) -> u64 {
  provider.get_block(BlockNumber::Latest).await.unwrap().unwrap().number.unwrap().as_u64()
}
