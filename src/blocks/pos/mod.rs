pub mod pos3d;
pub mod pos2d;
pub use pos3d::{BlockPos, ChunkPos, ChunkedPos};
pub use pos2d::{BlockPos2d, ColPos, ColedPos};

use super::CHUNK_S1I;

pub fn chunked(x: i32) -> (i32, usize) {
    let r = x.rem_euclid(CHUNK_S1I);
    ((x - r) / CHUNK_S1I, r as usize)
}

pub fn unchunked(cx: i32, dx: usize) -> i32 {
    cx * CHUNK_S1I + dx as i32
}
