// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
mod zircon_system_public_zircon_types;
pub use zircon_system_public_zircon_types::zx_status_t;
mod zircon_system_public_zircon_errors;
pub use zircon_system_public_zircon_errors::ZX_OK;
mod zircon_system_public_zircon_features;
pub use zircon_system_public_zircon_features::ZX_FEATURE_KIND_CPU;
pub use zircon_system_public_zircon_features::ZX_FEATURE_KIND_HW_BREAKPOINT_COUNT;
pub use zircon_system_public_zircon_features::ZX_FEATURE_KIND_HW_WATCHPOINT_COUNT;
pub use zircon_system_public_zircon_features::ZX_FEATURE_KIND_ADDRESS_TAGGING;
pub use zircon_system_public_zircon_features::ZX_FEATURE_KIND_VM;
pub use zircon_system_public_zircon_features::ZX_HAS_CPU_FEATURES;
pub use zircon_system_public_zircon_features::ZX_VM_FEATURE_CAN_MAP_XOM;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_FP;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_ASIMD;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_AES;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_PMULL;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SHA1;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SHA256;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_CRC32;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_ATOMICS;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_RDM;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SHA3;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SM3;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SM4;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_DP;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_DPB;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_FHM;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_TS;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_RNDR;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SHA512;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_I8MM;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SVE;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_ARM32;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ISA_SHA2;
pub use zircon_system_public_zircon_features::ZX_ARM64_FEATURE_ADDRESS_TAGGING_TBI;
pub type c_char = u8;
