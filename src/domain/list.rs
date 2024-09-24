use anyhow::Result;
use virt::connect::Connect;

pub fn list_domain(conn: &Connect) -> Result<()> {
    let doms = conn.list_all_domains(0)?;

    println!("{:<20} {}", "Instance Name", "State");
    println!("{:<20} {}", "-".repeat(20), "-".repeat(10));
    for dom in doms {
        let name = dom.get_name()?;
        let state = crate::domain::get_state_str(&dom)?;
        println!("{:<20} {}", name, state);
    }
    Ok(())
}
