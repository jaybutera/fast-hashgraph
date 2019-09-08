type EventId = usize;
type PeerId = usize;
type Transaction = u32;

struct Generation<T> {
    value: T,
    generation: usize,
}

// Used to generate indices that aren't being used
// It's primitive right now but will be more useful when I add deletions
struct IndexAllocator {
    latest_idx: usize,
}
impl IndexAllocator {
    fn allocate(&mut self) -> usize {
        let i = self.latest_idx;
        self.latest_idx += 1;
        i
    }

    /*
    fn peek(&self) -> usize {
        self.latest_idx
    }
    */
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
    allocator: IndexAllocator,
}

//#[derive(Serialize, Deserialize, Clone)] // Clone is temporary for graph unit tests
enum Event {
    Update {
        creator: PeerId,
        self_parent: PeerId,
        other_parent: Option<PeerId>,
        txs: Option< Vec<Transaction> >,
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
        let allocator = IndexAllocator{ latest_idx: 0 };

        let mut genesis = Vec::with_capacity(max);
        let mut self_parent = Vec::with_capacity(max);
        let mut other_parent = Vec::with_capacity(max);
        let mut txs = Vec::with_capacity(max);
        let mut creator = Vec::with_capacity(max);
        let mut round = Vec::with_capacity(max);
        let mut famous = Vec::with_capacity(max);
        let mut witness = Vec::with_capacity(max);

        unsafe {
            genesis.set_len(max);
            self_parent.set_len(max);
            other_parent.set_len(max);
            txs.set_len(max);
            creator.set_len(max);
            round.set_len(max);
            famous.set_len(max);
            witness.set_len(max);
        }

        Graph {
            genesis:      genesis,
            self_parent:  self_parent,
            other_parent: other_parent,
            txs:          txs,
            creator:      creator,
            round:        round,
            famous:       famous,
            witness:      witness,
            generation:   0,
            allocator:    allocator,
        }
    }

    /*
    fn get(&self, eid: &EventId) -> &Event {
    }
    */

    fn add_event(&mut self, e: Event) -> EventId {
        let eid = self.allocator.allocate();

        match e {
            Event::Genesis{ creator } => {
                self.genesis[eid] = true;
                self.creator[eid] = creator;
            },
            Event::Update{ creator, self_parent, other_parent, txs } => {
                self.creator[eid] = creator;
                self.self_parent[eid] = self_parent;
                self.other_parent[eid] = other_parent;
                self.txs[eid] = txs;
            }
        }

        // Update the current generation
        self.generation += 1;

        eid
    }

    fn is_famous(&self, eid: EventId) -> bool {
        //let event = self.get(eid);

        // Once an event is famous it won't become unfamous
        let fame = &self.famous[eid];
        if fame.value == true {
            return true;
        }

        // If it's not famous, check if its generation is current enough to trust the data
        let cur = self.generation;
        if fame.generation == cur {
            // Should always be false
            return fame.value;
        } else {
            // Need to recheck and update fame status

            return true;
        }
    }

    fn reachability_matrix(&self) {//, x: EventId, y: EventId) {
        // TODO: Is there a cleaner way to initialize this array?
        let len = self.allocator.latest_idx;
        let mut reach = Vec::with_capacity( len );
        for i in 0..len {
            let mut v = Vec::with_capacity( len );
            for _ in 0..len {
                v.push(false);
            }

            reach.push(v);
        }

        // Initialize with the zero pass (immediate reachability)
        for eid in 0..len {
            if self.genesis[eid] { continue }

            let sid = self.self_parent[eid];
            reach[eid][sid] = true;

            if let Some(oid) = self.other_parent[eid] {
                reach[eid][oid] = true;
            }
        }

        for k in 0..len {
            for i in 0..len {
                for j in 0..len {
                    reach[i][j] = reach[i][j] || ( reach[i][k] && reach[k][j] );
                }
            }
        }

        println!("{:?}", reach);
    }

    /*
    fn determine_famous(&self, eid: EventId) -> bool {
        // TODO: Filter all events to only those with round >= eid's round

        // Compute transtive closure with Floyd-Warshall
        // An event is famous if 2/3 future witnesses strongly see it.
    }

    fn is_witness(&self, eid: EventId) -> bool {
    }

    fn strongly_see(&self, x_id: EventId, y_id: EventId) -> bool {
        // Kruskal's algorithm
    }
    */
}

fn main() {
    let mut g = Graph::new();

    let id1 = g.add_event(Event::Genesis { creator: 0});
    let id2 = g.add_event(Event::Genesis { creator: 1});
    let id3 = g.add_event(Event::Update {
        creator: 2,
        self_parent: id1,
        other_parent: Some(id2),
        txs: Some( Vec::new() ),
    });
    let id4 = g.add_event(Event::Update {
        creator: 1,
        self_parent: id2,
        other_parent: Some(id3),
        txs: Some( Vec::new() ),
    });

    g.reachability_matrix();
}

/*
  1 2 3 4
1 0 0 0 0
2 0 0 0 0
3 1 1 0 0
4 1 1 1 0
*/
