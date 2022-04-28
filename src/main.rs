use clap::Parser;
use rand::Rng;
use std::net::UdpSocket;

// コマンドラインオプション
#[derive(Parser)]
#[clap(version = "1.0.0", author = "suzuki_ta")]
struct Opts {
    // #[clap(long, default_value = "127.0.0.1:4000")]
    /// 自分のIP
    #[clap(short, long, default_value = "localhost:4001")]
    from_address: String,

    /// 宛先IP
    #[clap(short, long, default_value = "localhost:4000")]
    to_address: String,

    /// packet size
    #[clap(short = 's', long, default_value = "8")]
    packet_size: i32,

    /// packet count (ex: image size / packet size: 768px / 8 = 96packet)
    #[clap(short = 'c', long, default_value = "96")]
    packet_count: i32,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let socket = UdpSocket::bind(opts.from_address)?; // localhost:4000をバインド
                                                      //let mut buf = [0; 2048 * 4]; // UDP受信バッファ 2048byte*4

    let mut rng = rand::thread_rng();

    loop {
        for i in 0..opts.packet_count {
            let first_px = i * opts.packet_size;
            let mut packet_data: String = first_px.to_string();
            for _j in 0..opts.packet_size {
                let r = &rng.gen_range(10.0..50.0).to_string();
                packet_data = packet_data + "," + r;
            }
            // Udp送信
            socket
                .send_to(format!("{}\n", packet_data).as_bytes(), &opts.to_address)
                .expect("failed to send response");

            println!("{}", packet_data);

            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}
