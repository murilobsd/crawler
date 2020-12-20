use queue::Queue;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn queue() {
        let mut q = Queue::new();
        assert_eq!(q.queue("url"), Ok(1));
    }

    #[test]
    fn dequeue() {
        let mut q = Queue::new();
        q.queue("url").unwrap();
        assert_eq!(q.dequeue(), Some("url"));
    }
}
