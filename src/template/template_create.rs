use anyhow::{anyhow, Context, Result};
use quick_xml::{
    events::{attributes::Attribute, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::{
    fs::{self, File},
    path::Path,
};
use virt::storage_pool::StoragePool;

pub fn create_template(
    pool: &StoragePool,
    template: &String,
    org_xml: &String,
    org_vol: &String,
) -> Result<()> {
    let pool_path = get_pool_path(&pool)?;

    let xml_path = pool_path.clone() + "/" + &template + ".xml";
    create_sealed_xml(Path::new(&org_xml), Path::new(&xml_path))?;

    let vol_path = pool_path.clone() + "/" + &template + ".qcow2";
    fs::copy(Path::new(&org_vol), Path::new(&vol_path))?;

    println!("Template {} is created", template);
    return Ok(());
}

// only support dir type
fn get_pool_path(pool: &StoragePool) -> Result<String> {
    let xml = pool.get_xml_desc(0)?;
    let mut reader = Reader::from_str(&xml);
    let mut pool_path: Option<String> = None;
    let mut in_path = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"path" => {
                in_path = true;
            }
            Ok(Event::Text(e)) if in_path => {
                pool_path = Some(e.unescape()?.to_string());
                break;
            }
            Err(e) => panic!("Error: {}", e),
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
    pool_path.with_context(|| anyhow!("Can not find pool path"))
}

fn create_sealed_xml(src: &Path, dst: &Path) -> Result<()> {
    let mut reader = Reader::from_file(src).unwrap();
    let mut writer = Writer::new(File::create(dst)?);
    let mut buf = Vec::new();
    let mut in_name = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"name" => {
                writer.write_event(Event::Start(e)).unwrap();
                in_name = true;
            }
            Ok(Event::Text(_)) if in_name => {
                writer
                    .write_event(Event::Text(BytesText::new("NAME")))
                    .unwrap();
                in_name = false;
            }
            Ok(Event::Empty(e)) if e.name().as_ref() == b"source" => {
                let mut elem = BytesStart::new("source");
                for attr in e.attributes() {
                    if let Ok(a) = attr {
                        if a.key.as_ref() == b"file" {
                            elem.push_attribute(Attribute::from(("file", "FILENAME")));
                        } else {
                            elem.push_attribute(a);
                        }
                    }
                }
                writer.write_event(Event::Empty(elem)).unwrap();
            }
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e).unwrap(),
            _ => (),
        }
    }
    Ok(())
}
