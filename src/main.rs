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
    size: usize,
}
impl IndexAllocator {
    fn allocate(&mut self) -> usize {
        let i = self.latest_idx;
        self.latest_idx += 1;
        self.size += 1;
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
    reachable: Vec<Vec<bool>>,

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
        let allocator = IndexAllocator{ latest_idx: 0, size: 0 };

        let mut genesis = Vec::with_capacity(max);
        let mut self_parent = Vec::with_capacity(max);
        let mut other_parent = Vec::with_capacity(max);
        let mut txs = Vec::with_capacity(max);
        let mut creator = Vec::with_capacity(max);
        let mut round = Vec::with_capacity(max);
        let mut famous = Vec::with_capacity(max);
        let mut witness = Vec::with_capacity(max);
        let mut reachable = Vec::with_capacity(max);

        unsafe {
            genesis.set_len(max);
            self_parent.set_len(max);
            other_parent.set_len(max);
            txs.set_len(max);
            creator.set_len(max);
            round.set_len(max);
            famous.set_len(max);
            witness.set_len(max);
            reachable.set_len(max);
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
            reachable:    reachable,
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
                self.reachable[eid] = self.reachable_events(&self_parent, &other_parent);
            }
        }

        // Update the current generation
        self.generation += 1;

        eid
    }

    // NOTE: This fn does not check whether the event's parents are valid and stored in the graph,
    // may lead to a panic
    fn reachable_events(&self,
                        self_parent: &EventId,
                        other_parent: &Option<EventId>)-> Vec<bool>
    {
        let len           = self.allocator.size;
        let mut reachable = Vec::with_capacity(len);

        // OR together the parent reachability lists
        let p1_reachable = &self.reachable[*self_parent];
        if let Some(op) = other_parent {
            let p2_reachable = &self.reachable[*op];

            for i in 0..len {
                let x1 = *p1_reachable.get(i).unwrap_or(&false);
                let x2 = *p2_reachable.get(i).unwrap_or(&false);

                reachable.push( x1 || x2 );
            }
        }
        else {
            for i in 0..len {
                let x1 = *p1_reachable.get(i).unwrap_or(&false);
                reachable.push( x1 );
            }
        }

        // Set the parents to be reachable
        reachable[*self_parent]  = true;
        if let Some(op) = other_parent {
            reachable[*op] = true;
        }

        reachable
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

    fn reachability_matrix(&self) -> Vec<Vec<bool>> {
        // TODO: Is there a cleaner way to initialize this array?
        let len = self.allocator.latest_idx;
        let mut reach = Vec::with_capacity( len );
        for _ in 0..len {
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

        //println!("{:?}", reach);
        reach
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

// Store the reachability data as an adjacency list since it is sparse and only the lower triangle
// is needed. Since a new event can only be added when its parents exist, no node can point to a
// newly added event before it is added. Therefore there's no need to update any previous
// reachability data. Only to fill out the list of the newly added event. This can be done with
// kruskal's algorithm in O(n^2).

/*
  1 2 3 4
1 0 0 0 0
2 0 0 0 0
3 1 1 0 0
4 1 1 1 0
*/
