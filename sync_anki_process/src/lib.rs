use anki_integration::{invoke, parse_card};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::cell::Cell;
use tokio::runtime::Runtime;
use serde::Serialize;
use serde_json;

#[derive(Clone, Serialize)]
struct Payload {
    question: String, 
    answer: String
}

pub struct SyncAnkiTask {
    // Add the fields later as needed
    pub cached_card: Cell<u64>,
    pub hits: Cell<u8>, // can represent 255
}

impl SyncAnkiTask {
    pub fn new() -> Self {
        SyncAnkiTask {
            cached_card: Cell::new(0),
            hits: Cell::new(0),
        }
    }

    pub fn spawn_background_thread(&self, running: Arc<AtomicBool>, tx: mpsc::Sender<String>) -> thread::JoinHandle<()> {
        let cached_card = self.cached_card.clone();
        let hits = self.hits.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();

            // Thread logic here
            loop {
                if !running.load(Ordering::SeqCst) {
                    // Do nothing (we are paused)
                    continue;
                }
                
                rt.block_on(async {
                    // Do something in the background thread
                    let response = invoke("guiCurrentCard", None).await;
                    match response {
                        Ok(r) => {
                            match parse_card(r) {
                                Ok(card) => {
                                    if card.result.cardId != cached_card.get() {
                                        // we have a new card
                                        tx.send(serde_json::to_string(&Payload { 
                                            question: card.result.fields.Front.value,
                                            answer: card.result.fields.Back.value
                                        }).unwrap()).unwrap();

                                        cached_card.set(card.result.cardId);
                                        hits.set(0);
                                    } else {
                                        // Update the number of hits this card has received
                                        // 255 is set as the memory size of a u8
                                        if hits.get() < 255 {
                                            hits.set(hits.get() + 1);
                                        }
                                    }
                                }
                                Err(err) => {
                                    if cached_card.get() != 0 {
                                        // I am using a cardId of 0 to represent no card being selected (either due to a missing connection to Anki or otherwise)
                                        cached_card.set(0);
                                        hits.set(0); 
                                    } else {
                                        // we already don't have a card selected. Increase the hit rate of no card selected
                                        if hits.get() < 255 {
                                            hits.set(hits.get() + 1);
                                        }
                                    }
                                    eprintln!("Card not open or an error is occurring: {:?}", err)
                                }
                            };
                        },
                        Err(_err) => {
                            eprintln!("Could not find Anki connection")
                        }
                    }
                });

                // based on the number of hits observed, set the wait time
                // can modify this to make the latency less or higher depending on restrictions and target speed
                let wait_time = match hits.get() {
                    0..=20 => 1,
                    21..=60 => 2,
                    61..=120 => 3,
                    121..=200 => 4,
                    201..=255 => 5,
                };

                thread::sleep(Duration::from_secs(wait_time));
            }
        })
    }
}
