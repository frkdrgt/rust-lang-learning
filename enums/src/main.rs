fn main() {
    let network = NetworkType::TCP;

    set_network_type(NetworkType::UDP);

    let server = Network {
        network_type: NetworkType::TCP,
        server_ip: String::from("127.0.0.1")
    };

    let ip_v4 = IpAddress::IpV4(String::from("127.0.0.1"));

    correct_network_type(NetworkType::UDP);
}

enum IpAddress {
    IpV4(String),
    IpV6(String)
}

enum NetworkType {
    TCP,
    UDP
}

fn set_network_type(network_type : NetworkType) {}

struct Network {
    network_type : NetworkType,
    server_ip : String
}


fn correct_network_type(network_type: NetworkType) -> bool {
    match network_type {
        NetworkType::TCP => {
            println!("TRUE VALUE");
            true
        },
        NetworkType::UDP => false
    }
}