use reqwest::Client;
// The web address of texcreate to send requests to
const ADDRESS: &str = "https://texcreate.mkproj.com";
/// Returns the github link to download a template file given a version number and template name
pub fn gh_link(num: u64, name: &str) -> String {
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{num}/{name}.json")
}
/// Returns the repo github link to download `repo.toml` given a version number
pub fn repo_link(num: u64) -> String {
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{num}/repo.toml")
}
/// Sends a request to get the latest mkproj template repo version number
pub async fn get_latest_num() -> u64 {
    let client = Client::new();
    let link = format!("{ADDRESS}/repo/latest");
    let resp = client.get(&link).send().await.unwrap();
    let b = resp.bytes().await.unwrap();
    let s = String::from_utf8(b.to_vec()).unwrap();
    let num = s.trim().parse::<u64>().unwrap();
    num
}