use deribit::models::{AuthRequest, Currency, GetTransfersRequest};
use deribit::{DeribitBuilder, DeribitError};
use dotenv::dotenv;
use fehler::throws;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
#[throws(DeribitError)]
fn get_transfers() {
    let _ = dotenv();
    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let mut rt = Runtime::new().expect("cannot create tokio runtime");

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        let req = GetTransfersRequest::with_currency(Currency::BTC);
        Ok::<_, DeribitError>(client.call(req).await?.await?)
    };
    let _ = rt.block_on(fut)?;
}
