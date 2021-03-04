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

License: MIT OR Apache-2.0
