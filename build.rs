fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    #[cfg(windows)]
    {
        // Check if winpty is installed
        let lib = match vcpkg::Config::new()
            .emit_includes(true)
            .find_package("winpty")
        {
            Ok(lib) => lib,
            Err(err) => {
                panic!("note: vcpkg did not find winpty: {err:#}");
            }
        };
        if lib.include_paths.is_empty() {
            panic!("maybe need to reinstall winpty by vcpkg");
        }
    }
}
