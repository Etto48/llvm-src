use std::env;
use std::path::{Path, PathBuf};

/// A builder for configuring the build of LLVM.
/// You need to call [Build::build] to actually build the LLVM source code.
pub struct Build {
    host: Option<String>,
    target: Option<String>,
    out_dir: Option<PathBuf>,
    profile: Option<String>,
}

/// The artifacts produced by the build.
/// This is only created by calling [Build::build].
/// You need to call [Artifacts::print_cargo_metadata] to print the cargo metadata.
pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    libs: Vec<String>,
}

impl Build {
    /// Create a new `Build` instance.
    /// Parameters are fetched from the environment variables `HOST`, `TARGET`, `OUT_DIR`, and `PROFILE`.
    /// If one of these variables is not set, it must be manually set using the corresponding method:
    ///
    /// - [Build::host]
    /// - [Build::target]
    /// - [Build::out_dir]
    /// - [Build::profile]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the host triple.
    pub fn host(&mut self, host: &str) -> &mut Self {
        self.host = Some(host.to_string());
        self
    }

    /// Set the target triple.
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }

    /// Set the output directory, this should be rust's "target" directory.
    /// The actual build directory will be `$OUT_DIR/llvm-build`.
    pub fn out_dir(&mut self, out_dir: &PathBuf) -> &mut Self {
        self.out_dir = Some(out_dir.join("llvm-build"));
        self
    }

    /// Set the profile, e.g. "Release" or "Debug".
    /// These are the same profiles as used by cmake.
    pub fn profile(&mut self, profile: &str) -> &mut Self {
        self.profile = Some(profile.to_string());
        self
    }

    /// Build the LLVM source code.
    /// This will panic if any of the required environment variables are not set (see [Build::new]).
    /// Returns an [Artifacts] struct, you will need to call [Artifacts::print_cargo_metadata]
    /// to print the cargo metadata and configure the build in the build script.
    pub fn build(&self) -> Artifacts {
        let host = self.host.as_ref().expect("HOST not set").as_str();
        let target = self.target.as_ref().expect("TARGET not set").as_str();
        let profile = self.profile.as_ref().expect("PROFILE not set").as_str();

        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let lib_dir = out_dir.join("lib");
        let include_dir = out_dir.join("include");

        let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("llvm-15.0.7/llvm");

        let mut config = cmake::Config::new(source_dir);

        config
            .host(host)
            .target(target)
            .out_dir(out_dir)
            .profile(profile)
            .build();

        let libs = std::fs::read_dir(out_dir.join("build/lib"))
            .unwrap()
            .into_iter()
            .map(|f| f.unwrap())
            .filter(|f| f.file_type().unwrap().is_file())
            .map(|f| {
                let file_name = f.file_name().into_string().unwrap();
                let last_dot = file_name.rfind('.').unwrap();
                file_name[..last_dot].to_string()
            })
            .collect::<Vec<_>>();

        Artifacts {
            include_dir,
            lib_dir,
            libs,
        }
    }
}

impl Default for Build {
    fn default() -> Self {
        let host = env::var("HOST").ok();
        let target = env::var("TARGET").ok();
        let out_dir = env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("llvm-build"));
        let profile = env::var("PROFILE").ok();

        Self {
            host,
            target,
            out_dir,
            profile,
        }
    }
}

impl Artifacts {
    /// Get the include directory.
    pub fn include(&self) -> &Path {
        &self.include_dir
    }

    /// Get the lib directory.
    pub fn lib(&self) -> &Path {
        &self.lib_dir
    }

    /// Get the list of libraries.
    pub fn libs(&self) -> &[String] {
        &self.libs
    }

    /// Print the cargo metadata.
    pub fn print_cargo_metadata(&self) {
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
        for lib in &self.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}
