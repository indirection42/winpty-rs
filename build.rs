fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    #[cfg(windows)]
    {
        use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
        use windows::{h, s};

        let kernel32_res = unsafe { GetModuleHandleW(h!("kernel32.dll")) };
        let kernel32 = kernel32_res.unwrap();
        let conpty = unsafe { GetProcAddress(kernel32, s!("CreatePseudoConsole")) };
        if conpty.is_some() {
            println!("cargo:rustc-cfg=feature=\"conpty\"");
        }

        // Check if winpty is installed
        let lib = match vcpkg::Config::new()
            .emit_includes(true)
            .find_package("winpty")
        {
            Ok(lib) => lib,
            Err(err) => {
                println!("note: vcpkg did not find winpty: {err:#}");
                return;
            }
        };
        if lib.include_paths.is_empty() {
            panic!("maybe need to reinstall winpty by vcpkg");
        }

        println!("cargo:rustc-cfg=feature=\"winpty\"");
    }
}
