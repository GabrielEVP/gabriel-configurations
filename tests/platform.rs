use gabriel::platform::{Architecture, OperatingSystem, Platform};

#[test]
fn detects_ubuntu_from_release_fields_not_display_name() {
    let platform = Platform::parse(
        "Linux",
        "x86_64",
        "NAME=\"Ubuntu Linux\"\nID=ubuntu\nVERSION_ID=\"26.04\"\n",
    )
    .unwrap();
    assert_eq!(platform.os, OperatingSystem::Ubuntu);
    assert_eq!(platform.architecture, Architecture::X86_64);
    assert_eq!(platform.version, [26, 4]);
}

#[test]
fn detects_apple_silicon_and_intel_macos() {
    for (name, architecture) in [
        ("arm64", Architecture::Aarch64),
        ("x86_64", Architecture::X86_64),
    ] {
        let platform = Platform::parse("Darwin", name, "26.0.1\n").unwrap();
        assert_eq!(platform.os, OperatingSystem::Macos);
        assert_eq!(platform.architecture, architecture);
        assert_eq!(platform.version, [26, 0, 1]);
    }
}

#[test]
fn rejects_other_distributions_even_with_ubuntu_in_the_name() {
    assert!(Platform::parse("Linux", "x86_64", "NAME=Ubuntu\nID=debian\nVERSION_ID=13").is_err());
}

#[test]
fn rejects_missing_invalid_or_duplicate_release_fields() {
    for release in [
        "ID=ubuntu",
        "ID=ubuntu\nVERSION_ID=nope",
        "ID=ubuntu\nVERSION_ID=26..04",
        "ID=ubuntu\nID=debian\nVERSION_ID=26.04",
        "ID=ubuntu\nVERSION_ID=26.04\nVERSION_ID=24.04",
    ] {
        assert!(Platform::parse("Linux", "x86_64", release).is_err());
    }
}

#[test]
fn rejects_unknown_architecture_and_kernel() {
    assert!(Platform::parse("Darwin", "riscv64", "26.0").is_err());
    assert!(Platform::parse("Windows", "x86_64", "11").is_err());
}
