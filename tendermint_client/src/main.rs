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

    let abci_info = client.abci_info().await.unwrap();
    println!("Got ABCI info: {:?}", abci_info);

    // let tx_hex =
    //     hex::decode("042D2C79D9E16D1F6F78236D51E8E807ADEC94F90BB7749C0ABF7867E39BBDBE").unwrap();
    // let found_tx = client
    //     .tx(Hash::Sha256(tx_hex.try_into().unwrap()), true)
    //     .await
    //     .unwrap();
    // println!("found_tx: {:?}", Tx::from_bytes(&found_tx.tx).unwrap());

    let tx_data = hex::decode(String::from("0aae070a94070a282f616b6173682e636572742e763162657461332e4d7367437265617465436572746966696361746512e7060a2c616b617368316335666e6b66717135796e3766656d7a3936306d373077306561346a3275726179646468616c12fb042d2d2d2d2d424547494e2043455254494649434154452d2d2d2d2d0a4d49494271444343415536674177494241674955516d696a73356c476e4c357751444f6d3778685a4a65546452596377436759494b6f5a497a6a3045417749770a4e7a45314d444d4741315545417777735957746863326778597a566d626d746d63584531655734335a6d5674656a6b324d4730334d4863775a574530616a4a310a636d46355a47526f595777774868634e4d6a51774d5445344d5441784f5455325768634e4d6a55774d5445334d5441784f545532576a41334d5455774d7759440a56515144444378686132467a6144466a4e575a7561325a7863545635626a646d5a5731364f54597762546377647a426c595452714d6e567959586c6b5a4768680a6244425a4d424d4742797147534d34394167454743437147534d34394177454841304941424973636d756c7441554144434c45626261506a77344b3464316b5a0a3531336a4b42384a4178473546666c675479714b3146616e46574665454f6b78346733502b41722b633567742f6239496554775231614972424d6d6a4f4441320a4d41384741315564457745422f7751464d414d424166387744675944565230504151482f42415144416751774d424d47413155644a51514d4d416f47434373470a4151554642774d434d416f4743437147534d343942414d43413067414d4555434948356d525375525a55687156686b4674776972794f7651696170554f5261740a5173672b3842644875645430416945417a6771434f6a4f696b703466704c6a4d62314c6171366d6b62376b6b592f5053396d66556449776f464a513d0a2d2d2d2d2d454e442043455254494649434154452d2d2d2d2d0a1ab8012d2d2d2d2d424547494e204543205055424c4943204b45592d2d2d2d2d0a4d466b77457759484b6f5a497a6a3043415159494b6f5a497a6a30444151634451674145697879613657304251414d49735274746f2b50446772683357526e6e0a58654d6f48776b4445626b562b5742504b6f725556716356595634513654486944632f344376357a6d43333976306835504248566f69734579513d3d0a2d2d2d2d2d454e44204543205055424c4943204b45592d2d2d2d2d0a1215637265617465642066726f6d2063616e697374657212660a500a460a1f2f636f736d6f732e63727970746f2e736563703235366b312e5075624b657912230a2103886c002d0286785a3359a783915a947ab43b00b32e1c7d23d43a1970479a810812040a020801180112120a0c0a0475616b7412043330303010a08d061a40e394dd6669973f9e4edc5cf798ba79fa8d33873dd49e92df031219e5c75184f24dd45b8463e0a425526cd6294c73ff523ee158ac0bf64e99f2ce76d4607eaaea")).unwrap();
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
