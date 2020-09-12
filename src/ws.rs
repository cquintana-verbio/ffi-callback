use accumulator::Accumulator;
use futures::stream::SplitStream;
use futures::StreamExt;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use std::os::raw::c_int;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use warp::ws::{Message, WebSocket};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Request {
    number: i32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Response {
    result: i32,
}

struct State {
    accumulator: Accumulator,
}

impl Default for State {
    fn default() -> Self {
        Self {
            accumulator: Accumulator::new(2),
        }
    }
}

impl State {
    pub fn register_callback<F>(&mut self, cb: &mut F)
    where
        F: FnMut(c_int),
    {
        self.accumulator.register_callback(cb)
    }
}

struct WsHandler {
    state: State,
    ws_rx: SplitStream<WebSocket>,
    ws_tx_chan: UnboundedSender<i32>,
}

impl WsHandler {
    pub fn new(ws: WebSocket, state: State) -> Self {
        let (ws_tx, ws_rx) = ws.split();
        let (send_message_tx, send_message_rx) = unbounded_channel::<i32>();
        tokio::spawn(async move {
            Self::send_message(ws_tx, send_message_rx).await;
        });
        Self {
            state,
            ws_rx,
            ws_tx_chan: send_message_tx,
        }
    }

    pub async fn run(&mut self) {
        let tx_clone = self.ws_tx_chan.clone();
        let mut cb = move |res: i32| {
            log::debug!("Res: {}", res);
            if let Err(e) = tx_clone.send(res) {
                log::error!("Could not send to WS Channel: {:?}", e);
            }
        };
        self.state.register_callback(&mut cb);

        while let Some(msg) = self.ws_rx.next().await {
            match msg {
                Ok(msg) => self.handle_message(msg).await,
                Err(e) => {
                    log::error!("Error handling message: {:?}", e);
                    return;
                }
            }
        }
    }

    async fn handle_message(&mut self, msg: Message) {
        if msg.is_text() {
            let body = msg.to_str().unwrap();
            let request: Request = serde_json::from_str(body).unwrap();
            log::info!("Received request: {:?}", request);
            self.state.accumulator.add(request.number);
        } else {
            log::error!("Received Message, but not Text: {:?}", msg);
        }
    }

    async fn send_message(
        mut ws_tx: SplitSink<WebSocket, Message>,
        mut send_message_rx: UnboundedReceiver<i32>,
    ) {
        while let Some(msg) = send_message_rx.next().await {
            let response = Response { result: msg };
            let as_string = serde_json::to_string(&response).unwrap();
            if let Err(e) = ws_tx.send(Message::text(as_string)).await {
                log::error!("Error sending message to WS: {:?}", e);
            }
        }
    }
}

pub async fn ws_handler(ws: WebSocket) {
    let state = State::default();
    let mut handler = WsHandler::new(ws, state);
    handler.run().await;
}
