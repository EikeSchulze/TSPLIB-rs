type NodeId = u32;
type Tour = Vec<NodeId>;

enum DataSections {
    NODE_COORD_SECTION(Vec<Node>),
    DEPOT_SECTION(Vec<NodeId>),
    DEMAND_SECTION(Vec<Demand>),
    EDGE_DATA_SECTION(Vec<Vec<NodeId>>),
    FIXED_EDGES_SECTION(Vec<Edge>),
    DISPLAY_DATA_SECTION(Vec<DisplayNode>),
    TOUR_SECTION(Vec<Tour>),
    EDGE_WEIGHT_SECTION // Mystery
}

pub struct Node {
    pub id: NodeId,
    pub x: f64,
    pub y: f64,
    pub z: f64
}

struct Demand {
    id: NodeId,
    demand: u32
}

struct Edge {
    from: NodeId,
    to: NodeId
}

struct DisplayNode {
    id: NodeId,
    x: f64,
    y: f64
}
