use chrono::Duration;
use regex::Regex;
use std::{
  fs::{self, File},
  path::PathBuf,
};
use std::io::Write;
use reqwest::Client;

use crate::{Error, Result};
use crate::model::get_versions_dir;

const SECURED_URL: &str =
  "https://fe3.delivery.mp.microsoft.com/ClientWebService/client.asmx/secured";

fn get_base_xml(path: PathBuf) -> Result<String> {
  Ok(fs::read_to_string(path)?)
}

fn download_request(path: PathBuf, identity: String, revision: String) -> Result<String> {
  let now = chrono::offset::Utc::now();

  let created_str = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
  let expire_str = (now + Duration::minutes(5)).format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

  Ok(get_base_xml(path)?
    .replace("{{CREATED}}", &created_str)
    .replace("{{EXPIRES}}", &expire_str)
    .replace("{{UPDATE_ID}}", &identity)
    .replace("{{REVISION_NUMBER}}", &revision))
}

async fn post_xml(client: Client, url: String, data: String) -> Result<String> {
  let res = client
    .post(url)
    .header("Content-Type", "application/soap+xml")
    .body(data)
    .send()
    .await?;

  let status = res.status();
  if !status.is_success() {
    Err(Error::VersionFailure(String::from(format!("Failed to get version url: {}", status.as_str()))))
  } else { Ok(res.text().await?) }
}

async fn download_url(client: Client, path: PathBuf, identity: String, revision: String) -> Result<String> {
  let req = download_request(path, identity, revision)?;
  let resp = post_xml(client, SECURED_URL.to_owned(), req).await?;

  let re = Regex::new(r"<Url>(http://tlu.dl.delivery.mp.microsoft.com/.+?)</Url>").unwrap();
  let captures = re.captures(&resp).unwrap();
  Ok(captures.get(1).unwrap().as_str().replace("&amp;", "&").to_owned())
}

async fn download_file(client: Client, url: String, destination: PathBuf) -> Result<()> {
  let res = client.get(&url).send().await?.bytes().await?;
  let mut file = File::create(destination)?;
  file.write(&res)?;
  Ok(())
}

pub async fn download_version(
  data_dir: PathBuf,
  req_path: PathBuf,
  identity: String,
  revision: String,
  version_name: String,
) -> Result<()> {
  println!("Downloading version");
  let file_name = format!("Minecraft-{}.appx", version_name);
  let destination = get_versions_dir(data_dir)?.join(PathBuf::from(file_name));
  let client = Client::new();
  let url = download_url(client.clone(), req_path, identity, revision).await?;
  download_file(client, url, destination).await?;
  println!("Downloaded version");
  Ok(())
}
