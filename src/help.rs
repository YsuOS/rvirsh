use anyhow::Result;

pub fn show_help() -> Result<()> {
    let width = 20;
    let main_helps: Vec<[&str; 2]> = vec![
        ["help", "Show this help"],
        ["list", "List all domains"],
        ["info", "Show domain various information"],
        ["define", "Define domain"],
        ["create", "Create and run domain"],
        ["start", "Start domain"],
        ["shutdown", "Shutdown domain"],
        ["reboot", "Reboot domain"],
        ["suspend", "Suspend domain"],
        ["resume", "Resume domain"],
        ["reset", "Reset domain"],
        ["poweroff", "Forcefully terminate domain"],
        [
            "delete",
            "Run 'undefine', 'vol-delete', and 'snapshot-delete'",
        ],
        ["net-list", "List all networks"],
        ["net-start", "Start network"],
        ["net-create", "Create and run network"],
        ["net-define", "Define network"],
        ["net-clean", "Run `net-stop` and `net-undefine`"],
        ["vol-list", "List all volumes"],
        ["vol-create", "Create volume"],
        ["vol-delete", "Delete volume"],
        ["snapshot-create", "Create and run domain snapshot"],
        ["snapshot-list", "List domain snapshots"],
        ["snapshot-current", "Get current snapshot on the domain"],
        ["snapshot-revert", "Set and revert snapshot on the domain"],
        ["snapshot-delete", "Delete a domain snapshots"],
        ["pool-define", "Define pool"],
        ["pool-create", "Create and run pool"],
        ["pool-start", "Start pool"],
        ["pool-list", "List all pools"],
        [
            "pool-clean",
            "Run 'pool-stop', 'pool-delete', and 'pool-undefine'",
        ],
        ["hostname", "Show the hypervisor hostname"],
        ["hostinfo", "Show the host information"],
        ["uri", "Show the hypervisor URI"],
        ["version", "Show version"],
    ];
    let sub_helps: Vec<[&str; 2]> = vec![
        ["autostart", "Enable autostart"],
        ["noautostart", "Disable autostart"],
        ["undefine", "Undefine domain"],
        ["dominfo", "Show domain information. Use `info`"],
        ["domstate", "Show domain state. Use `info`"],
        ["domid", "Show domain id. Use `info`"],
        ["domuuid", "Show domain uuid. Use `info`"],
        ["dumpxml", "Show domain information in XML"],
        ["net-autostart", "Enable net autostart"],
        ["net-noautostart", "Disable net autostart"],
        ["net-stop", "Stop net"],
        ["net-undefine", "Undefine net"],
        ["net-uuid", "Show net uuid"],
        ["net-info", "Show net information"],
        ["net-dumpxml", "Show net information in XML"],
        ["snapshot-info", "Show snapshot information"],
        ["snapshot-parent", "Get a parent of snapshot"],
        ["snapshot-dumpxml", "Show snapshot information in XML"],
        ["vol-info", "Show volume information"],
        ["vol-key", "Show volume key"],
        ["vol-path", "Show volume path"],
        ["vol-pool", "Show pool name the volume belongs to"],
        ["vol-dumpxml", "Show volume information in XML"],
        ["vol-wipe", "Wipe volume"],
        ["pool-info", "Show pool information"],
        ["pool-uuid", "Show pool uuid"],
        ["pool-refresh", "Refresh pool"],
        ["pool-stop", "Stop pool"],
        ["pool-delete", "Delete pool"],
        ["pool-undefine", "Undefine pool"],
        ["pool-autostart", "Enable pool autostart"],
        ["pool-noautostart", "Disable pool autostart"],
        ["pool-dumpxml", "Show pool information in XML"],
    ];

    println!("Usage: rv [COMMAND]");
    println!("\nMain Commands:");
    for h in main_helps {
        println!("{:<width$} {}", h[0], h[1]);
    }

    println!("\nThe following commands are supported, but it is not often used.");

    println!("\nSub Commands:");
    for h in sub_helps {
        println!("{:<width$} {}", h[0], h[1]);
    }

    println!("\nDeprecated Command:");
    println!("{:<width$} {}", "destroy", "Use 'poweroff'");
    println!("{:<width$} {}", "nodeinfo", "Use 'hostinfo'");
    println!("{:<width$} {}", "pool-destroy", "Use 'pool-stop'");
    println!("{:<width$} {}", "net-destroy", "Use 'net-stop'");
    println!();
    Ok(())
}
