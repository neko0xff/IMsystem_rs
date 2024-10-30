use crate::module::format_str::console_time;
use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::WebSocket;

/* 設計用於低延遲，全雙工&長期運行的連接*/

#[get("/channel1")]
// 1: 使用非同步迴圈和手動管理的串流和接收來處理訊息
pub fn echo_channel(ws: WebSocket) -> rocket_ws::Channel<'static> {
    println!("[{}] WebSocket:  /echo_ws/channel1", console_time());
    // 為 WebSocket 創建一個新頻道，並在非同步迴圈中處理接收的訊息
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => {
                    info!("/echo_ws/channel: Received message= {:?}", msg);
                    if let Err(e) = stream.send(msg).await {
                        error!("/echo_ws/channel1: Error sending message= {:?}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!(" /echo_ws/channel1: Error receiving message= {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }))
}

#[get("/stream1")]
// 2: 使用 `rocket_ws::Stream!` WebSocket 資料串流的巨集來更簡潔地處理訊息
pub fn echo_stream(ws: WebSocket) -> rocket_ws::Stream!['static] {
    println!("[{}] WebSocket:  /echo_ws/stream1", console_time());
    rocket_ws::Stream! { ws =>
        for await message in ws {
            match message {
                Ok(msg) => {
                    info!("/echo_ws/stream1: Received message= {:?}", msg);
                    yield msg;
                }
                Err(e) => {
                    error!("/echo_ws/stream1: Error receiving message= {:?}", e);
                    break;
                }
            }
        }
    }
    
}

#[get("/compose1")]
// 3: 使用簡單的串流組合直接傳遞訊息
pub fn echo_compose(ws: WebSocket) -> rocket_ws::Stream!['static] {
    println!("[{}] WebSocket: /echo_ws/compose1", console_time());
    // 創建一個簡單的傳遞串流，其中把接收到的訊息按原樣發送回客戶端
    ws.stream(|io| io)
}
