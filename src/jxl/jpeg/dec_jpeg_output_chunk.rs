// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! OutputChunk from dec_jpeg_output_chunk.h. Borrowed byte storage and owned
//! buffers replace the source pointer/unique_ptr pair without a self-reference.
//! The active prefix length is independent of the backing storage length.

pub enum ChunkStorage<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}
pub struct OutputChunk<'a> {
    pub storage: ChunkStorage<'a>,
    pub len: usize,
    pub null: bool,
}
impl<'a> OutputChunk<'a> {
    // Empty source vectors can have either null or retained allocated storage.
    #[allow(clippy::ptr_arg)]
    pub fn borrowed(bytes: &'a Vec<u8>) -> Self {
        Self {
            storage: ChunkStorage::Borrowed(bytes),
            len: bytes.len(),
            null: bytes.capacity() == 0,
        }
    }
    pub fn owned(size: usize) -> Self {
        Self {
            storage: ChunkStorage::Owned(vec![0; size]),
            len: size,
            null: size == 0,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            storage: ChunkStorage::Owned(bytes.to_vec()),
            len: bytes.len(),
            null: bytes.is_empty(),
        }
    }
    pub fn bytes(&self) -> &[u8] {
        let data = match &self.storage {
            ChunkStorage::Borrowed(data) => *data,
            ChunkStorage::Owned(data) => data,
        };
        &data[..self.len]
    }
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        match &mut self.storage {
            ChunkStorage::Owned(data) => data,
            ChunkStorage::Borrowed(_) => panic!("source requires an owning chunk"),
        }
    }
}
