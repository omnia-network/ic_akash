use base64::{engine::general_purpose::STANDARD, Engine as _};
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
use tendermint_rpc::{Client, HttpClient};

// from https://github.com/akash-network/cloudmos/blob/bb81e5076d3279806bb97759acc32dcae5790a85/deploy-web/src/utils/certificateUtils.ts#L34
fn generate_certificate(address: &str) -> Result<(X509, EcKey<Private>), String> {
    let group = EcGroup::from_curve_name(Nid::SECP256K1).map_err(|e| e.to_string())?;
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

    let abci_info = client.abci_info().await.unwrap();
    println!("Got ABCI info: {:?}", abci_info);

    // let tx_hex =
    //     hex::decode("56932C3FFFE36BC389B3E0078F113CB5EB8C1BD1C48FC9C359C0148D0BB0F0E9").unwrap();
    // let found_tx = client
    //     .tx(Hash::Sha256(tx_hex.try_into().unwrap()), true)
    //     .await
    //     .unwrap();
    // println!("found_tx: {:?}", found_tx.tx);

    let tx_data = hex::decode(String::from("0aac070a92070a282f616b6173682e636572742e763162657461332e4d7367437265617465436572746966696361746512e5060a2c616b617368316335666e6b66717135796e3766656d7a3936306d373077306561346a3275726179646468616c12fa040af7042d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d49494270544343415575674177494241674955466a6635712b6d4254695876696e664b6f6c6e6777442f59534e6377436759494b6f5a497a6a3045417749770a4e7a45314d444d4741315545417777735957746863326778597a566d626d746d63584531655734335a6d5674656a6b324d4730334d4863775a574530616a4a310a636d46355a47526f595777774868634e4d6a51774d5445334d6a41794d4445345768634e4d6a55774d5445324d6a41794d444534576a41334d5455774d7759440a56515144444378686132467a6144466a4e575a7561325a7863545635626a646d5a5731364f54597762546377647a426c595452714d6e567959586c6b5a4768680a624442574d42414742797147534d343941674547425375424241414b41304941424558664a726a5a6674354e44416b6537327463456e727942524477367943380a636266447a76466d7a6779634b32787875566c4e3169767370474d45563448415a4347615748356447344f515835585a6f74374c7749616a4f4441324d4138470a41315564457745422f7751464d414d424166387744675944565230504151482f42415144416751774d424d47413155644a51514d4d416f4743437347415155460a42774d434d416f4743437147534d343942414d43413067414d4555434951445a4e4f3243577659417277447871724d596a465a565a34396c7374414d76436b4a0a426a66684c75583862674967492f315a36664e4d37474c384c6d59755831673175773162574f3161486f57754d566849547765495172303d0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a1ab7010ab4012d2d2d2d2d424547494e204543205055424c4943204b45592d2d2d2d2d0a4d465977454159484b6f5a497a6a3043415159464b34454541416f44516741455264386d754e6c2b336b304d43523776613177536576494645504472494c78780a7438504f3857624f444a777262484735575533574b2b796b597752586763426b495a7059666c3062673542666c646d693373764168673d3d0a2d2d2d2d2d454e44204543205055424c4943204b45592d2d2d2d2d0a1215637265617465642066726f6d2063616e697374657212640a4e0a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a2103886c002d0286785a3359a783915a947ab43b00b32e1c7d23d43a1970479a810812040a02080112120a0c0a0475616b7412043330303010a08d061a406ca6fad2145eed7c15414c92bb668456701dc645d857ed5bcb7d88d35e7579ce710060244251a6852ec1d55d011e647fdd22ad3d0d0de8896ef58251100729cf")).unwrap();
    let broadcast_res = client.broadcast_tx_sync(tx_data).await.unwrap();
    println!("broadcast_res: {:?}", broadcast_res);

    // let address = "akash1c5fnkfqq5yn7femz960m70w0ea4j2urayddhal";

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
