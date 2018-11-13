use std::collections::{HashMap,HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct Performer {
    id: PerformerId,
    heading: Heading,
    position: Position,
}

impl Performer {
    fn speaking_areas(&self) -> (Sector, Sector) {}

    fn hearing_areas(&self) -> (Sector, Sector) {}
}

impl PartialEq for Performer {
    fn eq(&self, o: &Self) -> bool {
        self.id == o.id
    }
}

impl Eq for Performer {}

impl Hash for Performer {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.id.hash(h);
    }
}

type PerformerId = u64;

type Performers = HashSet<Performer>;

struct PerformerIndex(HashMap<PerformerId, Performer>);

impl PerformerIndex {
    fn from(ps: &Performers) -> Self {
        PerformerIndex(ps.iter().map(|&p| (p.id, p.clone())).collect())
    }
}

struct PerformerGraph {
    edges: HashMap<PerformerId, Edges>,
}

impl PerformerGraph {
    fn new() -> Self {
        Self { edges: HashMap::new() }
    }

    fn connections_count(&self) -> u64 {
        self.edges.values().map(|es| es.count()).sum()
    }

    fn connect(&mut self, c: &Connection) {
        match (self.edges.get_mut(&c.source), self.edges.get_mut(&c.sink)) {
            (None, None) => {
                self.edges.insert(c.source, Edges {
                    ears: Ears(Slots::empty()),
                    mouth: Mouth(Slots::with(&c.source_side, &c.sink)),
                });

                self.edges.insert(c.sink, Edges {
                    mouth: Mouth(Slots::empty()),
                    ears: Ears(Slots::with(&c.sink_side, &c.source)),
                });
            },

            (Some(Edges { mouth: Mouth(slots), .. }), None) => {
                slots.add(&c.source_side, &c.sink);
                self.edges.insert(c.sink, Edges {
                    mouth: Mouth(Slots::empty()),
                    ears: Ears(Slots::with(&c.sink_side, &c.source)),
                });
            }

            (None, Some(Edges { ears: Ears(slots), .. })) => {
                slots.add(&c.sink_side, &c.source);
                self.edges.insert(c.source, Edges {
                    ears: Ears(Slots::empty()),
                    mouth: Mouth(Slots::with(&c.source_side, &c.sink)),
                });
            },

            (Some(Edges { mouth: Mouth(m_slots), .. }), Some(Edges { ears: Ears(e_slots), .. })) => {
                e_slots.add(&c.sink_side, &c.source);
                m_slots.add(&c.source_side, &c.sink);
            },
        }
    }
}

struct Slots {
    right: Option<PerformerId>,
    left: Option<PerformerId>,
}

impl Slots {
    fn empty() -> Self {
        Self { left: None, right: None }
    }

    fn with(s: &Side, p: &PerformerId) -> Self {
        match s {
            Side::Left => Self { left: Some(p.clone()), right: None },
            Side::Right => Self { right: Some(p.clone()), left: None },
        }
    }

    fn available_on(&self, s: &Side) -> bool {
        match s {
            Side::Left => self.left.is_none(),
            Side::Right => self.right.is_none(),
        }
    }

    fn add(&mut self, s: &Side, p: &PerformerId) {
        match s {
            Side::Left => self.left = Some(p.clone()),
            Side::Right => self.right = Some(p.clone()),
        };
    }

    fn has(&self, p: &PerformerId) -> bool {
        match (self.right, self.left) {
            (None, None) => false,
            (Some(p2), None) => *p == p2,
            (None, Some(p2)) => *p == p2,
            (Some(p2), Some(p3)) => *p == p2 || *p == p3,
        }
    }

    fn filled(&self) -> u64 {
        self.left.map(|_| 1).unwrap_or(0) + self.right.map(|_| 1).unwrap_or(0)
    }
}

struct Edges {
    ears: Ears,
    mouth: Mouth,
}

impl Edges {
    fn count(&self) -> u64 {
        self.ears.speakers() + self.mouth.listeners()
    }
}

struct Mouth(Slots);

impl Mouth {
    fn talking_to(&self, p: &PerformerId) -> bool {
        let Mouth(slots) = self;
        slots.has(p)
    }

    fn listeners(&self) -> u64 {
        let Mouth(slots) = self;
        slots.filled()
    }
}

struct Ears(Slots);

impl Ears {
    fn listening_to(&self, p: &PerformerId) -> bool {
        let Ears(slots) = self;
        slots.has(p)
    }

    fn speakers(&self) -> u64 {
        let Ears(slots) = self;
        slots.filled()
    }
}

enum Side {
    Left,
    Right,
}

struct Connection {
    source: PerformerId,
    source_side: Side,
    sink: PerformerId,
    sink_side: Side,
}

trait Source {
    fn output_free(&self, &Side) -> bool;
}

impl Source for Mouth {
    fn output_free(&self, s: &Side) -> bool {
        let Mouth(slots) = self;
        slots.available_on(s)
    }
}

trait Sink {
    fn input_free(&self, &Side) -> bool;
}

impl Sink for Ears {
    fn input_free(&self, s: &Side) -> bool {
        match self {
            Ears(slots) => slots.available_on(s),
        }
    }
}

impl Connection {
    fn can_connect(&self, m: &Source, e: &Sink) -> bool {
        m.output_free(&self.source_side) && e.input_free(&self.sink_side)
    }
}

// return connections that can be made given a set of edges
fn possible_connections(cs: Vec<Connection>, edges: &HashMap<PerformerId, Edges>) -> Vec<Connection> {
    cs.iter().filter(|&c| connection_is_possible(c, &edges)).map(|&c| c).collect()
}

// a connection can be made in an edge set under the following conditions:
//   1. neither source nor sink exist in the edge set yet
//   2. source exists, has an available mouth slot and sink does not exist
//   3. sink exists, has an available ear slot and source does not exist
//   4. source and sink exist, connection can connect them
// *NB* I think checking `talking_to` and `listening_to` is overkill; if
//      a performer isn't `talking_to` another performer, the other performer
//      _shouldn't_ be `listening_to` the source, but.....
fn connection_is_possible(c: &Connection, edges: &HashMap<PerformerId, Edges>) -> bool {
    match (edges.get(&c.source), edges.get(&c.sink)) {
        (None, None) => true,
        (Some(source), None) =>
            !source.mouth.talking_to(&c.sink) &&
            source.mouth.output_free(&c.source_side),
        (None, Some(sink)) =>
            !sink.ears.listening_to(&c.source) &&
            sink.ears.input_free(&c.sink_side),
        (Some(source), Some(sink)) =>
            !source.mouth.talking_to(&c.sink) &&
            !sink.ears.listening_to(&c.source) &&
            c.can_connect(&source.mouth, &sink.ears),
    }
}

fn connections(p1: &Performer, p2: &Performer) -> Vec<Connection> {
    let (left_audience, right_audience) = p1.speaking_areas();
    let (audible_l, audible_r) = p2.hearing_areas();
    let mut connections = vec![];

    if left_audience.contains(&p2.position) && audible_l.contains(&p1.position) {
        connections.push(Connection {
            source: p1.id,
            source_side: Side::Left,
            sink: p2.id,
            sink_side: Side::Left,
        })
    }

    if left_audience.contains(&p2.position) && audible_r.contains(&p1.position) {
        connections.push(Connection {
            source: p1.id,
            source_side: Side::Left,
            sink: p2.id,
            sink_side: Side::Right,
        })
    }

    if right_audience.contains(&p2.position) && audible_l.contains(&p1.position) {
        connections.push(Connection {
            source: p1.id,
            source_side: Side::Right,
            sink: p2.id,
            sink_side: Side::Left,
        })
    }

    if right_audience.contains(&p2.position) && audible_r.contains(&p1.position) {
        connections.push(Connection {
            source: p1.id,
            source_side: Side::Right,
            sink: p2.id,
            sink_side: Side::Right,
        })
    }

    connections
}

struct Sector {
    radius: f64,
    center: Position,
    start: Vector,
    end: Vector,
}

impl Sector {
    fn contains(&self, p: &Position) -> bool {
        let vec = p.displacement(&self.center);
        self.end.clockwise_of(&vec) && vec.clockwise_of(&self.start) &&
            vec.magnitude() < self.radius
    }
}

#[derive(Clone)]
struct Position(f64, f64);

impl Position {
    fn displacement(&self, other: &Position) -> Vector {
        Vector(other.0 - self.0, other.1 - self.1)
    }
}

struct Vector(f64, f64);

impl Vector {
    // taken from:
    // https://stackoverflow.com/questions/13652518/efficiently-find-points-inside-a-circle-sector#13675772
    fn clockwise_of(&self, other: &Vector) -> bool {
        self.inverse_normal().dot(other) <= 0f64
    }

    fn normal(&self) -> Vector {
        Vector(self.1, -self.0)
    }

    fn inverse_normal(&self) -> Vector {
        Vector(-self.1, self.0)
    }

    fn magnitude(&self) -> f64 {
        self.0
    }

    fn direction(&self) -> f64 {
        self.1
    }

    fn dot(&self, other: &Vector) -> f64 {
        (self.0 * other.0) + (self.1 * other.1)
    }
}

#[derive(Clone)]
struct Heading(f64);

// intuitively, this should be float, but i know that floats
// are kind of a pain in rust vis a vis sorting, so here we are!
type Score = u64;

type ScoredGraph = (PerformerGraph, Score);

fn evaluate(ps: &Performers) -> Option<ScoredGraph> {
    let index = PerformerIndex::from(ps);
    ps.iter()
        .flat_map(|p| graphs(p, &index).iter().map(|&g| (g, g.connections_count())))
        .max_by_key(|&(_, c)| c)
}

fn graphs(start: &Performer, index: &PerformerIndex) -> Vec<PerformerGraph> {
    vec![]
}

fn build_graph(start: &Performer, index: &PerformerIndex) -> PerformerGraph {
    let mut graph = PerformerGraph::new();
    graph
}
