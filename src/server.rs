use anyhow::Result;
use ktxstats::v3::KtxstatsV3;

use crate::ktxstats_v3;

pub fn server(data: &[u8]) -> Result<Server> {
    Ok(Server::from(&ktxstats_v3(data)?))
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Server {
    pub hostname: String,
    pub address: String,
    pub ip: String,
    pub port: u32,
}

impl From<&KtxstatsV3> for Server {
    fn from(stats: &KtxstatsV3) -> Self {
        Server {
            hostname: stats.hostname.clone(),
            address: format!("{}:{}", stats.ip, stats.port),
            ip: stats.ip.clone(),
            port: stats.port,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_server() -> Result<()> {
        {
            let demo_data = read("tests/files/duel_holy_vs_dago[bravado]20240426-1659.mvd")?;
            assert_eq!(
                server(&demo_data)?,
                Server {
                    hostname: "QUAKE.SE KTX:28501".to_string(),
                    address: "46.227.68.148:28501".to_string(),
                    ip: "46.227.68.148".to_string(),
                    port: 28501,
                }
            );
        }
        {
            let demo_data = read("tests/files/wipeout_red_vs_blue[q3dm6qw]20240406-2028.mvd")?;
            assert_eq!(
                server(&demo_data).unwrap_err().to_string(),
                "ktxstats not found"
            );
        }
        Ok(())
    }
}
