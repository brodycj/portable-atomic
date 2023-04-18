// This file is @generated by target_spec.sh.
// It is not intended for manual editing.

#![allow(non_camel_case_types)]

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TargetArch {
    aarch64,
    amdgpu,
    arm,
    asmjs,
    avr,
    bpf,
    hexagon,
    loongarch64,
    m68k,
    mips,
    mips64,
    msp430,
    nvptx,
    nvptx64,
    powerpc,
    powerpc64,
    riscv32,
    riscv64,
    s390x,
    sparc,
    sparc64,
    spirv,
    wasm32,
    wasm64,
    x86,
    x86_64,
    xtensa,
}
pub use TargetArch::*;
impl TargetArch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::aarch64 => "aarch64",
            Self::amdgpu => "amdgpu",
            Self::arm => "arm",
            Self::asmjs => "asmjs",
            Self::avr => "avr",
            Self::bpf => "bpf",
            Self::hexagon => "hexagon",
            Self::loongarch64 => "loongarch64",
            Self::m68k => "m68k",
            Self::mips => "mips",
            Self::mips64 => "mips64",
            Self::msp430 => "msp430",
            Self::nvptx => "nvptx",
            Self::nvptx64 => "nvptx64",
            Self::powerpc => "powerpc",
            Self::powerpc64 => "powerpc64",
            Self::riscv32 => "riscv32",
            Self::riscv64 => "riscv64",
            Self::s390x => "s390x",
            Self::sparc => "sparc",
            Self::sparc64 => "sparc64",
            Self::spirv => "spirv",
            Self::wasm32 => "wasm32",
            Self::wasm64 => "wasm64",
            Self::x86 => "x86",
            Self::x86_64 => "x86_64",
            Self::xtensa => "xtensa",
        }
    }
}
impl core::fmt::Display for TargetArch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TargetOs {
    aix,
    android,
    cuda,
    dragonfly,
    emscripten,
    espidf,
    freebsd,
    fuchsia,
    haiku,
    hermit,
    horizon,
    illumos,
    ios,
    l4re,
    linux,
    macos,
    netbsd,
    none,
    nto,
    openbsd,
    psp,
    redox,
    solaris,
    solid_asp3,
    tvos,
    uefi,
    unknown,
    vita,
    vxworks,
    wasi,
    watchos,
    windows,
    xous,
}
pub use TargetOs::*;
impl TargetOs {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::aix => "aix",
            Self::android => "android",
            Self::cuda => "cuda",
            Self::dragonfly => "dragonfly",
            Self::emscripten => "emscripten",
            Self::espidf => "espidf",
            Self::freebsd => "freebsd",
            Self::fuchsia => "fuchsia",
            Self::haiku => "haiku",
            Self::hermit => "hermit",
            Self::horizon => "horizon",
            Self::illumos => "illumos",
            Self::ios => "ios",
            Self::l4re => "l4re",
            Self::linux => "linux",
            Self::macos => "macos",
            Self::netbsd => "netbsd",
            Self::none => "none",
            Self::nto => "nto",
            Self::openbsd => "openbsd",
            Self::psp => "psp",
            Self::redox => "redox",
            Self::solaris => "solaris",
            Self::solid_asp3 => "solid_asp3",
            Self::tvos => "tvos",
            Self::uefi => "uefi",
            Self::unknown => "unknown",
            Self::vita => "vita",
            Self::vxworks => "vxworks",
            Self::wasi => "wasi",
            Self::watchos => "watchos",
            Self::windows => "windows",
            Self::xous => "xous",
        }
    }
}
impl Default for TargetOs {
    fn default() -> Self {
        Self::none
    }
}
impl core::fmt::Display for TargetOs {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TargetEnv {
    eabihf,
    gnu,
    gnueabihf,
    libnx,
    msvc,
    musl,
    newlib,
    none,
    nto70,
    nto71,
    ohos,
    psx,
    relibc,
    sgx,
    uclibc,
}
pub use TargetEnv::*;
impl TargetEnv {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::eabihf => "eabihf",
            Self::gnu => "gnu",
            Self::gnueabihf => "gnueabihf",
            Self::libnx => "libnx",
            Self::msvc => "msvc",
            Self::musl => "musl",
            Self::newlib => "newlib",
            Self::none => "",
            Self::nto70 => "nto70",
            Self::nto71 => "nto71",
            Self::ohos => "ohos",
            Self::psx => "psx",
            Self::relibc => "relibc",
            Self::sgx => "sgx",
            Self::uclibc => "uclibc",
        }
    }
}
impl Default for TargetEnv {
    fn default() -> Self {
        Self::none
    }
}
impl core::fmt::Display for TargetEnv {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TargetEndian {
    big,
    little,
}
pub use TargetEndian::*;
impl TargetEndian {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::big => "big",
            Self::little => "little",
        }
    }
}
impl Default for TargetEndian {
    fn default() -> Self {
        Self::little
    }
}
impl core::fmt::Display for TargetEndian {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}