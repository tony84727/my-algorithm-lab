use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Packet {
    source: i32,
    destination: i32,
    timestamp: i32,
}

pub struct Router {
    queue: VecDeque<Packet>,
    destinations: HashMap<i32, BTreeMap<i32, usize>>,
    memory_limit: usize,
    packets: HashSet<Packet>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    pub fn new(memory_limit: i32) -> Self {
        let memory_limit = memory_limit as usize;
        Self {
            queue: VecDeque::with_capacity(memory_limit),
            destinations: HashMap::new(),
            packets: HashSet::new(),
            memory_limit,
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet {
            source,
            destination,
            timestamp,
        };
        if self.packets.contains(&packet) {
            return false;
        }
        *self
            .destinations
            .entry(destination)
            .or_default()
            .entry(timestamp)
            .or_default() += 1;
        self.packets.insert(packet.clone());
        self.queue.push_front(packet);
        if self.queue.len() > self.memory_limit {
            self.pop_packet();
        }
        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        let Some(Packet {
            source,
            destination,
            timestamp,
        }) = self.pop_packet()
        else {
            return Vec::new();
        };
        vec![source, destination, timestamp]
    }

    pub fn get_count(&mut self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        self.destinations
            .entry(destination)
            .or_default()
            .range(start_time..=end_time)
            .map(|(_, v)| *v)
            .sum::<usize>() as i32
    }

    fn pop_packet(&mut self) -> Option<Packet> {
        let p = self.queue.pop_back()?;
        *self
            .destinations
            .entry(p.destination)
            .or_default()
            .entry(p.timestamp)
            .or_default() -= 1;
        self.packets.remove(&p);
        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_example_1() {
        let mut router = Router::new(3);
        assert!(router.add_packet(1, 4, 90));
        assert!(router.add_packet(2, 5, 90));
        assert!(!router.add_packet(1, 4, 90));
        assert!(router.add_packet(3, 5, 95));
        assert!(router.add_packet(4, 5, 105));
        assert_eq!(vec![2, 5, 90], router.forward_packet());
        assert_eq!(1, router.get_count(5, 100, 110));
    }

    #[test]
    fn test_solution_example_2() {
        let mut router = Router::new(2);
        assert!(router.add_packet(7, 4, 90));
        assert_eq!(vec![7, 4, 90], router.forward_packet());
        assert_eq!(Vec::<i32>::new(), router.forward_packet());
    }
}
