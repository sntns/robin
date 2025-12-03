use neli::consts::socket::NlFamily;
use neli::err::SocketError;
use neli::socket::synchronous::NlSocketHandle;
use neli::utils::Groups;

pub struct FamilyResolver;

impl FamilyResolver {
    /// Resolve generic family name "batadv" and return family id.
    pub fn resolve(sock: &mut NlSocketHandle) -> Result<u16, SocketError> {
        // neli provides convenience resolver as method on socket
        sock.resolve_genl_family("batadv")
    }

    pub fn connect_socket() -> Result<NlSocketHandle, SocketError> {
        NlSocketHandle::connect(NlFamily::Generic, None, Groups::empty())
    }
}
