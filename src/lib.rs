//! # nvme-nostd — Bare-metal NVMe driver for `#![no_std]` environments
//!
//! A pure-Rust, `#![no_std]` NVM Express driver that speaks directly to NVMe
//! controllers via PCI BAR0 memory-mapped registers. Implements admin commands
//! (Identify, Create I/O Queue) and I/O commands (Read, Write, Flush) per the
//! NVMe 1.4+ specification.
//!
//! ## Architecture
//!
//! ```text
//! NvmeController          — owns BAR0, admin queue, controller identity
//!   └─ NvmeDisk           — namespace handle with read/write/flush + BlockDevice
//!        └─ QueuePair     — submission + completion queue with doorbell access
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nvme_nostd::NvmeController;
//!
//! // BAR0 physical address from PCI enumeration (must be identity-mapped)
//! let mut ctrl = unsafe {
//!     NvmeController::init(bar0_addr).expect("nvme init failed")
//! };
//! let disk = ctrl.namespace(1).expect("namespace 1 not found");
//! let mut buf = [0u8; 512];
//! disk.read_sectors(0, 1, &mut buf, &mut ctrl).expect("read failed");
//! ```

#![no_std]

extern crate alloc;

pub mod registers;
pub mod queue;
pub mod admin;
pub mod io;
pub mod driver;

pub use driver::{NvmeController, NvmeDisk, NvmeBlockDevice, NvmeError};
