use std::collections::{HashMap,HashSet};

struct Performer {
    id: PerformerId,
    heading: Heading,
    position: Position,
}

impl Performer {
    fn speaking_area(&self) -> ArcSector {
    }

    fn hearing_areas(&self) -> (ArcSector, ArcSector) {
    }
}

type PerformerId = u64;

struct Position(f64, f64);

struct Heading(f64);

type Performers = HashSet<Performer>;

struct PerformerGraph {
    edges: HashMap<PerformerId, Edges>,
    performers: HashMap<PerformerId, Performer>, // feels like a big redundancy
}

struct Edges {
    ears: (Option<PerformerId>, Option<PerformerId>),
    mouth: (Option<PerformerId>, Option<PerformerId>),
}

fn connectable(p1: &Performer,  p2: &Performer) -> bool {
    let audience = p1.speaking_area();
    let (audible_l, audible_r) = p2.hearing_areas();
    audience.contains(&p2.position) && (audience.intersects(&audible_l) || audience.intersects(&audible_r))
}

struct ArcSector {
}

impl ArcSector {
    fn contains(&self, p: &Position) -> bool {
    }

    fn intersects(&self, other: &Self) -> bool {
    }
}
