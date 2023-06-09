extern crate mac_address;

use std::io::{Result, BufWriter, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::fs::{metadata, File, OpenOptions};
use mac_address::*;
use hex::encode;
use chrono::Local;
use pcap::{Device, ConnectionStatus};

#[derive(Debug)]
#[allow(dead_code)]
struct Detail {
    name: String,
    mac: String,
    addrs: String,
    status: String,
}

fn convert_mac(m: String, mut t: String) -> Result<String>{
    for (i, v) in m.char_indices() {
        if i + 1 == m.chars().count() {
            t += &v.to_string();
        } else {
            match i % 2 {
                1 => t += &format!("{}.", v),
                0 => t += &v.to_string(),
                _ => panic!("Invalid value."),
            }
        }
    }
    Ok(t)
}

fn mac_inter(interface: &Device) -> Result<Detail> {
    let mface = match mac_address_by_name(&interface.name) { 
        Ok(Some(m)) => convert_mac(encode(m.bytes()), "".to_string()).unwrap(),
        Ok(None) => "Can't found mac address from interface.".to_string(),
        Err(e) => panic!("{:?}", e),
    };
    let status = match interface.flags.connection_status{
        ConnectionStatus::Unknown => "Unknown",
        ConnectionStatus::Connected => "Connected",
        ConnectionStatus::Disconnected => "Disconnected",
        ConnectionStatus::NotApplicable => "NotApplicable",
    };
    Ok(Detail{
        name: interface.name.clone(),
        mac: mface,
        addrs: interface.addresses[interface.addresses.len() - 1].addr.to_string(),
        status: status.to_string()
    })
}

fn create_file(interface: &Device, dir: String, content: String) -> Result<()> {
    let device = mac_inter(interface).unwrap();
    let now = Local::now().format("%Y-%m-%dT%H.00.00").to_string();
    let filename = format!("{}{}_{}_{}.log", dir, device.mac, device.addrs, now);
    if metadata(filename.clone()).is_ok() {
        let file = OpenOptions::new().append(true).open(filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
    }else{
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
    }
    Ok(())
}

pub fn dump(dir: String) -> Result<()> {
    let interface = &Device::list().unwrap()[0];
    let mut cmd = Command::new("sudo tcpdump")
        .arg("-i")
        .arg(interface.name.as_str())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command.");
    let stdout = cmd.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines().flatten() {
        create_file(interface, dir.clone(), format!("{}\n", line))?;
    }
    let status = cmd.wait().unwrap();
    println!("tcpdump exited with status: {}", status);
    Ok(())
}
