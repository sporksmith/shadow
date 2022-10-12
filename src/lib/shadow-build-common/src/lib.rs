use std::path::Path;

pub struct ShadowBuildCommon {
    deps: Option<system_deps::Dependencies>,
    build_src_root: Box<Path>,
    src_root: Box<Path>,
}

impl ShadowBuildCommon {
    pub fn new(repo_root: &Path, system_deps: Option<system_deps::Dependencies>) -> Self {
        let src_root = {
            let mut p = repo_root.to_path_buf();
            p.push("src");
            p.into_boxed_path()
        };

        let build_src_root = {
            let mut p = repo_root.to_path_buf();
            p.push("build");
            p.push("src");
            p.into_boxed_path()
        };

        // Conservatively re-run build scripts if anything in their package directory
        // changes.
        println!("cargo:rerun-if-changed=.");

        // *Very* conservatively re-run build script if shd-config.h changes.
        // It currently ~always changes, particularly since it includes a build
        // timestamp.
        println!(
            "cargo:rerun-if-changed={}/shd-config.h",
            build_src_root.to_str().unwrap()
        );

        Self {
            deps: system_deps,
            build_src_root,
            src_root,
        }
    }

    pub fn cc_build(&self) -> cc::Build {
        let mut b = cc::Build::new();
        println!("cargo:rerun-if-env-changed=CC");
        println!("cargo:rerun-if-env-changed=CXX");
        println!("cargo:rerun-if-env-changed=CFLAGS");
        println!("cargo:rerun-if-env-changed=CXXFLAGS");

        b.define("_GNU_SOURCE", None)
            .include(&*self.build_src_root)
            .include(&*self.src_root)
            // Always include debug symbols and frame pointers for
            // ease of debugging
            .flag("-ggdb")
            .flag("-fno-omit-frame-pointer")
            // Disable extra warnings (-Wall, -Wextra) until if and when they're
            // fixed in our C code.
            .warnings(false)
            // Enable some select extra warnings
            .flag("-Wreturn-type")
            .flag("-Wswitch")
            // By default, *don't* convert any remaining warnings into errors (-Werror).
            // -Werror is currently enabled here via CFLAGS, which
            // cmake sets depending on the option SHADOW_WERROR.
            .warnings_into_errors(false);

        if let Some(deps) = &self.deps {
            b.includes(deps.all_include_paths());
        }

        if let Some("true") = std::env::var("DEBUG").ok().as_ref().map(|s| s.as_str()) {
            b.flag("-DDEBUG")
                // we only check for unused functions when builing in debug mode since some
                // functions are only called when logging, which can be #ifdef'd out in
                // release mode
                .flag("-Wunused-function");
        } else {
            b.flag("-DNDEBUG");
        }

        b
    }

    pub fn bindgen_builder(&self) -> bindgen::Builder {
        let mut builder = bindgen::Builder::default()
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .clang_args(&[
                &format!("-I{}", self.build_src_root.to_str().unwrap()),
                &format!("-I{}", self.src_root.to_str().unwrap()),
                "-D_GNU_SOURCE",
            ])
            //# used to generate #[must_use] annotations)
            .enable_function_attribute_detection();

        if let Some(deps) = &self.deps {
            for path in deps.all_include_paths() {
                builder = builder.clang_args(&[format!("-I{}", path.to_str().unwrap())]);
            }
        }
        builder
    }

    pub fn cbindgen_base_config(&self) -> cbindgen::Config {
        let header = "
/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */
// clang-format off";
        cbindgen::Config {
            language: cbindgen::Language::C,
            line_length: 100,
            documentation_style: cbindgen::DocumentationStyle::C99,
            header: Some(header.into()),
            autogen_warning: Some(
                "/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */"
                    .into(),
            ),
            enumeration: cbindgen::EnumConfig {
                prefix_with_name: true,
                rename_variants: cbindgen::RenameRule::ScreamingSnakeCase,
                ..cbindgen::EnumConfig::default()
            },
            function: cbindgen::FunctionConfig {
                must_use: Some("__attribute__((warn_unused_result))".into()),
                ..cbindgen::FunctionConfig::default()
            },
            export: cbindgen::ExportConfig {
                rename: std::collections::HashMap::from([
                    ("timeval".into(), "struct timeval".into()),
                    ("timespec".into(), "struct timespec".into()),
                ]),
                // All types.
                item_types: vec![
                    cbindgen::ItemType::Enums,
                    cbindgen::ItemType::Constants,
                    cbindgen::ItemType::Globals,
                    cbindgen::ItemType::Structs,
                    cbindgen::ItemType::Unions,
                    cbindgen::ItemType::Typedefs,
                    cbindgen::ItemType::OpaqueItems,
                    cbindgen::ItemType::Functions,
                ],
                ..cbindgen::ExportConfig::default()
            },
            ..cbindgen::Config::default()
        }
    }
}