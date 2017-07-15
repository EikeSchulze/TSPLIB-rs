trait Specification {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &Type;
    fn get_comment(&self) -> &str;
    fn get_dimension(&self) -> usize;
    fn get_capacity(&self) -> Option<u32>;
    fn get_edge_weight_type(&self) -> &EdgeWeightType;
    fn get_edge_weight_format(&self) -> &EdgeWeightFormat;
    fn get_edge_data_format(&self) -> &EdgeDataFormat;
    fn get_node_coord_type(&self) -> &NodeCoordType;
    fn get_display_data_type(&self) -> &DisplayDataType;
}

#[derive(Debug)]
pub enum Type {
    TSP,
    ATSP,
    SOP,
    HCP,
    CVRP,
    TOUR
}

#[derive(Debug)]
pub enum EdgeWeightType {
    EXPLICIT,
    EUC_2D,
    EUC_3D,
    MAX_2D,
    MAX_3D,
    MAN_2D,
    MAN_3D,
    CEIL_2D,
    GEO,
    ATT,
    XRAY1,
    XRAY2,
    SPECIAL
}

#[derive(Debug)]
pub enum EdgeWeightFormat {
    FUNCTION,
    FULL_MATRIX,
    UPPER_ROW,
    LOWER_ROW,
    UPPER_DIAG_ROW,
    LOWER_DIAG_ROW,
    UPPER_COL,
    LOWER_COL,
    UPPER_DIAG_COL,
    LOWER_DIAG_COL
}

#[derive(Debug)]
pub enum EdgeDataFormat {
    EDGE_LIST,
    ADJ_LIST
}

#[derive(Debug)]
pub enum NodeCoordType {
    TWOD_COORDS,
    THREED_COORDS,
    NO_COORDS
}

#[derive(Debug)]
pub enum DisplayDataType {
    COORD_DISPLAY,
    TWOD_DISPLAY,
    NO_DISPLAY
}
