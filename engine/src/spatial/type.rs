use crate::*;

/// A grid cell key for 2D spatial partitioning, combining x and y indices.
pub type CellKey2D = (i32, i32);

/// A grid cell key for 3D spatial partitioning, combining x, y, and z indices.
pub type CellKey3D = (i32, i32, i32);

/// A list of body indices stored within a single grid cell.
pub type CellEntries = Vec<usize>;

/// A hash map from 2D cell keys to lists of body indices.
pub type SpatialCellMap2D = HashMap<CellKey2D, CellEntries>;

/// A hash map from 3D cell keys to lists of body indices.
pub type SpatialCellMap3D = HashMap<CellKey3D, CellEntries>;
