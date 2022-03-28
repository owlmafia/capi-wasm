#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::fs::ReadDir;
    use std::{
        env,
        fs::{self, OpenOptions},
        io::Write,
        path::Path,
        process::Command,
    };

    #[test]
    fn update_teal() -> Result<()> {
        let teal_path = Path::new("../../teal");

        // Update core's TEAL, to ensure that the copied TEAL is up to date (corresponds to PyTeal)
        update_teal_in_teal_dir(teal_path)?;

        // Copy the TEAL to this dao (as strings in Rust files)
        let core_teal_templates_dir = fs::read_dir(teal_path.join("teal_template"))?;
        import_teal_from(core_teal_templates_dir)?;

        println!("TEAL updated");

        Ok(())
    }

    /// Updates TEAL in the teal directory
    /// Mote specifically: Calls a script in the teal directory that compiles core's PyTeal and overwrites core's TEAL files.
    fn update_teal_in_teal_dir(teal_path: &Path) -> Result<()> {
        // the update teal script uses paths relative to core, so we switch to core's dir
        let initial_dir = env::current_dir()?;
        env::set_current_dir(&teal_path)?;

        let teal_update_script_path = "scripts/update_teal.sh";

        let script_res = Command::new("sh").arg(teal_update_script_path).status()?;
        println!("Update core TEAL script res: {:?}", script_res);

        env::set_current_dir(&initial_dir)?;

        Ok(())
    }

    fn import_teal_from(dir: ReadDir) -> Result<()> {
        let wasm_teal_path = Path::new("./src/teal");

        for entry in dir {
            let path = entry?.path();
            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            // Ignore files
            if [
                "always_succeeds",   // this is just for debugging / tests
                "capi_app_clear",    // related to capi asset
                "capi_app_approval", // related to capi asset
                "capi_escrow",       // related to capi asset
            ]
            .contains(&file_stem)
            {
                continue;
            }

            let path_to_write = wasm_teal_path.join(format!("{}.rs", file_stem));

            let teal_to_copy = fs::read(path)?;

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path_to_write)
                .unwrap();

            let mut contents: Vec<u8> = b"pub const SRC: &str = r#\"".to_vec();
            contents.extend(b"\n".to_vec());
            contents.extend(&teal_to_copy);
            contents.extend(b"\n".to_vec());
            contents.extend(b"\"#;".to_vec());
            file.write_all(&contents)?;
        }
        Ok(())
    }
}
