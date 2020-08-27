use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};
use tokio::{stream::StreamExt, time};
use structopt::StructOpt;

fn duration_from_secs(input: &str) -> Result<Duration, std::num::ParseIntError> {
    let secs: u64 = input.parse()?;
    Ok(Duration::from_secs(secs))
}

#[derive(StructOpt)]
struct Config {
    #[structopt(long, short, parse(try_from_str = duration_from_secs), default_value = "5", help = "How long should notifications show on screen")]
    timeout: Duration,
}

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_args();
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

            if start.elapsed() > config.timeout {
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
