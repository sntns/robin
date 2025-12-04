use neli::consts::genl::{CtrlAttr, CtrlCmd};
use neli::consts::nl::GenlId;
use neli::consts::socket::NlFamily;
use neli::err::RouterError;
use neli::genl::Genlmsghdr;
use neli::router::synchronous::NlRouter;
use neli::utils::Groups;

pub struct FamilyResolver;

impl FamilyResolver {
    /// Resolve generic family name "batadv" and return netlink family id.
    pub fn resolve() -> Result<u16, RouterError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>> {
        let family_name = "batadv";
        let (sock, _mcast_receiver) =
            NlRouter::connect(NlFamily::Generic, None, Groups::empty()).unwrap();
        sock.resolve_genl_family(family_name)
    }
}
