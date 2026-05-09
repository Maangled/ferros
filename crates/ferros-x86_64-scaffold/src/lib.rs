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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelHandoffContract {
    arch: &'static str,
    kernel_artifact: ArtifactFamily,
    handoff_checkpoint: BootCheckpoint,
    first_kernel_checkpoint: BootCheckpoint,
    scope: &'static str,
}

impl KernelHandoffContract {
    pub const fn new(
        arch: &'static str,
        kernel_artifact: ArtifactFamily,
        handoff_checkpoint: BootCheckpoint,
        first_kernel_checkpoint: BootCheckpoint,
        scope: &'static str,
    ) -> Self {
        Self {
            arch,
            kernel_artifact,
            handoff_checkpoint,
            first_kernel_checkpoint,
            scope,
        }
    }

    #[must_use]
    pub const fn arch(&self) -> &'static str {
        self.arch
    }

    #[must_use]
    pub const fn kernel_artifact(&self) -> ArtifactFamily {
        self.kernel_artifact
    }

    #[must_use]
    pub const fn handoff_checkpoint(&self) -> BootCheckpoint {
        self.handoff_checkpoint
    }

    #[must_use]
    pub const fn first_kernel_checkpoint(&self) -> BootCheckpoint {
        self.first_kernel_checkpoint
    }

    #[must_use]
    pub const fn scope(&self) -> &'static str {
        self.scope
    }
}

pub const X86_64_KERNEL_HANDOFF_CONTRACT: KernelHandoffContract = KernelHandoffContract::new(
    TARGET_ARCH,
    ArtifactFamily::KernelImage,
    BootCheckpoint::KernelHandoffAttempted,
    BootCheckpoint::FirstKernelCheckpointEmitted,
    "architecture-contract only; no execution or bring-up claim",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootArtifactLineageContract {
    arch: &'static str,
    boot_artifact: ArtifactFamily,
    kernel_artifact: ArtifactFamily,
    observation_artifact: ArtifactFamily,
    checkpoint: BootCheckpoint,
    scope: &'static str,
}

impl BootArtifactLineageContract {
    pub const fn new(
        arch: &'static str,
        boot_artifact: ArtifactFamily,
        kernel_artifact: ArtifactFamily,
        observation_artifact: ArtifactFamily,
        checkpoint: BootCheckpoint,
        scope: &'static str,
    ) -> Self {
        Self {
            arch,
            boot_artifact,
            kernel_artifact,
            observation_artifact,
            checkpoint,
            scope,
        }
    }

    #[must_use]
    pub const fn arch(&self) -> &'static str {
        self.arch
    }

    #[must_use]
    pub const fn boot_artifact(&self) -> ArtifactFamily {
        self.boot_artifact
    }

    #[must_use]
    pub const fn kernel_artifact(&self) -> ArtifactFamily {
        self.kernel_artifact
    }

    #[must_use]
    pub const fn observation_artifact(&self) -> ArtifactFamily {
        self.observation_artifact
    }

    #[must_use]
    pub const fn checkpoint(&self) -> BootCheckpoint {
        self.checkpoint
    }

    #[must_use]
    pub const fn scope(&self) -> &'static str {
        self.scope
    }
}

pub const X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT: BootArtifactLineageContract =
    BootArtifactLineageContract::new(
        TARGET_ARCH,
        ArtifactFamily::UefiApplication,
        ArtifactFamily::KernelImage,
        ArtifactFamily::SerialLog,
        BootCheckpoint::KernelHandoffAttempted,
        "architecture-lineage only; artifact provenance without execution proof",
    );

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostedRehearsalArtifact {
    RuntimeBoundaries,
    X86SubcoreSmoke,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeObservationArtifact {
    KernelCheckpointLog,
    DeviceRunFinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceVocabularySeparationContract {
    arch: &'static str,
    hosted_rehearsal: HostedRehearsalArtifact,
    native_observation: NativeObservationArtifact,
    scope: &'static str,
}

impl EvidenceVocabularySeparationContract {
    pub const fn new(
        arch: &'static str,
        hosted_rehearsal: HostedRehearsalArtifact,
        native_observation: NativeObservationArtifact,
        scope: &'static str,
    ) -> Self {
        Self {
            arch,
            hosted_rehearsal,
            native_observation,
            scope,
        }
    }

    #[must_use]
    pub const fn arch(&self) -> &'static str {
        self.arch
    }

    #[must_use]
    pub const fn hosted_rehearsal(&self) -> HostedRehearsalArtifact {
        self.hosted_rehearsal
    }

    #[must_use]
    pub const fn native_observation(&self) -> NativeObservationArtifact {
        self.native_observation
    }

    #[must_use]
    pub const fn scope(&self) -> &'static str {
        self.scope
    }
}

pub const X86_64_EVIDENCE_VOCABULARY_SEPARATION: EvidenceVocabularySeparationContract =
    EvidenceVocabularySeparationContract::new(
        TARGET_ARCH,
        HostedRehearsalArtifact::RuntimeBoundaries,
        NativeObservationArtifact::KernelCheckpointLog,
        "vocabulary-separation only; hosted rehearsal is distinct from future native observation",
    );

#[cfg(test)]
mod tests {
    use super::{
        checkpoint_label, foundation_ready, ArtifactFamily, BootCheckpoint, ARTIFACT_CONTRACTS,
        BOOT_CHECKPOINTS, FOUNDATION_MARKER, HostedRehearsalArtifact,
        NativeObservationArtifact, ROOT_POSTURE, TARGET_ARCH,
        X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT, X86_64_EVIDENCE_VOCABULARY_SEPARATION,
        X86_64_KERNEL_HANDOFF_CONTRACT,
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

    #[test]
    fn scaffold_x86_64_kernel_handoff_contract_reuses_contract_vocab() {
        assert_eq!(X86_64_KERNEL_HANDOFF_CONTRACT.arch(), TARGET_ARCH);
        assert_eq!(
            X86_64_KERNEL_HANDOFF_CONTRACT.kernel_artifact(),
            ArtifactFamily::KernelImage
        );
        assert_eq!(
            X86_64_KERNEL_HANDOFF_CONTRACT.handoff_checkpoint(),
            BootCheckpoint::KernelHandoffAttempted
        );
        assert_eq!(
            X86_64_KERNEL_HANDOFF_CONTRACT.first_kernel_checkpoint(),
            BootCheckpoint::FirstKernelCheckpointEmitted
        );
    }

    #[test]
    fn scaffold_x86_64_kernel_handoff_contract_states_non_claim_scope() {
        assert_eq!(
            X86_64_KERNEL_HANDOFF_CONTRACT.scope(),
            "architecture-contract only; no execution or bring-up claim"
        );
    }

    #[test]
    fn scaffold_x86_64_boot_artifact_lineage_contract_reuses_artifact_vocab() {
        assert_eq!(X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT.arch(), TARGET_ARCH);
        assert_eq!(
            X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT.boot_artifact(),
            ArtifactFamily::UefiApplication
        );
        assert_eq!(
            X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT.kernel_artifact(),
            ArtifactFamily::KernelImage
        );
        assert_eq!(
            X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT.observation_artifact(),
            ArtifactFamily::SerialLog
        );
        assert_eq!(
            X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT.checkpoint(),
            BootCheckpoint::KernelHandoffAttempted
        );
    }

    #[test]
    fn scaffold_x86_64_boot_artifact_lineage_contract_states_non_claim_scope() {
        assert_eq!(
            X86_64_BOOT_ARTIFACT_LINEAGE_CONTRACT.scope(),
            "architecture-lineage only; artifact provenance without execution proof"
        );
    }

    #[test]
    fn scaffold_x86_64_evidence_vocabulary_keeps_hosted_and_native_terms_separate() {
        assert_eq!(X86_64_EVIDENCE_VOCABULARY_SEPARATION.arch(), TARGET_ARCH);
        assert_eq!(
            X86_64_EVIDENCE_VOCABULARY_SEPARATION.hosted_rehearsal(),
            HostedRehearsalArtifact::RuntimeBoundaries
        );
        assert_eq!(
            X86_64_EVIDENCE_VOCABULARY_SEPARATION.native_observation(),
            NativeObservationArtifact::KernelCheckpointLog
        );
    }

    #[test]
    fn scaffold_x86_64_evidence_vocabulary_separation_states_non_claim_scope() {
        assert_eq!(
            X86_64_EVIDENCE_VOCABULARY_SEPARATION.scope(),
            "vocabulary-separation only; hosted rehearsal is distinct from future native observation"
        );
    }
}