use rumqttd::{Broker, Config};

fn main() {
    let config = Config::default();
    let mut broker = Broker::new(config);
    
    config.max_connections = 1000; // 设置最大连接数
    config.max_inflight_messages = 100; // 设置最大飞行中消息数
    println!("MQTT Broker is Start!");
    println!("Server Port= 1833/tcp");
    broker.start().unwrap();
}
