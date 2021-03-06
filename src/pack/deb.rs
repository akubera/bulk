use std::io::{self, Write};

use config::Metadata;
use deb_ext::WriteDebExt;


pub fn format_deb_control<W: Write>(out: &mut W, meta: &Metadata,
    version: &str, architecture: &str)
    -> io::Result<()>
{
    try!(out.write_kv("Package", &meta.name));
    try!(out.write_kv("Version", &version));
    try!(out.write_kv("Maintainer", "bulk"));
    try!(out.write_kv("Architecture", architecture));
    if let Some(ref deps) = meta.depends {
        try!(out.write_kv("Depends", deps));
    }
    try!(out.write_kv("Description",
        &format!("{}\n{}", meta.short_description, meta.long_description)));
    Ok(())
}
