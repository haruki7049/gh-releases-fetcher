use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: usize,
    author: User,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<Asset>,
    tarball_url: String,
    zipball_url: String,
    body: String,

    reactions: Reactions,
}

#[derive(Debug, Deserialize)]
struct Asset {
    url: String,
    id: usize,
    node_id: String,
    name: String,
    label: String,
    uploader: User,
    content_type: String,
    state: String,
    size: usize,
    download_count: usize,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id: usize,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,

    #[serde(rename = "type")]
    user_type: String,

    user_view_type: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize)]
struct Reactions {
    url: String,
    total_count: usize,
    laugh: usize,
    hooray: usize,
    confused: usize,
    heart: usize,
    rocket: usize,
    eyes: usize,

    #[serde(rename = "+1")]
    plus_one: usize,

    #[serde(rename = "-1")]
    minus_one: usize,
}

fn main() -> Result<(), reqwest::Error> {
    let owner = "rust-lang";
    let repo = "rust";
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);

    let client = reqwest::blocking::Client::new();
    let response: Vec<Release> = client.get(url)
        .header(reqwest::header::CONTENT_TYPE, "application_json")
        .header(reqwest::header::USER_AGENT, "gh-relfetch")
        .send()?
        .json()?;

    println!("{:?}", response);

    Ok(())
}
