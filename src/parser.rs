use anyhow::Result;
use serde::{Deserialize, Serialize};
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
    #[serde(rename(serialize = "@dev"))]
    dev: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct OSType {
    #[serde(rename(serialize = "@arch"))]
    arch: String,
    #[serde(rename(serialize = "@machine"))]
    machine: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct VmPort {
    #[serde(rename(serialize = "@state"))]
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
    #[serde(rename(serialize = "@mode"))]
    mode: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Clock {
    #[serde(rename(serialize = "@offset"))]
    offset: String,
    timer: Vec<Timer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Timer {
    #[serde(rename(serialize = "@name"))]
    name: String,
    #[serde(rename(serialize = "@tickpolicy"))]
    tickpolicy: Option<String>,
    #[serde(rename(serialize = "@present"))]
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
    #[serde(rename(serialize = "@enabled"))]
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
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    #[serde(rename(serialize = "@device"))]
    device: String,
    driver: Driver,
    source: Source,
    target: Target,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Driver {
    #[serde(rename(serialize = "@name"))]
    name: String,
    #[serde(rename(serialize = "@type"))]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Source {
    #[serde(rename(serialize = "@file"))]
    file: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Target {
    #[serde(rename(serialize = "@dev"))]
    dev: String,
    #[serde(rename(serialize = "@bus"))]
    bus: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Controller {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    #[serde(rename(serialize = "@model"))]
    model: String,
    #[serde(rename(serialize = "@port"))]
    ports: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Interface {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    source: NetworkSource,
    model: Model,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NetworkSource {
    #[serde(rename(serialize = "@network"))]
    network: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Model {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Console {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Channel {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    source: Option<ChannelSource>,
    target: ChannelTarget,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ChannelSource {
    #[serde(rename(serialize = "@mode"))]
    mode: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ChannelTarget {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    #[serde(rename(serialize = "@name"))]
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Input {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    #[serde(rename(serialize = "@bus"))]
    bus: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Graphics {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
    #[serde(rename(serialize = "@port"))]
    port: String,
    #[serde(rename(deserialize = "tlsPort", serialize = "@tlsPort"))]
    tls_port: String,
    #[serde(rename(serialize = "@autoport"))]
    autoport: String,
    image: Image,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Image {
    #[serde(rename(serialize = "@compression"))]
    compression: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Sound {
    #[serde(rename(serialize = "@model"))]
    model: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Video {
    model: VideoModel,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct VideoModel {
    #[serde(rename(serialize = "@type"))]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RedirDev {
    #[serde(rename(serialize = "@bus"))]
    bus: String,
    #[serde(rename(serialize = "@type"))]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MemBalloon {
    #[serde(rename(serialize = "@model"))]
    model: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RNG {
    #[serde(rename(serialize = "@model"))]
    model: String,
    backend: Backend,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Backend {
    #[serde(rename(deserialize = "model", serialize = "@model"))]
    backend_model: String,
    #[serde(rename = "$value")]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "domain")]
struct Domain {
    #[serde(rename(serialize = "@type"))]
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

pub fn yaml_to_xml() -> Result<()> {
    let mut yaml_file = File::open("test.yaml")?;
    let mut yaml_contents = String::new();
    yaml_file.read_to_string(&mut yaml_contents)?;
    yaml_contents = yaml_contents.replace("value", "$value");

    let domain: Domain = serde_yml::from_str(&yaml_contents)?;
    println!("{:?}", domain);
    let xml = serde_xml_rs::to_string(&domain)?;
    println!("{}", xml);
    Ok(())
}

pub fn xml_to_yaml() -> Result<()> {
    let mut xml_file = File::open("test.xml")?;
    let mut xml_contents = String::new();
    xml_file.read_to_string(&mut xml_contents)?;

    let domain: Domain = serde_xml_rs::from_str(&xml_contents)?;
    println!("{:?}", domain);
    let yaml = serde_yml::to_string(&domain)?.replace("$value", "value");
    println!("{}", yaml);

    Ok(())
}
