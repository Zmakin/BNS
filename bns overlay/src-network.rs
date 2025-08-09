use libp2p::{Swarm, gossipsub::{Gossipsub, GossipsubConfigBuilder}, kad::Kademlia, identity, PeerId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BitmapMessage {
    BitmapRegistration { entry: super::database::BitmapEntry },
    BitmapTransfer { blockheight: String, new_owner: String, transfer_block: u64 },
    BnsInscription { entry: super::database::BnsEntry },
    TimestampRequest { inscription_id: String },
    TimestampResponse { inscription_id: String, timestamp: u64 },
}

pub struct Network {
    swarm: Swarm<Gossipsub>,
}

impl Network {
    pub fn new(local_key: identity::Keypair, bootstrap_nodes: Vec<String>, network: &str) -> Self {
        let local_peer_id = PeerId::from(local_key.public());
        let gossipsub = Gossipsub::new(local_peer_id, GossipsubConfigBuilder::default().build().unwrap()).unwrap();
        let mut swarm = Swarm::new(
            libp2p::tcp::GenTcpConfig::new(),
            gossipsub,
            Kademlia::new(local_peer_id, libp2p::kad::store::MemoryStore::new(local_peer_id)),
        );
        swarm.behaviour_mut().gossipsub.subscribe(&format!("bitmap-inscriptions-{}", network).into_bytes()).unwrap();
        for node in bootstrap_nodes {
            swarm.dial(node.parse().unwrap()).unwrap();
        }
        Network { swarm }
    }

    pub fn broadcast_message(&mut self, message: &BitmapMessage) {
        let message_bytes = serde_cbor::to_vec(message).unwrap();
        self.swarm.behaviour_mut().gossipsub.publish(format!("bitmap-inscriptions").into_bytes(), message_bytes).unwrap();
    }

    pub fn poll(&mut self) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            loop {
                if let Some(event) = self.swarm.next().await {
                    // Handle incoming messages, sync registries
                }
            }
        });
    }
}
