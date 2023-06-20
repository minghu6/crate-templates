let crate_type = variable::get("crate_type");
let crate_name = variable::get("crate_name");

switch crate_type {
    "bin" => {
        print("Its bin");
        file::rename("bin/main.rs", `bin/${crate_name}.rs`)
    },
    "lib" => print("Its lib"),
    _ => ()
}
