use crate::{println, CPUID};

pub fn run_tests() {
    check_for_features();
}


fn check_for_features() {
    let features = CPUID.get_feature_info().unwrap();
    assert!(features.has_apic());
    assert!(features.has_acpi());
    assert!(features.has_sse3());
    println!("CPU features test: SUCCESS");
}
