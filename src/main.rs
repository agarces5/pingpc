use clap::Parser;
use cli::Args;

mod cli;

fn main() {
    let args = Args::parse();

    args.execute();

    // if let Some(hotel) = args.centro {
    //     for i in range.clone() {
    //         let output = comando_ping(format!("192.168.{}.{}", &hotel, i));
    //         if String::from_utf8_lossy(&output.stdout).contains("tiempo") {
    //             println!("192.168.{}.{}: Activo", &hotel, i);
    //         }
    //     }
    // }

    // if let Some(red) = args.total {
    //     for i in range {
    //         let output = comando_ping(format!("{}.{}", &red, i));
    //         if output.status.code().unwrap() == 0 {
    //             println!("{}.{}: Activo", &red, i);
    //         }
    //     }
    // }

    // if let Some(ip) = args.ip {
    //     let output = comando_ping(ip.clone());
    //     if output.status.code().unwrap() == 0 {
    //         println!("{}: Activo", &ip);
    //     }
    // }
}
