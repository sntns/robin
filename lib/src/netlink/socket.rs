use crate::error::RobinError;

use neli::consts::nl::NlmF;
use neli::consts::socket::NlFamily;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::router::synchronous::{NlRouter, NlRouterReceiverHandle};
use neli::utils::Groups;

pub struct BatadvSocket {
    sock: NlRouter,
    family_id: u16,
}

impl BatadvSocket {
    pub fn connect() -> Result<Self, RobinError> {
        let (sock, _mcast) =
            NlRouter::connect(NlFamily::Generic, None, Groups::empty()).map_err(|e| {
                RobinError::Netlink(format!("Failed to connect with NlRouter: {:?}", e))
            })?;
        let family_id = sock
            .resolve_genl_family("batadv")
            .map_err(|e| RobinError::Netlink(format!("Failed to resolve family: {:?}", e)))?;

        Ok(Self { sock, family_id })
    }

    pub fn send(
        &mut self,
        flags: NlmF,
        msg: Genlmsghdr<u8, u16>,
    ) -> Result<NlRouterReceiverHandle<u16, Genlmsghdr<u8, u16>>, RobinError> {
        let recv = self
            .sock
            .send(self.family_id, flags, NlPayload::Payload(msg))
            .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

        Ok(recv)
    }
}
