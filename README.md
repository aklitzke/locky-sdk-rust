# Locky Rust SDK

🧩 [Locky](https://getloc.ky) is a cloud-based key-management system specifically focused on preventing
harvest-now decrypt-later quantum attacks.

**Standardized**: Utilizes NIST-approved post-quantum cryptographic parameters and algorithms (FIPS-203)<br>
**Fast**: Keys are retrieved in under 50ms<br>
**Secure**: Root keys are 'split up' so that if an entire datacenter's data were compromised, no keys would be exposed<br>
**Reliable**: No downtime even facing a catastrophic datacenter loss<br>
**Flexible**: Locky can be used to store keys for AES, ChaCha20, Ascon, and most other data encryption algorithms<br>

[Rust Docs](https://docs.rs/locky/latest/locky/)

[Locky Website](https://getloc.ky)

[Locky Documentation](https://gitbook.getloc.ky)

# Example
Retrieving a key from Locky
```
# use locky::{LockyClient, LockyEnv};
# use aes_gcm::{
#       aead::{Aead, AeadCore, KeyInit, OsRng},
#       Aes256Gcm, Nonce, Key
# };
# tokio_test::block_on(async {
# // make an account for testing
# let (account_id, access_token) = locky::get_test_account().await;
// Connect to Locky staging environment.
let mut client = LockyClient::new(LockyEnv::Staging)
    .with_creds(account_id, access_token);
# client.create_key("test_db_key").await.unwrap();

// Securely get a secret from the cloud service
let key = client.get_key("test_db_key").await.unwrap();

// Use the secret to encrypt some data
let cipher = Aes256Gcm::new((&*key).into());

// Never send this key over a network. Even if the communication is encrypted,
// unless it specifially uses a post-quantum secure protocol (such as the one
// one used by Locky) it will vulnerable to harvest-now decrypt-later
// attacks.
drop(key);
# });
```

## Creating an account
```
# use locky::{LockyClient, LockyEnv};
# tokio_test::block_on(async {
let mut client = LockyClient::new(LockyEnv::Staging);

// Make an account in our staging environment
let account_id = client.create_account("cool-test-account@getloc.ky").await.unwrap();

// the access token needs to be stored securely, but it does not need
// to be stored in a quantum-secure manner. So however you currently
// manage secrets is probably fine!
let access_token = client.get_access_token().unwrap();
# });
```

## Creating a key
```
# use locky::{LockyClient, LockyEnv};
# tokio_test::block_on(async {
# let (account_id, access_token) = locky::get_test_account().await;
let mut client = LockyClient::new(LockyEnv::Staging).with_creds(account_id, access_token);

// Alternately, you can use our CLI or web interface to create a key
client.create_key("test_key").await.unwrap();
# });
```

# A Note On Staging
The staging environment is **deleted every 24 hours**. It is a test environment.
Security is not guaranteed and any accounts, keys, or data you create
will be lost. Do not store anything in staging besides ephemeral test data!
