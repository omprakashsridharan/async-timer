use {
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc,Mutex},
        task::{Context,Poll,Waker},
        thread,
        time::Duration
    }
};

pub struct SharedState {
    completed: bool,
    waker: Option<Waker>
}

pub struct TimerFuture{
    shared_state: Arc<Mutex<SharedState>>
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>,cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        }else{
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    fn new(duration: Duration) -> Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move ||{
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take(){
                waker.wake()
            }
        });
        TimerFuture{shared_state}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world");
    tokio::spawn(async move {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    println!("outside");
        TimerFuture::new(Duration::new(5, 0)).await;
        println!("done outside!");
    Ok(())
}