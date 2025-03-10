fn main() {
    // workaround to enable automatic rebuild of diesel migrations
    // https://docs.rs/diesel_migrations/2.2.0/diesel_migrations/macro.embed_migrations.html#automatic-rebuilds
    println!("cargo:rerun-if-changed=path/to/your/migration/dir/relative/to/your/Cargo.toml");
    
    // run build-time helpers for tauri app
    tauri_build::build();
}
