/// Selected BATADV attributes (from linux/uapi/batman_adv.h)
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum Attribute {
    BatadvAttrUnspec = 0,
    BatadvAttrVersion = 1,
    BatadvAttrAlgoName = 2,
    BatadvAttrMeshIfindex = 3,
    BatadvAttrMeshIfname = 4,
    BatadvAttrMeshAddress = 5,
    BatadvAttrHardIfindex = 6,
    BatadvAttrHardIfname = 7,
    BatadvAttrHardAddress = 8,
    BatadvAttrOrigAddress = 9,
    // ...
    BatadvAttrLastSeenMsecs = 24,
    BatadvAttrNeighAddress = 25,
    BatadvAttrTq = 26,
    BatadvAttrThroughput = 27,
    BatadvAttrRouter = 30,
    // ... add needed attributes
}

impl From<Attribute> for u16 {
    fn from(a: Attribute) -> Self {
        a as u16
    }
}

// +------------------------+-----------------------------------------------+---------------------------------------------+
// | Attribute              | Description                                   | When to use                                 |
// +------------------------+-----------------------------------------------+---------------------------------------------+
// | BatadvAttrUnspec        | Unspecified / unused                         | Never used                                  |
// | BatadvAttrVersion       | Batman-adv protocol version                  | When querying mesh information              |
// | BatadvAttrAlgoName      | Routing algorithm name                        | When querying routing info                  |
// | BatadvAttrMeshIfindex   | Index of the mesh interface                   | Specify mesh interface in requests          |
// | BatadvAttrMeshIfname    | Name of the mesh interface                    | Specify mesh interface in requests          |
// | BatadvAttrMeshAddress   | MAC address of the mesh interface             | Identify mesh interface                     |
// | BatadvAttrHardIfindex   | Index of the physical interface               | When querying underlying hardware           |
// | BatadvAttrHardIfname    | Name of the physical interface                | When querying underlying hardware           |
// | BatadvAttrHardAddress   | MAC of the physical interface                 | Identify hardware interface                 |
// | BatadvAttrOrigAddress   | MAC address of an originator node             | When listing originators                     |
// | BatadvAttrLastSeenMsecs | Last seen timestamp of a node (ms)            | Know freshness of originator info           |
// | BatadvAttrNeighAddress  | MAC address of a neighbor                     | For neighbor information                     |
// | BatadvAttrTq            | Link quality metric (TQ)                      | For routing or debugging                     |
// | BatadvAttrThroughput    | Throughput of a link/node                     | For performance stats                        |
// | BatadvAttrRouter        | Indicates if node is a router                 | For routing decisions                        |
// +------------------------+-----------------------------------------------+---------------------------------------------+
