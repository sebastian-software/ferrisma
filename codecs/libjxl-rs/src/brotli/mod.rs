// Copyright 2009, 2010, 2013-2016 by the Brotli Authors.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Mechanical translation of Brotli at db24a98aed38a16de78cc05ca15cdd8dc2a0d92d.
//!
//! The retained streaming decoder uses the actual callback-owned allocations.
//! [`instance::DecoderInstance`] owns its state; raw streaming cursors and
//! borrowed dictionaries retain their documented source lifetime and aliasing
//! requirements. The source-distinct stack-owned one-shot entry is separate.
//! Native state and partial-bit representations are currently admitted only
//! on AArch64 and x86-64. Ordinary Rust builds do not compile or link C code.

pub mod bit_reader;
pub mod bitfield;
pub mod block_length;
pub mod command;
pub mod compound_dictionary;
pub mod constants;
pub mod context_map;
pub mod default_memory;
pub mod dictionary;
pub mod distance;
pub mod huffman;
pub mod huffman_reader;
pub mod instance;
pub mod layout;
pub mod memory;
pub mod metablock;
pub mod mtf;
pub mod raw_bytes;
pub mod raw_transform;
pub mod reader_copy;
pub mod ringbuffer;
pub mod running;
pub mod shared_dictionary;
pub mod state;
pub mod static_descriptors;
pub mod status;
pub mod stream;
pub mod stream_api;
pub mod stream_blocks;
pub mod stream_commands;
pub mod stream_header;
pub mod stream_input;
pub mod stream_output;
pub mod stream_switch;
pub mod stream_tables;
pub mod symbols;
pub mod tables;
pub mod transform;
pub mod tree_group;
