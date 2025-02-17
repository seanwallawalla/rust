// Targets the Cortex-M0, Cortex-M0+ and Cortex-M1 processors (ARMv6-M architecture)

use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "thumbv6m-none-eabi".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            abi: "eabi".into(),
            // The ARMv6-M architecture doesn't support unaligned loads/stores so we disable them
            // with +strict-align.
            features: "+strict-align".into(),
            // There are no atomic instructions available in the instruction set of the ARMv6-M
            // architecture
            max_atomic_width: Some(0),
            atomic_cas: false,
            ..super::thumb_base::opts()
        },
    }
}
