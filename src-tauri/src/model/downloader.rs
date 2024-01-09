use chrono::Duration;
use std::{
    collections::HashMap,
    fs::{self, File},
};
use xmltree::{Element, XMLNode};

use crate::Result;

const XMLNS: &str = "http://www.w3.org/2000/xmlns";
const SECURED_URL: &str =
    "https://fe3.delivery.mp.microsoft.com/ClientWebService/client.asmx/secured";
const SOAP: &str = "http://www.w3.org/2003/05/soap-envelope";
const ADDRESSING: &str = "http://www.w3.org/2005/08/addressing";
const SECEXT: &str =
    "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd";
const SECUTIL: &str =
    "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd";
const WUWS: &str = "http://schemas.microsoft.com/msus/2014/10/WindowsUpdateAuthorization";
const WUCLIENT: &str = "http://www.microsoft.com/SoftwareDistribution/Server/ClientWebService";
const DEVICE_ATTRIBUTES: &str = "E:BranchReadinessLevel=CBB&DchuNvidiaGrfxExists=1&ProcessorIdentifier=Intel64%20Family%206%20Model%2063%20Stepping%202&CurrentBranch=rs4_release&DataVer_RS5=1942&FlightRing=Retail&AttrDataVer=57&InstallLanguage=en-US&DchuAmdGrfxExists=1&OSUILocale=en-US&InstallationType=Client&FlightingBranchName=&Version_RS5=10&UpgEx_RS5=Green&GStatus_RS5=2&OSSkuId=48&App=WU&InstallDate=1529700913&ProcessorManufacturer=GenuineIntel&AppVer=10.0.17134.471&OSArchitecture=AMD64&UpdateManagementGroup=2&IsDeviceRetailDemo=0&HidOverGattReg=C%3A%5CWINDOWS%5CSystem32%5CDriverStore%5CFileRepository%5Chidbthle.inf_amd64_467f181075371c89%5CMicrosoft.Bluetooth.Profiles.HidOverGatt.dll&IsFlightingEnabled=0&DchuIntelGrfxExists=1&TelemetryLevel=1&DefaultUserRegion=244&DeferFeatureUpdatePeriodInDays=365&Bios=Unknown&WuClientVer=10.0.17134.471&PausedFeatureStatus=1&Steam=URL%3Asteam%20protocol&Free=8to16&OSVersion=10.0.17134.472&DeviceFamily=Windows.Desktop";

fn element(name: &str, elements: Vec<XMLNode>) -> XMLNode {
    let mut el = Element::new(name);
    for c in elements {
        el.children.push(c);
    }
    XMLNode::Element(el)
}

fn element_attrs(name: &str, elements: Vec<XMLNode>, attrs: HashMap<&str, &str>) -> XMLNode {
    let mut el = Element::new(name);
    for c in elements {
        el.children.push(c);
    }

    for (k, v) in attrs {
        el.attributes.insert(k.to_owned(), v.to_owned());
    }
    XMLNode::Element(el)
}

fn download_tickets(msa_user_token: Option<String>) -> XMLNode {
    let mut tickets = Element::new(&("wuws:WindowsUpdateTicketsToken"));
    tickets
        .attributes
        .insert("u:id".to_string(), "ClientMSA".to_owned());
    tickets
        .attributes
        .insert("xmlns:wsu".to_string(), SECUTIL.to_string());
    tickets
        .attributes
        .insert("xmlns:wuws".to_string(), WUWS.to_owned());

    match msa_user_token {
        Some(token) => {
            tickets.children.push(element_attrs(
                "TicketType",
                Vec::from([element("User", Vec::from([XMLNode::Text(token)]))]),
                HashMap::from([("Name", "MSA"), ("Version", "1.0"), ("Policy", "MBI_SSL")]),
            ));
        }
        None => {}
    }

    tickets.children.push(element_attrs(
        "TicketType",
        Vec::new(),
        HashMap::from([("Name", "AAD"), ("Version", "1.0"), ("Policy", "MBI_SSL")]),
    ));

    XMLNode::Element(tickets)
}

fn download_header(url: String, method: &str, msa_user_token: Option<String>) -> XMLNode {
    let now = chrono::offset::Utc::now();

    let created_str = now.format("%+").to_string();
    let expire_str = (now + Duration::minutes(5)).format("%+").to_string();

    element(
        &("s:Header"),
        Vec::from([
            element_attrs(
                &("a:Action"),
                Vec::from([XMLNode::Text(
                    "http://www.microsoft.com/SoftwareDistribution/Server/ClientWebService/"
                        .to_owned()
                        + method,
                )]),
                HashMap::from([(("s:mustUnderstand"), "1")]),
            ),
            element(
                &("a:MessageID"),
                Vec::from([XMLNode::Text(
                    "urn:uuid:5754a03d-d8d5-489f-b24d-efc31b3fd32d".to_string(),
                )]),
            ),
            element_attrs(
                &("a:To"),
                Vec::from([XMLNode::Text(url)]),
                HashMap::from([(("s:mustUnderstand"), "1")]),
            ),
            element_attrs(
                &("a:Security"),
                Vec::from([element(
                    &("u:Timestamp"),
                    Vec::from([
                        element(&("u:Created"), Vec::from([XMLNode::Text(created_str)])),
                        element(&("u:Expires"), Vec::from([XMLNode::Text(expire_str)])),
                    ]),
                )]),
                HashMap::from([("s:mustUnderstand", "1"), ("xmlns:o", SECEXT)]),
            ),
            download_tickets(msa_user_token),
        ]),
    )
}

fn download_request(identity: String, revision: String, msa_user_token: Option<String>) -> Element {
    let mut envelope = Element::new("s:Envelope");

    envelope
        .attributes
        .insert("xmlns:a".to_string(), ADDRESSING.to_owned());
    envelope
        .attributes
        .insert("xmlns:s".to_string(), SOAP.to_owned());
    envelope
        .attributes
        .insert("xmlns:u".to_string(), SECUTIL.to_string());

    envelope.children.push(download_header(
        SECURED_URL.to_owned(),
        "GetExtendedUpdateInfo2",
        msa_user_token,
    ));

    envelope.children.push(element(
        &("s:Body"),
        Vec::from([element(
            &("GetExtendedUpdateInfo2"),
            Vec::from([
                element(
                    &("updateIDs"),
                    Vec::from([element(
                        &("UpdateIdentity"),
                        Vec::from([
                            element(
                                &("UpdateID"),
                                Vec::from([XMLNode::Text(identity.to_owned())]),
                            ),
                            element(
                                &("RevisionNumber"),
                                Vec::from([XMLNode::Text(revision.to_owned())]),
                            ),
                        ]),
                    )]),
                ),
                element(
                    &("infoTypes"),
                    Vec::from([element(
                        &("XmlUpdateFragmentType"),
                        Vec::from([XMLNode::Text(String::from("FileUrl"))]),
                    )]),
                ),
                element(
                    &("deviceAttributes"),
                    Vec::from([XMLNode::Text(DEVICE_ATTRIBUTES.to_owned())]),
                ),
            ]),
        )]),
    ));
    // soap_body.children.push(value)
    // envelope.children.push(xmltree::XMLNode::Element(soap_body));
    envelope
}

async fn post_xml(url: String, data: Element) -> Result<()> {
    let file = File::create("request.xml")?;
    data.write(file)?;
    let data_str = fs::read_to_string("request.xml")?;
    println!("{}", data_str);

    let client = reqwest::Client::new();
    let req = client
        .post(url)
        .header("Content-Type", "application/soap+xml")
        .body(data_str)
        .send()
        .await?;

    let status = req.status();
    let txt = req.text().await?;
    println!("{} {}", txt, status.as_str());
    Ok(())
}

pub async fn download_version(identity: String, revision: String, destination: String) {
    println!("Downloading version");
    let down_req = download_request(identity, revision, None);
    let res = post_xml(SECURED_URL.to_owned(), down_req).await;
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    }
    println!("Downloaded version");
}
