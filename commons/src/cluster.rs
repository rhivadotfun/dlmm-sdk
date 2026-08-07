use url::Url;

pub enum Cluster {
    Mainnet,
}

impl Cluster {
    pub fn url(self) -> Url {
        match self {
            Cluster::Mainnet => Url::parse("https://api.mainnet-beta.solana.com").unwrap(),
        }
    }
}
