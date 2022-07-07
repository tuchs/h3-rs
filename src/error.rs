use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError, PartialEq)]
pub enum Error {
    #[error("The operation failed but a more specific error is not available")]
    Failed, // 1

    #[error("Argument was outside of acceptable range (when a more specific error code is not available)")]
    Domain, // 2

    #[error("Latitude or longitude arguments were outside of acceptable range")]
    LatLngDomain, // 3

    #[error("Resolution argument was outside of acceptable range")]
    ResDomain, // 4

    #[error("H3Index cell argument was not valid")]
    CellInvalid, // 5

    #[error("H3Index directed edge argument was not valid")]
    DirectedEdgeInvalid, // 6

    #[error("H3Index undirected edge argument was not valid")]
    UndirectedEdgeInvalid, // 7

    #[error("H3Index vertex argument was not valid")]
    VertexInvalid, // 8

    #[error("Pentagon distortion was encountered")]
    Pentagon, // 9

    #[error("Duplicate input was encountered in the arguments")]
    DuplicateInput, // 10

    #[error("H3Index cell arguments were not neighbors")]
    NotNeighbors, // 11

    #[error("H3Index cell arguments had incompatible resolutions")]
    ResMismatch, // 12

    #[error("Necessary memory allocation failed")]
    Memory, // 13

    #[error("Bounds of provided memory were not large enough")]
    MemoryBounds, // 14

    #[error("Mode or flags argument was not valid")]
    OptionInvalid, // 15
}
