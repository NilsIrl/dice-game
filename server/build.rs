fn main() {
    println!("cargo:rerun-if-env-changed=DATABASE_URL");
    println!("cargo:rerun-if-changed=src/schema.rs");
    for entry in walkdir::WalkDir::new("migrations") {
        println!("cargo:rerun-if-changed={}", entry.unwrap().path().display());
    }
    std::process::Command::new("diesel")
        .arg("migration")
        .arg("run")
        .status()
        .unwrap();
}
