use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};
use tokio::{stream::StreamExt, time};

#[derive(serde::Deserialize, Debug)]
struct MakoValue<T> {
    #[serde(rename = "type")]
    ty: String,
    data: T,
}

type List = MakoValue<Vec<Vec<Notification>>>;

#[derive(serde::Deserialize, Debug)]
struct Notification {
    id: MakoValue<u64>,
}

const NOTIFICATION_DURATION: Duration = Duration::from_secs(6);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut map = HashMap::new();

    let mut interval = time::interval(Duration::from_secs(1));
    while let Some(_) = interval.next().await {
        let mako_output = tokio::process::Command::new("makoctl")
            .arg("list")
            .output()
            .await?;
        let mako_output: List = serde_json::from_slice(&mako_output.stdout)?;

        let mut current = HashSet::with_capacity(5);
        for app in mako_output
            .data
            .into_iter()
            .map(|array| array.into_iter())
            .flatten()
        {
            current.insert(app.id.data);

            if !map.contains_key(&app.id.data) {
                map.insert(app.id.data, Instant::now());
            }
        }

        map.retain(|key, start| {
            if !current.contains(key) {
                return false;
            }

            if start.elapsed() > NOTIFICATION_DURATION {
                std::process::Command::new("makoctl")
                    .arg("dismiss")
                    .arg("-n")
                    .arg(key.to_string())
                    .spawn()
                    .unwrap();
                false
            } else {
                true
            }
        });
    }

    Ok(())
}
