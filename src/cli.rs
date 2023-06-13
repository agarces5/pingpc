use std::{
    fmt::Display,
    ops::RangeInclusive,
    process::{Command, Output},
};

use clap::Parser;

#[derive(Debug)]
enum Address<'a> {
    IP(Vec<&'a str>),
    Red(Vec<&'a str>),
    Hotel(Vec<&'a str>),
}
impl<'a> Display for Address<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::IP(ip_vec) => {
                let joined = ip_vec.join(".");
                write!(f, "{}", joined)
            }
            Address::Red(red_vec) => {
                let joined = red_vec.join(".");
                write!(f, "{}", joined)
            }
            Address::Hotel(hotel_vec) => {
                let joined = hotel_vec.join(".");
                write!(f, "192.168.{}", joined)
            }
        }
    }
}

/// Utilidad para hacer ping a una IP, hotel o a una red, pudiendo seleccionar el rango de IP.
#[derive(Parser, Debug)]
#[command(name = "PingPC")]
#[command(author = "Antonio Garcés")]
#[command(
    help_template = "{about-section} \n {usage-heading} {usage} \n {all-args} {tab} \n\n Autor: {author-with-newline}"
)]
#[command(about, long_about)]
pub struct Args {
    /// Rango de IP (Ej. 1-255)
    #[arg(short, long)]
    range: Option<String>,

    /// IP, Centro o Red (Ej. 192.168.0.1, 101, 172.17.3)
    address: String,
}

impl Args {
    pub fn execute(&self) {
        let range = self.range();
        let address = self.address();
        match address {
            Address::IP(_) => {
                let output = Self::comando_ping(format!("{}", &address));
                if output.status.code().unwrap() == 0 {
                    println!("{}: Activo", &address);
                }
            }
            Address::Red(_) => {
                for i in range {
                    let output = Self::comando_ping(format!("{}.{}", &address, i));
                    if output.status.code().unwrap() == 0 {
                        println!("{}.{}: Activo", &address, i);
                    }
                }
            }
            Address::Hotel(_) => {
                for i in range {
                    let output = Self::comando_ping(format!("{}.{}", &address, i));
                    if output.status.code().unwrap() == 0 {
                        println!("{}.{}: Activo", &address, i);
                    }
                }
            }
        }
    }
    fn address(&self) -> Address {
        let address = self.address.trim().split('.').collect::<Vec<_>>();
        match address.len() {
            1 => Address::Hotel(address),
            3 => Address::Red(address),
            4 => Address::IP(address),
            _ => panic!("Demasiados numeros"),
        }
    }
    fn range(&self) -> RangeInclusive<u8> {
        match self.range.clone() {
            Some(range) => {
                let range = range
                    .trim()
                    .split('-')
                    .map(|n| {
                        n.parse::<u8>()
                            .expect("Debes introducir un rango entre 0-255")
                    })
                    .collect::<Vec<_>>();
                if range.len() != 2 {
                    panic!("Se deben introducir 2 valores");
                }

                range[0]..=range[1]
            }
            None => match self.address() {
                Address::IP(_) => 1..=1,
                Address::Red(_) => 1..=255,
                Address::Hotel(_) => 101..=255,
            },
        }
    }

    fn comando_ping(ip: String) -> Output {
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "ping -n 1 -w 10", ip.as_str()])
                .output()
                .expect("Failed to ping")
        }
        #[cfg(target_os = "linux")]
        {
            let args = format!("ping -c 1 {}", ip);
            Command::new("sh")
                .args(["-c", &args])
                .output()
                .expect("Failed to ping")
        }
    }
}
