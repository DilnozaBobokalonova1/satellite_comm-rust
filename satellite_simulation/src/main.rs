use std::{collections::HashMap, time::Duration};

use clap::{Arg, ArgMatches, Command};
use rand::Rng;
use simulation::{graph::SatelliteNetwork, satellite::Satellite};
use tokio_serial::{SerialPort, SerialPortBuilder, SerialPortBuilderExt};
mod common;
mod communication;
mod routing;
mod security;
mod simulation;
mod storage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let matches = Command::new("Satellite Simulator")
        .version("1.0")
        .about("Simulates a satellite communication network!")
        .arg(
            Arg::new("num-satellites")
                .short('n')
                .long("num-satellites")
                .help("Number of satellites to simulate")
                .default_value("10")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let num_satellites: usize = *matches.get_one::<usize>("num-satellites").unwrap_or(&5);

    // Create a network of satellites by first generating them then creating a graph and
    // updating their respective positions in the graph
    let mut network = SatelliteNetwork::new();
    network.generate_satellite_network(num_satellites);
    network.update_satellite_graph();

    // let connections: HashMap<u32, Vec<u32>> = find_nearby_satellites(&network.satellites);

    // for (sat_id, neighbors) in connections {
    //     println!("Satellite {} can communicate with {:?}", sat_id, neighbors);
    // }

    // UART LISTEN
    // let port_name = "/dev/tty.usbmodem2103";
    // let baud_rate = 115_200;
    // let serial_port_builder: SerialPortBuilder = tokio_serial::new(port_name, baud_rate);
    // let mut port = match serial_port_builder.timeout(Duration::from_secs(360)).open_native_async() {
    //     Ok(serial_stream) => serial_stream,
    //     Err(e) => {
    //         eprintln!("Failed to open serial port: {}", e);
    //         return;
    //     }
    // };

    // println!("Listening for UART commands from the board...");

    // // we are receiving commands/data by bytes
    // let (tx, mut rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel::<String>(BUFFER_SIZE);

    // tokio::spawn(async move {
    //     while let Some(message) = rx.recv().await {
    //         println!("Receiver got satellite data: {}", message);
    //     }
    // });

    // tokio::spawn(async move {
    //     // will store full message here if chunks are sent; considered complete once \n is encountered
    //     let mut message_buffer = String::new();
    //     loop {
    //         // port.readable().await.unwrap();
    //         let mut buf = vec![0u8; BUFFER_SIZE];
    //         match port.read(&mut buf).await {
    //             Ok(bytes_read) if bytes_read > 0 => {
    //                 let received_data = String::from_utf8_lossy(&buf[..bytes_read]);
    //                 println!("Received data: {}", received_data);

    //                 message_buffer.push_str(&received_data);
    //                 if received_data.contains("\n") {
    //                     let complete_message = message_buffer.clone();
    //                     message_buffer.clear();
    //                     // now lets transmit the complete data to receivers
    //                     let _ = tx.send(complete_message.to_string()).await;
    //                 }
    //             }

    //             Ok(_) => {}, // in case of 0 size data read
    //             // Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
    //             //     // no data available yet, so we continue
    //             //     continue;
    //             // }
    //             Err(e) => eprintln!("UART Read failed with error: {}", e),
    //         }
    //     }
    // });
}

const BUFFER_SIZE: usize = 1024;
