use anyhow::Result;
use virt::{
    connect::Connect, domain::Domain, storage_pool::StoragePool, storage_vol::StorageVol,
    sys::VIR_DOMAIN_NONE,
};

pub fn spawn_domain(
    conn: &Connect,
    pool: &StoragePool,
    name: &str,
    template: &str,
    tmp_xml: &mut str,
    tmp_vol: &StorageVol,
) -> Result<()> {
    crate::pool::refresh_pool(&pool)?;

    let vol_name = name.to_string().clone() + ".qcow2";
    StorageVol::create_xml_from(
        &pool,
        &tmp_vol
            .get_xml_desc(0)?
            .replace(&tmp_vol.get_name()?, &vol_name),
        &tmp_vol,
        0,
    )?;

    let xml = set_name_xml(
        &name,
        &set_fname_xml(
            &StorageVol::lookup_by_name(&pool, &vol_name)?.get_path()?,
            &tmp_xml,
        ),
    );

    let dom = Domain::create_xml(conn, &xml, VIR_DOMAIN_NONE)?;

    println!(
        "Domain {} is created from template {}",
        dom.get_name()?,
        template
    );
    Ok(())
}

fn set_name_xml(name: &str, xml: &str) -> String {
    let xml = set_xml(name, xml, "NAME");
    xml.to_string()
}

fn set_fname_xml(name: &str, xml: &str) -> String {
    let xml = set_xml(name, xml, "FILENAME");
    xml.to_string()
}

fn set_xml(name: &str, xml: &str, dst: &str) -> String {
    let xml = xml.replace(dst, name);
    xml.to_string()
}
