// declared in Cargo.toml as "[build-dependencies]"
extern crate build_deps;

fn main() {
    // Enumerate IDL files in sub-folders "data/*/*.idl"
    build_deps::rerun_if_changed_paths( "data/*/*.idl" )
        .unwrap();

    // Capture added files in sub-folders "data/*"
    build_deps::rerun_if_changed_paths( "data/*" )
        .unwrap();

    // Capture added files/sub-folders in "data"
    build_deps::rerun_if_changed_paths( "data" )
        .unwrap();
}