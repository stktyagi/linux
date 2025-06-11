// SPDX-License-Identifier: GPL-2.0

//! Rust sample custom module.

use kernel::prelude::*;

module! {
    type: RustSampleCustom,
    name: "rust_sample_custom",
    author: "Your Name",
    description: "A custom Rust kernel module sample",
    license: "GPL",
}

struct RustSampleCustom;

impl kernel::Module for RustSampleCustom {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Custom Rust sample module loaded\n");
        pr_info!("Hello from Rust kernel module!\n");
        Ok(RustSampleCustom)
    }
}

impl Drop for RustSampleCustom {
    fn drop(&mut self) {
        pr_info!("Custom Rust sample module unloaded\n");
    }
}
