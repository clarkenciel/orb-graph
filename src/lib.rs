use std::collections::{HashMap};

struct Performer {
    id: PerformerId,
    heading: Heading,
    position: Position,
}

impl Performer {
    fn speaking_areas(&self) -> (Sector, Sector) {}

    fn hearing_areas(&self) -> (Sector, Sector) {}
}

type PerformerId = u64;

type Performers = HashMap<PerformerId, Performer>;

struct PerformerGraph {
    edges: HashMap<PerformerId, Edges>,
    performers: Performers, // feels like a big redundancy
}

struct Slots {
    right: Option<PerformerId>,
    left: Option<PerformerId>,
}

impl Slots {
    fn available_on(&self, s: &Side) -> bool {
        match s {
            Side::Left => self.left.is_none(),
            Side::Right => self.right.is_none(),
        }
    }
}

struct Edges {
    ears: Ears,
    mouth: Mouth,
}

struct Mouth(Slots);

struct Ears(Slots);

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
        match self {
            Mouth(slots) => slots.available_on(s),
        }
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
fn connection_is_possible(c: &Connection, edges: &HashMap<PerformerId, Edges>) -> bool {
    match (edges.get(&c.source), edges.get(&c.sink)) {
        (None, None) => true,
        (Some(source), None) => source.mouth.output_free(&c.source_side),
        (None, Some(sink)) => sink.ears.input_free(&c.sink_side),
        (Some(source), Some(sink)) => c.can_connect(&source.mouth, &sink.ears),
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

struct Heading(f64);
