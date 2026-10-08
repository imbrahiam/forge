use forge_packages::{Catalog, Repository};

#[test]
fn ripgrep_maps_to_a_native_ucrt64_package() {
    let plan = Catalog::curated().plan(["ripgrep"]).unwrap();
    assert_eq!(plan.packages.len(), 1);
    assert_eq!(plan.packages[0].name, "ripgrep");
    assert_eq!(plan.packages[0].target, "mingw-w64-ucrt-x86_64-ripgrep");
    assert_eq!(plan.packages[0].repository, Repository::Ucrt64);
}

#[test]
fn unknown_packages_fail_the_entire_plan_instead_of_silently_disappearing() {
    let error = Catalog::curated()
        .plan(["ripgrep", "not-a-package"])
        .unwrap_err();
    assert!(error.to_string().contains("not-a-package"), "{error}");
}

#[test]
fn developer_tools_keep_msys_and_native_package_provenance_distinct() {
    let plan = Catalog::curated()
        .plan(["git", "bash", "zsh", "python", "node"])
        .unwrap();
    let identities: Vec<_> = plan
        .packages
        .iter()
        .map(|p| (p.target, p.repository))
        .collect();
    assert_eq!(
        identities,
        [
            ("git", Repository::Msys),
            ("bash", Repository::Msys),
            ("zsh", Repository::Msys),
            ("mingw-w64-ucrt-x86_64-python", Repository::Ucrt64),
            ("mingw-w64-ucrt-x86_64-nodejs", Repository::Ucrt64),
        ]
    );
}

#[test]
fn repeated_packages_are_deduplicated_in_first_request_order() {
    let plan = Catalog::curated()
        .plan(["zsh", "ripgrep", "zsh", "git", "ripgrep"])
        .unwrap();
    let names: Vec<_> = plan.packages.iter().map(|p| p.name).collect();
    assert_eq!(names, ["zsh", "ripgrep", "git"]);
}

#[test]
fn empty_requests_are_rejected_instead_of_creating_a_meaningless_plan() {
    let error = Catalog::curated().plan([]).unwrap_err();
    assert!(
        error.to_string().contains("at least one package"),
        "{error}"
    );
}
