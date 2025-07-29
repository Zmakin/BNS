use crate::BitmapIndexer;

pub fn run(config: crate::config::Config) {
    let network_name = config.chain.clone();
    let mut indexer = BitmapIndexer::new(config, network_name);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(indexer.run());
}
