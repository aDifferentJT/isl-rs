use std::{env, fs::File, io::Write, path::Path, process::Command};
use tar::Archive;
use xz::read::XzDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let num_jobs = env::var("NUM_JOBS")?;
    let isl_url = "https://libisl.sourceforge.io/isl-0.27.tar.xz";
    let isl_tar_path = Path::new(&out_dir).join("isl-0.27.tar.xz");
    let isl_dir_path = Path::new(&out_dir).join("isl-0.27/");
    let pwd_at_build_start_path = env::current_dir()?;

    // Extract only if not already extracted
    if !isl_dir_path.exists() {
        // Download only if file not already present
        if !isl_tar_path.exists() {
            println!("Downloading {}...", isl_url);
            let resp = reqwest::blocking::get(isl_url)?;
            if !resp.status().is_success() {
                return Err(format!("Failed to download {}: {}", isl_url, resp.status()).into());
            }
            let mut file = File::create(&isl_tar_path)?;
            let bytes = resp.bytes()?;
            file.write_all(&bytes)?;
        } else {
            println!("Using cached archive: {}", isl_tar_path.display());
        }

        println!("Extracting to {}...", isl_tar_path.display());
        let file = File::open(&isl_tar_path)?;
        let decompressor = XzDecoder::new(file);
        let mut archive = Archive::new(decompressor);
        archive.unpack(&out_dir)?;
    } else {
        println!("Already extracted: {}", isl_tar_path.display());
    }

    // Goto isl/ before building anything
    // (Not ideal but better than `make` emitting intermediary files into tree)
    env::set_current_dir(&isl_dir_path).expect("Could not cd into isl/");

    /* autogen.sh is only for git, not tar
    Command::new("./autogen.sh").status()
                                .expect("failed to autoreconf!");
    */
    Command::new("./configure").args(["--prefix",
                                      out_dir.as_str(),
                                      "--with-pic=isl",
                                      "--with-int=imath",
                                      "--enable-shared=no",
                                      "--enable-static=yes"])
                               .status()
                               .expect("failed to configure!");

    Command::new("make").args(&["-j", num_jobs.as_str()])
                        .status()
                        .expect("failed to make!");
    Command::new("make").args(&["install"])
                        .status()
                        .expect("failed to make install!");

    // Go back to old_pwd
    env::set_current_dir(pwd_at_build_start_path).expect("Could not cd back into OUT_DIR");

    println!("cargo:rustc-link-search=native={out_dir}/lib/");
    println!("cargo:rustc-link-lib=static=isl");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
