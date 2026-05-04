#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

pub use ferros_core::{foundation_ready, FOUNDATION_MARKER, FOUNDATION_VERSION};

pub const TARGET_ARCH: &str = "x86_64";
pub const ROOT_POSTURE: &str = "FERROS-root";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamily {
    UefiApplication,
    KernelImage,
    RootfsImage,
    SerialLog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactContract {
    family: ArtifactFamily,
    file_name: &'static str,
    purpose: &'static str,
}

impl ArtifactContract {
    pub const fn new(
        family: ArtifactFamily,
        file_name: &'static str,
        purpose: &'static str,
    ) -> Self {
        Self {
            family,
            file_name,
            purpose,
        }
    }

    #[must_use]
    pub const fn family(&self) -> ArtifactFamily {
        self.family
    }

    #[must_use]
    pub const fn file_name(&self) -> &'static str {
        self.file_name
    }

    #[must_use]
    pub const fn purpose(&self) -> &'static str {
        self.purpose
    }
}

pub const ARTIFACT_CONTRACTS: [ArtifactContract; 4] = [
    ArtifactContract::new(
        ArtifactFamily::UefiApplication,
        "ferros-bootx64.efi",
        "future UEFI application entry artifact",
    ),
    ArtifactContract::new(
        ArtifactFamily::KernelImage,
        "ferros-kernel-x86_64.elf",
        "future kernel handoff artifact",
    ),
    ArtifactContract::new(
        ArtifactFamily::RootfsImage,
        "ferros-rootfs.img",
        "future read-only Phase 0 disk image",
    ),
    ArtifactContract::new(
        ArtifactFamily::SerialLog,
        "qemu-serial.log",
        "future human-readable boot checkpoint trail",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootCheckpoint {
    FirmwareEntered,
    BootArtifactLocated,
    KernelHandoffAttempted,
    FirstKernelCheckpointEmitted,
}

#[must_use]
pub const fn checkpoint_label(checkpoint: BootCheckpoint) -> &'static str {
    match checkpoint {
        BootCheckpoint::FirmwareEntered => "firmware entered",
        BootCheckpoint::BootArtifactLocated => "boot artifact located",
        BootCheckpoint::KernelHandoffAttempted => "kernel handoff attempted",
        BootCheckpoint::FirstKernelCheckpointEmitted => "first kernel checkpoint emitted",
    }
}

pub const BOOT_CHECKPOINTS: [BootCheckpoint; 4] = [
    BootCheckpoint::FirmwareEntered,
    BootCheckpoint::BootArtifactLocated,
    BootCheckpoint::KernelHandoffAttempted,
    BootCheckpoint::FirstKernelCheckpointEmitted,
];

#[cfg(test)]
mod tests {
    use super::{
        checkpoint_label, foundation_ready, ArtifactFamily, ARTIFACT_CONTRACTS,
        BOOT_CHECKPOINTS, FOUNDATION_MARKER, ROOT_POSTURE, TARGET_ARCH,
    };

    #[test]
    fn scaffold_artifact_names_match_the_boot_path_research_contract() {
        assert_eq!(TARGET_ARCH, "x86_64");
        assert_eq!(ROOT_POSTURE, "FERROS-root");

        assert_eq!(ARTIFACT_CONTRACTS[0].family(), ArtifactFamily::UefiApplication);
        assert_eq!(ARTIFACT_CONTRACTS[0].file_name(), "ferros-bootx64.efi");
        assert_eq!(ARTIFACT_CONTRACTS[1].file_name(), "ferros-kernel-x86_64.elf");
        assert_eq!(ARTIFACT_CONTRACTS[2].file_name(), "ferros-rootfs.img");
        assert_eq!(ARTIFACT_CONTRACTS[3].file_name(), "qemu-serial.log");
    }

    #[test]
    fn scaffold_checkpoint_labels_match_the_boot_observation_order() {
        let labels = BOOT_CHECKPOINTS.map(checkpoint_label);

        assert_eq!(
            labels,
            [
                "firmware entered",
                "boot artifact located",
                "kernel handoff attempted",
                "first kernel checkpoint emitted",
            ]
        );
    }

    #[test]
    fn scaffold_preserves_the_portable_foundation_seam() {
        assert_eq!(FOUNDATION_MARKER, "foundation-ready");
        assert!(foundation_ready());
    }
}