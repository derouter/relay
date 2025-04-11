use std::net::Ipv4Addr;

use clap::Parser;
use libp2p::{
    Multiaddr,
    futures::StreamExt as _,
    identify, identity,
    multiaddr::Protocol,
    noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};

#[derive(Debug, Parser)]
#[clap(name = "DeRouter P2P relay")]
struct Args {
    /// Hex-encoded 32-bytes-long ed25519 secret key.
    #[clap(long)]
    secret_key: String,

    /// Port to listen on.
    #[clap(long)]
    port: u16,
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    relay: relay::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

const IDENTIFY_PROTOCOL: &str = "/derouter/relay/0.1.0";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let secret_key = hex::decode(args.secret_key)?;
    let local_key = identity::Keypair::ed25519_from_bytes(secret_key)?;

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| Behaviour {
            relay: relay::Behaviour::new(key.public().to_peer_id(), Default::default()),
            ping: ping::Behaviour::new(ping::Config::new()),
            identify: identify::Behaviour::new(identify::Config::new(
                IDENTIFY_PROTOCOL.to_string(),
                key.public(),
            )),
        })?
        .build();

    let listen_addr_tcp = Multiaddr::empty()
        .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
        .with(Protocol::Tcp(args.port));

    let listen_addr_quic = Multiaddr::empty()
        .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
        .with(Protocol::Udp(args.port))
        .with(Protocol::QuicV1);

    println!("PeerId: /p2p/{}", swarm.local_peer_id());

    swarm.listen_on(listen_addr_tcp)?;
    swarm.listen_on(listen_addr_quic)?;

    loop {
        match swarm.next().await.expect("should be infinite") {
            SwarmEvent::Behaviour(event) => {
                if let BehaviourEvent::Identify(identify::Event::Received {
                    info: identify::Info { observed_addr, .. },
                    ..
                }) = &event
                {
                    swarm.add_external_address(observed_addr.clone());
                }

                println!("{event:?}")
            }

            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {address:?}");
            }

            _ => {}
        }
    }
}
