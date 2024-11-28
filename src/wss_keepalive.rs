use std::{net::TcpStream, sync::{atomic::{AtomicUsize, Ordering}, Arc}, thread, time::Duration};
use serde_json::Value;
use tungstenite::{stream::MaybeTlsStream, Message};
use crate::tungstenite::{BinanceWebSocketClient, WebSocketState};
use std::error::Error;

type Conn = WebSocketState<MaybeTlsStream<TcpStream>>;
pub struct WssKeepalive {
    url: String,
    prepare_block: Option<Box<dyn Fn(&mut Conn)>>,
    conn: Option<Conn>,
    stream_ticket: Arc<AtomicUsize>,
}

impl WssKeepalive {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            prepare_block: None,
            conn: None,
            stream_ticket: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn connect(&mut self) -> &Self {
        let ret = BinanceWebSocketClient::connect_with_url(&self.url);
        match ret {
            Ok(conn) => {
                self.conn = Some(conn);
            },
            Err(e) => {
                println!("Connection failed. {:?}", e);
                self.conn = None;
            },
        }
        self
    }

    pub fn prepare<F: 'static>(mut self, block: F) -> Self 
        where F: Fn(&mut Conn) {
        self.prepare_block = Some(Box::new(block));
        self
    }

    pub fn close(&mut self) {
        self.stream_ticket.fetch_add(1, Ordering::SeqCst);
    }

    pub fn stream<F>(&mut self, block: &mut F, skip_error: bool) -> Result<(), Box<dyn Error>>
        where F: FnMut(Message) -> Result<bool, Box<dyn Error>> {
        let stream_ticket = self.stream_ticket.fetch_add(1, Ordering::SeqCst);
        loop {
            if stream_ticket != self.stream_ticket.load(Ordering::SeqCst) - 1 {
                println!("Ticket exit wss_keepalive");
                break;
            }
            if self.conn.is_none() {
                self.connect();
                if self.conn.is_some() {
                    if let Some(b) = self.prepare_block.as_ref() {
                        b(self.conn.as_mut().unwrap());
                    }
                } else {
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                let conn = self.conn.as_mut().unwrap();

                loop {
                    if conn.as_mut().can_read() {
                        let ret = conn.as_mut().read();
                        match ret {
                            Ok(message) => {
                                match &message {
                                    Message::Text(string_data) => {
                                        let json_value: Value = serde_json::from_str(string_data).unwrap();
                                        let e =  json_value.get("e");
                                        if let Some(v) = e {
                                            if v.as_str().unwrap() == "listenKeyExpired" {
                                                self.conn = None;
                                                break;
                                            }
                                        }
                                    },
                                    _ => {}
                                }

                                match message {
                                    _ => {
                                        let block_ret = block(message);
                                        match block_ret {
                                            Ok(continue_flag) => {
                                                if !continue_flag {
                                                    return Ok(());
                                                }
                                            },
                                            Err(e) => {
                                                println!("Error: {:?}", e);
                                                if !skip_error {
                                                    return Err(e);
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            Err(e) => {
                                println!("Error data {:?}", e);
                            }
                        }
                    } else {
                        println!("Connection disconnected.");
                        self.conn = None;
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}