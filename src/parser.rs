use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::{fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Name {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CurrentMemory {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Memory {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Vcpu {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Os {
    #[serde(rename = "type")]
    os_type: OSType,
    boot: Boot,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Boot {
    dev: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct OSType {
    arch: String,
    machine: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct VmPort {
    state: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Empty {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Features {
    acpi: Option<Empty>,
    apic: Option<Empty>,
    vmport: VmPort,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CPU {
    mode: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Clock {
    offset: String,
    timer: Vec<Timer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Timer {
    name: String,
    tickpolicy: Option<String>,
    present: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PM {
    #[serde(rename = "suspend-to-mem")]
    suspend_to_mem: Suspend,
    #[serde(rename = "suspend-to-disk")]
    suspend_to_disk: Suspend,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Suspend {
    enabled: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Devices {
    emulator: String,
    disk: Disk,
    controller: Vec<Controller>,
    interface: Interface,
    console: Console,
    channel: Vec<Channel>,
    input: Input,
    graphics: Graphics,
    sound: Sound,
    video: Video,
    redirdev: Vec<RedirDev>,
    memballoon: MemBalloon,
    rng: RNG,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Disk {
    r#type: String,
    device: String,
    driver: Driver,
    source: Source,
    target: Target,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Driver {
    name: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Source {
    file: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Target {
    dev: String,
    bus: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Controller {
    r#type: String,
    model: String,
    ports: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Interface {
    r#type: String,
    source: NetworkSource,
    model: Model,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NetworkSource {
    network: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Model {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Console {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Channel {
    r#type: String,
    source: Option<ChannelSource>,
    target: ChannelTarget,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ChannelSource {
    mode: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ChannelTarget {
    r#type: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Input {
    r#type: String,
    bus: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Graphics {
    r#type: String,
    port: String,
    #[serde(rename = "tlsPort")]
    tls_port: String,
    autoport: String,
    image: Image,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Image {
    compression: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Sound {
    model: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Video {
    model: VideoModel,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct VideoModel {
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RedirDev {
    bus: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MemBalloon {
    model: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RNG {
    model: String,
    backend: Backend,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Backend {
    #[serde(rename = "model")]
    backend_model: String,
    #[serde(rename = "$value")]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Domain {
    r#type: String,
    name: Name,
    memory: Memory,
    #[serde(rename = "currentMemory")]
    current_memory: CurrentMemory,
    vcpu: Vcpu,
    os: Os,
    features: Features,
    cpu: CPU,
    clock: Clock,
    pm: PM,
    devices: Devices,
}

pub fn test() -> Result<()> {
    let mut xml_file = File::open("test.xml")?;
    let mut xml_contents = String::new();
    xml_file.read_to_string(&mut xml_contents)?;

    let domain: Domain = from_str(&xml_contents)?;
    println!("{:?}", domain);

    Ok(())
}
