use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwkStorage;
use identity_iota::verification::jwk::EdCurve;
use identity_iota::verification::jwk::Jwk;
use identity_iota::verification::jwk::JwkParamsOkp;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::jwu;
use identity_stronghold::StrongholdStorage;
use iota_identity_example::random_stronghold_path;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::Password;
use iota_stronghold::procedures::KeyType;
use iota_stronghold::procedures::StrongholdProcedure;
use iota_stronghold::Client;
use iota_stronghold::ClientError;
use iota_stronghold::Location;
use iota_stronghold::Stronghold;

static IDENTITY_VAULT_PATH: &str = "iota_identity_vault";
pub(crate) static IDENTITY_CLIENT_PATH: &[u8] = b"iota_identity_client";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = random_stronghold_path();

    let password = Password::from("secure_password".to_owned());

    let stronghold = StrongholdSecretManager::builder()
        .password(password.clone())
        .build(path.clone())?;

    let stronghold_storage = StrongholdStorage::new(stronghold);

    let output = stronghold_storage
        .generate(JwkMemStore::ED25519_KEY_TYPE, JwsAlgorithm::EdDSA)
        .await
        .unwrap();

    let key_id = output.key_id;

    ////////////////////////////////////////////////////////////////////////
    // We have a key ID and we want to get the public key from stronghold.
    ////////////////////////////////////////////////////////////////////////

    let location = Location::generic(
        IDENTITY_VAULT_PATH.as_bytes().to_vec(),
        key_id.to_string().as_bytes().to_vec(),
    );

    let public_key_procedure = iota_stronghold::procedures::PublicKey {
        ty: KeyType::Ed25519,
        private_key: location,
    };

    let stronghold = StrongholdSecretManager::builder()
        .password(password.clone())
        .build(path.clone())?;

    let stronghold_inner = stronghold.inner().await;
    let client = get_client(&stronghold_inner);
    let procedure_result = client
        .execute_procedure(StrongholdProcedure::PublicKey(public_key_procedure))
        .unwrap();

    let public_key: Vec<u8> = procedure_result.into();

    let mut params = JwkParamsOkp::new();
    params.x = jwu::encode_b64(public_key);
    params.crv = EdCurve::Ed25519.name().to_owned();
    let mut jwk: Jwk = Jwk::from_params(params);
    jwk.set_alg(JwsAlgorithm::EdDSA.name());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    println!("{:?}", jwk);

    Ok(())
}

fn get_client(stronghold: &Stronghold) -> Client {
    let client = stronghold.get_client(IDENTITY_CLIENT_PATH);
    match client {
        Ok(client) => client,
        Err(ClientError::ClientDataNotPresent) => load_or_create_client(stronghold),
        Err(_) => panic!(""),
    }
}

fn load_or_create_client(stronghold: &Stronghold) -> Client {
    match stronghold.load_client(IDENTITY_CLIENT_PATH) {
        Ok(client) => client,
        Err(ClientError::ClientDataNotPresent) => {
            stronghold.create_client(IDENTITY_CLIENT_PATH).unwrap()
        }
        Err(_) => panic!(""),
    }
}
