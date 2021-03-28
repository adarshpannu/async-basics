// See https://cfsamson.github.io/book-exploring-async-basics/8_0_implementing_our_own_runtime.html
// See https://github.com/cfsamson/examples-node-eventloop



#![allow(unused_variables)]
fn main() {
    pub struct Runtime {
        /// Available threads for the threadpool
        available_threads: Vec<usize>,
        /// Callbacks scheduled to run
        callbacks_to_run: Vec<(usize, Js)>,
        /// All registered callbacks
        callback_queue: HashMap<usize, Box<dyn FnOnce(Js)>>,
        /// Number of pending epoll events, only used by us to print for this example
        epoll_pending_events: usize,
        /// Our event registrator which registers interest in events with the OS
        epoll_registrator: minimio::Registrator,
        // The handle to our epoll thread
        epoll_thread: thread::JoinHandle<()>,
        /// None = infinite, Some(n) = timeout in n ms, Some(0) = immediate
        epoll_timeout: Arc<Mutex<Option<i32>>>,
        /// Channel used by both our threadpool and our epoll thread to send events
        /// to the main loop
        event_reciever: Receiver<PollEvent>,
        /// Creates an unique identity for our callbacks
        identity_token: usize,
        /// The number of events pending. When this is zero, we're done
        pending_events: usize,
        /// Handles to our threads in the threadpool
        thread_pool: Vec<NodeThread>,
        /// Holds all our timers, and an Id for the callback to run once they expire
        timers: BTreeMap<Instant, usize>,
        /// A struct to temporarely hold timers to remove. We let Runtinme have
        /// ownership so we can reuse the same memory
        timers_to_remove: Vec<Instant>,
    }
}
