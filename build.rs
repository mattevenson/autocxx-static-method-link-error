fn main() -> miette::Result<()> {
    let path: std::path::PathBuf = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path]).build()?;
    b.flag_if_supported("-std=c++14").compile("cereal"); 
    println!("cargo:rerun-if-changed=src/main.rs");
    Ok(())
}
