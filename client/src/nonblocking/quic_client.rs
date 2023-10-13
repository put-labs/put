#[deprecated(
    since = "1.15.0",
    note = "Please use `put_quic_client::nonblocking::quic_client::QuicClientConnection` instead."
)]
pub use put_quic_client::nonblocking::quic_client::QuicClientConnection as QuicTpuConnection;
pub use put_quic_client::nonblocking::quic_client::{
    QuicClient, QuicClientCertificate, QuicLazyInitializedEndpoint,
};
