use locky::{LockyClient, LockyEnv};

#[tokio::test]
async fn can_create_account() {
    let mut locky = LockyClient::new(LockyEnv::Staging);
    locky.create_account("sdk.test@getloc.ky").await.unwrap();
}

#[tokio::test]
async fn can_make_key() {
    let mut locky = LockyClient::new(LockyEnv::Staging);
    locky.create_account("sdk.test+2@getloc.ky").await.unwrap();
    locky.create_key("test_key_1").await.unwrap();
    let key = locky.get_key("test_key_1").await.unwrap();
    drop(key);
}

#[tokio::test]
async fn can_log_back_in() {
    let mut locky = LockyClient::new(LockyEnv::Staging);
    let account_id = locky.create_account("sdk.test+3@getloc.ky").await.unwrap();
    locky.create_key("test_key").await.unwrap();
    let key1 = locky.get_key("test_key").await.unwrap();
    let access_token = locky.get_access_token().unwrap();
    drop(locky);

    let mut locky = LockyClient::new(LockyEnv::Staging).with_creds(account_id, access_token);
    let key2 = locky.get_key("test_key").await.unwrap();
    assert_eq!(key1, key2);
}
