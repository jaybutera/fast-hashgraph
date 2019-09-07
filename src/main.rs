//type EventId = usize;
struct EventId {
    genesis: bool,
    index: usize,
}
type PeerId = usize;
type Transaction = u32;

struct Generation<T> {
    data: T,
    generation: usize,
}

// Used to generate indices that aren't being used
// It's primitive right now but will be more useful when I add deletions
struct IndexAllocator {
    latest_idx: usize,
}
impl IndexAllocator {
    fn allocate(&mut self) -> usize {
        self.latest_idx += 1;
        self.latest_idx
    }
}

struct Graph {
    // Will never change
    genesis: Vec<bool>,
    self_parent: Vec<EventId>,
    other_parent: Vec< Option<EventId> >,
    txs: Vec< Option<Vec<Transaction>> >,
    creator: Vec<PeerId>,
    round: Vec<u32>,

    // May be changed after creation so these should be private
    famous: Vec< Generation<bool> >,
    witness: Vec< Generation<bool> >,

    // Internal tracking of update generations
    generation: usize,
}

#[derive(Serialize, Deserialize, Clone)] // Clone is temporary for graph unit tests
enum Event {
    Update {
        creator: PeerId,
        self_parent: PeerId,
        other_parent: Option<PeerId>,
        txs: Vec<Transaction>,
    },
    Genesis{creator: PeerId},
}

/*
impl Event {
    pub fn hash(&self) -> String {
        let mut hasher = Sha3::sha3_256();
        let serialized = serde_json::to_string(self).unwrap();
        hasher.input_str(&serialized[..]);
        hasher.result_str()
    }
}
*/


impl Graph {
    fn new() -> Self {
        let max = 1000;
        Graph {
            genesis:      Vec::with_capacity(max),
            self_parent:  Vec::with_capacity(max),
            other_parent: Vec::with_capacity(max),
            txs:          Vec::with_capacity(max),
            creator:      Vec::with_capacity(max),
            round:        Vec::with_capacity(max),
            famous:       Vec::with_capacity(max),
            witness:      Vec::with_capacity(max),
            generation:   0,
        }
    }

    fn get(&self, eid: &EventId) -> &Event {
    }

    fn add_event(&mut self, e: Event) -> EventId {
        match e {
            Genesis => 
        }
    }

    fn is_famous(&self, eid: &EventId) -> bool {
        //let event = self.get(eid);

        // Once an event is famous it won't become unfamous
        let self.famous[eid.index];
        if self.famous[eid.index].data == true {
            return true;
        }

        // If it's not famous, check if its generation is current enough to trust the data
        let cur = self.generation;
        if self.get(eid) != cur {
        }
    }

    fn is_witness(&self, eid: EventId) -> bool {
    }
}

fn main() {
    println!("Hello, world!");
}
