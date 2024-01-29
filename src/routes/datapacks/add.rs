use color_print::cprintln;

use crate::types::{AddDatapackSchema, Config};

/// Performs the datapack add operation
pub async fn add(schema: AddDatapackSchema) {
    let mut config = Config::load();

    if !config.api_url.ends_with("/") {
        config.api_url.push('/');
    }

    let url = format!("{}datapacks/add", config.api_url.clone());

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
        cprintln!("<green>Successfully added datapack!</green>");
    }
}
