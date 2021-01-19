use async_std::{
    channel::{unbounded, Sender},
    task,
};
use std::time::Duration;

async fn event_generator(event_sender: Sender<usize>) {
    println!("Event generator is running >>>>>>>\n",);
    for event in 1..11 {
        task::sleep(Duration::from_secs(1)).await;
        let _ = event_sender.send(event);

        // If you don't call `.await`, then that cold future will never run
        // which means the channel WILL NOT have any event there!!!
        // let _ = event_sender.send(event);
        let _ = event_sender.send(event).await;

        println!("sended event: {}", event);
    }
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let (event_sender, event_receiver) = unbounded();

    let _ = task::spawn(event_generator(event_sender));

    let event_consumer = task::spawn(async move {
        println!("event_consumer is running >>>>>>\n");

        while let Ok(temp_event) = event_receiver.recv().await {
            println!("Got event: {:?}", temp_event);
            task::sleep(Duration::from_secs(1)).await;
        }
    });

    task::block_on(event_consumer);

    println!("All tasks are done");

    Ok(())
}
