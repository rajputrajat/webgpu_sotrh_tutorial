use anyhow::{bail, Context, Result};
use fs_extra::{self, file::read_to_string};
use shaderc::{self, ShaderKind};
use std::path::PathBuf;

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: ShaderKind,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let extension = src_path
            .extension()
            .context("file has no extension")?
            .to_str()
            .context("extension cannot be converted to &str")?;
        let kind = match extension {
            "vert" => ShaderKind::Vertex,
            "frag" => ShaderKind::Fragment,
            "comp" => ShaderKind::Compute,
            _ => bail!("unsupported shader: {:?}", src_path),
        };
        let src = read_to_string(src_path)?;
        let spv_path = src_path.with_extension(format!("{}.spv", extension));
        Ok(Self {
            src,
            src_path,
            spv_path,
            kind,
        })
    }
}

fn main() -> Result<()> {
    Ok(())
}
