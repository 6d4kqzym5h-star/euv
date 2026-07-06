use crate::*;

/// A uniform-grid spatial hash for broad-phase collision culling in 2D.
///
/// Bodies are inserted by their world-space axis-aligned bounding box.
/// A query returns all candidate indices whose AABBs overlap the query region,
/// dramatically reducing narrow-phase collision checks from O(n²) to near O(n).
#[derive(Clone, Data, New, PartialEq)]
pub struct SpatialHashGrid2D {
    /// The world-space size of each grid cell.
    #[get(type(copy))]
    pub(crate) cell_size: f64,
    /// The inverse of `cell_size`, precomputed for fast coordinate-to-cell hashing.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) inverse_cell_size: f64,
    /// The hash map from cell key to the list of body indices occupying that cell.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) cells: SpatialCellMap2D,
}

/// A uniform-grid spatial hash for broad-phase collision culling in 3D.
///
/// Bodies are inserted by their world-space axis-aligned bounding box.
/// A query returns all candidate indices whose AABBs overlap the query region,
/// dramatically reducing narrow-phase collision checks from O(n²) to near O(n).
#[derive(Clone, Data, New, PartialEq)]
pub struct SpatialHashGrid3D {
    /// The world-space size of each grid cell.
    #[get(type(copy))]
    pub(crate) cell_size: f64,
    /// The inverse of `cell_size`, precomputed for fast coordinate-to-cell hashing.
    #[get(type(copy))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) inverse_cell_size: f64,
    /// The hash map from cell key to the list of body indices occupying that cell.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) cells: SpatialCellMap3D,
}
