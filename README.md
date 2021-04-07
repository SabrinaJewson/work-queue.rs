# work-queue

A concurrent work-stealing queue for building schedulers.

## Examples

Distribute some tasks in a thread pool:

```rust
use work_queue::{Queue, LocalQueue};

struct Task(Box<dyn Fn(&mut LocalQueue<Task>) + Send>);

let threads = 4;

let queue: Queue<Task> = Queue::new(threads, 128);

// Push some tasks to the queue.
for _ in 0..500 {
    queue.push(Task(Box::new(|local| {
        do_work();

        local.push(Task(Box::new(|_| do_work())));
        local.push(Task(Box::new(|_| do_work())));
    })));
}

// Spawn threads to complete the tasks.
let handles: Vec<_> = queue
    .local_queues()
    .map(|mut local_queue| {
        std::thread::spawn(move || {
            while let Some(task) = local_queue.pop() {
                task.0(&mut local_queue);
            }
        })
    })
    .collect();

for handle in handles {
    handle.join().unwrap();
}
```

## Comparison with crossbeam-deque

This crate is similar in purpose to [`crossbeam-deque`](https://docs.rs/crossbeam-deque), which
also provides concurrent work-stealing queues. However there are a few notable differences:

- This crate is more high level - work stealing is done automatically when calling `pop`
instead of you having to manually call it.
- As such, we do not support as much customization as `crossbeam-deque` - but the algorithm
itself can be optimized better.
- Queues have a fixed number of local queues that they support, and this number cannot grow.
- Each local queue has a fixed capacity, unlike `crossbeam-deque` which supports local queue
growth. This makes our local queues faster.

## Implementation

This crate's queue implementation is based off [Tokio's current scheduler]. The idea is that
each thread holds a fixed-capacity local queue, and there is also an unbounded global queue
accessible by all threads. In the general case each worker thread will only interact with its
local queue, avoiding lots of synchronization - but if one worker thread happens to have a
lot less work than another, it will be spread out evenly due to work stealing.

Additionally, each local queue stores a [non-stealable LIFO slot] to optimize for message
passing patterns, so that if one task creates another, that created task will be polled
immediately, instead of only much later when it reaches the front of the local queue.

[Tokio's current scheduler]: https://tokio.rs/blog/2019-10-scheduler
[non-stealable LIFO slot]: https://tokio.rs/blog/2019-10-scheduler#optimizing-for-message-passing-patterns

## Testing

- Test it normally using `cargo test`
- Test it with Miri using `cargo +nightly miri test`
- Test it with ThreadSanitizer using `RUSTFLAGS="-Zsanitizer=thread --cfg tsan" cargo +nightly test --tests -Zbuild-std --target={your target triple}`
- Test it with Loom using `RUSTFLAGS="--cfg loom" cargo test --tests --release`

## License

MIT OR Apache-2.0
