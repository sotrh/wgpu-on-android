use std::fs::File;
use std::io::{self, prelude::*};
use std::env;
use std::path::Path;

extern crate shaderc;

// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }

fn main() {
    let path = env::current_dir().unwrap();
    let shaders = ["src/shader.vert", "src/shader.frag"];
    let mut compiler = shaderc::Compiler::new().unwrap();
    for shader in &shaders {
        let path = path.join(shader);
        let kind = determine_shader_kind(path.clone()).unwrap();
        let new_path = path.with_extension(match kind {
            shaderc::ShaderKind::Vertex => "vert.spv",
            shaderc::ShaderKind::Fragment => "frag.spv",
            _ => "spv",
        });
        let code = file_to_string(path.clone()).unwrap();
        let spirv = compiler.compile_into_spirv(
            &code, 
            kind, 
            &path.to_str().unwrap(), 
            "main", 
            None
        ).unwrap();
        write_to_file(new_path, spirv.as_binary_u8()).unwrap();
    }
}

fn write_to_file<P: AsRef<Path>>(file: P, spirv: &[u8]) -> io::Result<()> {
    eprintln!("file: {:?}", file.as_ref());
    let mut out = File::create(file)?;
    out.write_all(spirv)
}

fn determine_shader_kind<P: AsRef<Path>>(path: P) -> Option<shaderc::ShaderKind> {
    let path = path.as_ref();
    path.extension().map(|e| {
        match e.to_str() {
            Some("vert") => shaderc::ShaderKind::Vertex,
            Some("frag") => shaderc::ShaderKind::Fragment,
            _ => panic!("Unsupported shader type extension: \"{:?}\"", e),
        }
    })
}

fn file_to_string<P: AsRef<Path>>(filename: P) -> io::Result<String> {
    eprintln!("filename: {:?}", filename.as_ref());
    let mut file = File::open(filename)?;
    let mut output = String::new();

    file.read_to_string(&mut output).unwrap();

    Ok(output)
}