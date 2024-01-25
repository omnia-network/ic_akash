use base64::{engine::general_purpose::STANDARD, Engine as _};
use cosmrs::tx::Tx;
use openssl::{
    asn1::Asn1Time,
    bn::{BigNum, MsbOption},
    ec::{EcGroup, EcKey},
    hash::MessageDigest,
    nid::Nid,
    pkey::{PKey, Private},
    x509::{
        extension::{BasicConstraints, ExtendedKeyUsage, KeyUsage},
        X509Builder, X509NameBuilder, X509,
    },
};
use tendermint::Hash;
use tendermint_rpc::{Client, HttpClient};

// from https://github.com/akash-network/cloudmos/blob/bb81e5076d3279806bb97759acc32dcae5790a85/deploy-web/src/utils/certificateUtils.ts#L34
fn generate_certificate(address: &str) -> Result<(X509, EcKey<Private>), String> {
    // we need to use secp256r1, which is named X9_62_PRIME256V1 in openssl
    // see https://www.ietf.org/rfc/rfc5480.txt
    // and https://github.com/openssl/openssl/blob/ead44e19fa3ff7d189876081880f1adb3dfdf30b/apps/ecparam.c#L215-L219
    let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).map_err(|e| e.to_string())?;
    let ecdsa = EcKey::generate(&group).map_err(|e| e.to_string())?;
    let key_pair = PKey::from_ec_key(ecdsa.clone()).map_err(|e| e.to_string())?;
    let mut cert_builder =
        X509Builder::new().map_err(|e| format!("failed to create certificate: {:?}", e))?;
    let serial_number = {
        let mut serial = BigNum::new().map_err(|e| e.to_string())?;
        serial
            .rand(159, MsbOption::MAYBE_ZERO, false)
            .map_err(|e| e.to_string())?;
        serial.to_asn1_integer().map_err(|e| e.to_string())?
    };
    cert_builder
        .set_serial_number(&serial_number)
        .map_err(|e| e.to_string())?;
    let mut x509_name = X509NameBuilder::new().map_err(|e| e.to_string())?;
    x509_name
        .append_entry_by_text("CN", address)
        .map_err(|e| e.to_string())?;
    let x509_name = x509_name.build();
    cert_builder
        .set_issuer_name(&x509_name)
        .map_err(|e| e.to_string())?;
    cert_builder
        .set_subject_name(&x509_name)
        .map_err(|e| e.to_string())?;
    let not_before = Asn1Time::days_from_now(0).map_err(|e| e.to_string())?;
    cert_builder
        .set_not_before(&not_before)
        .map_err(|e| e.to_string())?;
    let not_after = Asn1Time::days_from_now(365).map_err(|e| e.to_string())?;
    cert_builder
        .set_not_after(&not_after)
        .map_err(|e| e.to_string())?;

    cert_builder
        .set_pubkey(&key_pair)
        .map_err(|e| e.to_string())?;

    cert_builder
        .append_extension(BasicConstraints::new().critical().ca().build().unwrap())
        .unwrap();
    cert_builder
        .append_extension(
            KeyUsage::new()
                .critical()
                .key_encipherment()
                .data_encipherment()
                .build()
                .unwrap(),
        )
        .unwrap();
    cert_builder
        .append_extension(ExtendedKeyUsage::new().client_auth().build().unwrap())
        .unwrap();

    cert_builder
        .sign(&key_pair, MessageDigest::sha256())
        .map_err(|e| e.to_string())?;
    let cert = cert_builder.build();

    Ok((cert, ecdsa))
}

#[tokio::main]
async fn main() {
    let client = HttpClient::new("https://rpc.sandbox-01.aksh.pw:443").unwrap();

    let address = "akash1435dj4zjfz59rux9akthdcf6cy7h693fte6ge2";

    let abci_info = client.abci_info().await.unwrap();
    println!("Got ABCI info: {:?}", abci_info);

    let latest_block = client.latest_block().await.unwrap();
    println!(
        "Got latest block height: {:?}",
        latest_block.block.header.height
    );

    let query_res = client
        .abci_query(
            Some(String::from("/cosmos.auth.v1beta1.Query/Account")),
            hex::decode(String::from("0a2c616b617368317a686436373538673874686c7363726e347532306879783770347a3978616b6678767a703037")).unwrap(),
            None,
            false,
        )
        .await
        .unwrap();
    println!("account response (hex): {}", hex::encode(query_res.value));

    // let tx_hex =
    //     hex::decode("042D2C79D9E16D1F6F78236D51E8E807ADEC94F90BB7749C0ABF7867E39BBDBE").unwrap();
    // let found_tx = client
    //     .tx(Hash::Sha256(tx_hex.try_into().unwrap()), true)
    //     .await
    //     .unwrap();
    // println!("found_tx: {:?}", Tx::from_bytes(&found_tx.tx).unwrap());

    let tx_data = hex::decode(String::from("0a7e0a650a2c2f616b6173682e6465706c6f796d656e742e763162657461332e4d7367436c6f73654465706c6f796d656e7412350a330a2c616b61736831343335646a347a6a667a353972757839616b746864636636637937683639336674653667653210d2a3b6011215637265617465642066726f6d2063616e697374657212670a500a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a2103e050e27a933f56115d690a4c27fd058b50b89f6c4e996453f0c314618f63cafc12040a020801180d12130a0d0a0475616b74120532303030301080ea301a40049bc68e31d167504a7b2c41182014c8098fa95b0ce8ebd58c1884a394575f3076de957e9da306ca5b40ae015f45eaf15284f3e13817cb691f5c26ee378694c6")).unwrap();
    let broadcast_res = client.broadcast_tx_sync(tx_data).await.unwrap();
    println!("broadcast_res: {:?}", broadcast_res);

    // let query_res = client
    //     .abci_query(
    //         Some(String::from("/akash.market.v1beta4.Query/Bids")),
    //         // hex::decode(String::from("0a330a2c616b61736831343335646a347a6a667a353972757839616b746864636636637937683639336674653667653210f6b7b501")).unwrap(),
    //         hex::decode(String::from("0a330a2c616b61736831343335646a347a6a667a353972757839616b746864636636637937683639336674653667653210d2a3b601")).unwrap(),
    //         None,
    //         false,
    //     )
    //     .await
    //     .unwrap();
    // println!("bids (query_res): {:?}", query_res);

    // let (cert, key_pair) = generate_certificate(address).unwrap();
    // let cert_pem = cert.to_pem().unwrap();
    // let pub_key_pem = key_pair.public_key_to_pem().unwrap();
    // let pub_key_pem = String::from_utf8(pub_key_pem.clone())
    //     .unwrap()
    //     .replace("PUBLIC KEY", "EC PUBLIC KEY");
    // let pub_key_pem = pub_key_pem.as_bytes();
    // println!("Cert PEM: {}", String::from_utf8(cert_pem.clone()).unwrap());
    // println!(
    //     "Pub Key PEM: {}",
    //     String::from_utf8(pub_key_pem.to_vec()).unwrap()
    // );

    // let cert_pem_base64 = STANDARD.encode(cert_pem);
    // println!("Cert PEM Base64: {cert_pem_base64}");
    // let pub_key_pem_base64 = STANDARD.encode(pub_key_pem);
    // println!("Pub Key PEM Base64: {pub_key_pem_base64}");
}
