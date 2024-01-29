use color_print::cprintln;

use crate::types::{Config, UpdateDatapackSchema};

/// Performs the datapack update operation
pub async fn update(schema: UpdateDatapackSchema) {
    let mut config = Config::load();

    if !config.api_url.ends_with("/") {
        config.api_url.push('/');
    }

    let url = format!("{}datapacks/update", config.api_url.clone());

    let json_body = serde_json::to_string(&schema).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .body(json_body)
        .header("Authorization", config.api_key)
        .send()
        .await
        .unwrap();

    let code = res.status();

    if !code.is_success() {
        let text = res.text().await.unwrap();

        cprintln!(
            "<red>Encountered an error: {:?}</red> <yellow>(code {})</yellow>",
            text,
            code
        );
    } else {
        cprintln!("<green>Successfully updated datapack!</green>");
    }
}
