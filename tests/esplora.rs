//! Run some integration tests against Blockstream's public testnet Esplora instance.
#[cfg(feature = "esplora")]
mod tests {
    use std::net::SocketAddr;
    use std::str::FromStr;

    use anyhow::Result;

    use bdk::blockchain::{Blockchain, EsploraBlockchain, NoopProgress};
    use bdk::database::MemoryDatabase;
    use bdk::wallet::Wallet;

    use bitcoin::{Network, Txid};

    const ESPLORA_URL: &str = "https://blockstream.info/testnet/api";
    const DESCRIPTOR: &str = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    const TXID: &str = "e0b603b64aae91c8fb49d452103ebd99620b38a5fb36d69c9cc801e20d1f95a6";

    #[test]
    fn get_tx_without_dns_resolver() -> Result<()> {
        let txid = Txid::from_str(TXID)?;

        let blockchain = EsploraBlockchain::new(ESPLORA_URL, None);
        let tx = blockchain.get_tx(&txid)?;

        assert!(tx.is_some());

        Ok(())
    }

    #[test]
    fn get_tx_with_dns_resolver() -> Result<()> {
        let txid = Txid::from_str(TXID)?;

        let sock = SocketAddr::from_str("1.1.1.1:53")?;
        let blockchain = EsploraBlockchain::new(ESPLORA_URL, Some(sock));
        let tx = blockchain.get_tx(&txid)?;

        assert!(tx.is_some());

        Ok(())
    }

    #[test]
    fn sync_wallet() -> Result<()> {
        let blockchain = EsploraBlockchain::new(ESPLORA_URL, None);
        let wallet = Wallet::new(
            DESCRIPTOR,
            None,
            Network::Testnet,
            MemoryDatabase::default(),
            blockchain,
        )?;
        wallet.sync(NoopProgress, None)?;

        Ok(())
    }
}
