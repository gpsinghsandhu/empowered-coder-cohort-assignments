use std::thread::spawn;
use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender, SyncSender, channel, TryRecvError, RecvTimeoutError, sync_channel};
use std::collections::{HashMap, HashSet};

/// A service to create a broadcast channel where we could have multiple consumers. This can also be used as multiple
/// producer multiple consumer channel by cloning the returned transmitter
///
/// Usage:
/// ```
///      let (tx, service) = create_channel::<String>()
///      let rx1 = service.create_receiver();
///      let rx2 = service.create_receiver();
///
///      spawn(move || {
///          for val in rx1 {
///              println!("Received val {}", val);
///          }
///      });
///
///      spawn(move || {
///          for val in rx2 {
///              println!("Received val {}", val);
///          }
///      });
///
///      tx.send("Hello");
///      tx.send("World"); // use tx.clone() for multiple producer use case
///```
pub struct BroadcastService<T> {
  subscription_tx: SyncSender<Sender<T>>
}


pub trait Broadcast<T> where T: Clone + Send + 'static {
    /// Creates a broadcast channel for single producer multiple consumer use case. Can be used for multiple
    /// producer too.
    /// Returns a tuple ([Sender<T>], [BroadcastService<T>]) where:
    ///
    ///   [Sender<T>] can be used to send messages. Note: this could also be cloned for multiple producer use case
    ///
    ///   [BroadcastService<T>] can be used to create multiple receivers by using [`create_receiver`] method
    ///
    /// Internally it creates a thread which keeps on listening to two channels:
    ///
    /// Subscription channel -> is used to listen to any new receivers registered on the broadcast channel
    ///
    /// Broadcast channel -> is used to listen to any new messages, and then broadcast those message to the
    /// registered subscribers
    fn create_channel() -> (Sender<T>, BroadcastService<T>);

    /// Creates a reciever to the broadcast channel. This function blocks the thread until receiver is actually
    /// subscribed to the broadcast message. Reason for this is to have clear semantics that reciever will not lose
    /// any message after this
    fn create_receiver(&mut self) -> Receiver<T>;
}

impl <T> Broadcast<T> for BroadcastService<T>
    where T: Clone + Send + 'static {
    fn create_channel() -> (Sender<T>, BroadcastService<T>) {
        // create subscription channel
        let (subscription_tx, subscription_rx) = sync_channel::<Sender<T>>(0);
        // service instance
        let service = BroadcastService {
            subscription_tx
        };
        // create broadcast message channel
        let (broadcast_tx, broadcast_rx) = channel::<T>();

        spawn(move || {
            let mut txn_map = HashMap::new::<>();
            let mut id = 0;
            let mut subscription_channel_open = true;
            loop {
                if subscription_channel_open {
                    match subscription_rx.try_recv() {
                        Ok(val) => {
                            id += 1;
                            txn_map.insert(id, val);
                        },
                        Err(TryRecvError::Disconnected) => {
                            println!("Subscription channel closed");
                            subscription_channel_open = false;
                        },
                        Err(TryRecvError::Empty) => {}
                    }
                }

                // prioritising broadcast channel for listening to new messages, here we block for
                // max 10ms
                let mut removal_set = HashSet::new();
                match broadcast_rx.recv_timeout(Duration::from_millis(10)) {
                    Ok(val) => {
                        for (id, txn) in &txn_map {
                            match txn.send(val.clone()) {
                                Ok(_) => {},
                                Err(_) => {
                                    removal_set.insert(*id);
                                },
                            };
                        }
                        txn_map.retain(|k, _| !removal_set.contains(k));
                    },
                    Err(RecvTimeoutError::Timeout) => {},
                    Err(RecvTimeoutError::Disconnected) => {
                        println!("Broadcast channel closed");
                        break;
                    }
                }
            }
        });
        (broadcast_tx, service)
    }

    fn create_receiver(&mut self) -> Receiver<T> {
        let (tx, rx) = channel::<T>();
        self.subscription_tx.send(tx.clone()).unwrap();
        rx
    }
}