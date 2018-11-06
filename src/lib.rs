use std::collections::{HashMap,HashSet};

struct Performer {
    id: PerformerId,
    heading: Heading,
    position: Position,
}

impl Performer {
    fn speaking_area(&self) -> Sector {
    }

    fn hearing_areas(&self) -> (Sector, Sector) {
    }
}

type PerformerId = u64;


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

struct Sector {
    radius: f64,
    center: Position,
    start: Vector,
    end: Vector,
}

impl Sector {
    fn contains(&self, p: &Position) -> bool {
        let vec = p.displacement(&self.center);
        self.end.clockwise_of(&vec) && vec.clockwise_of(&self.start) && vec.magnitude() < self.radius
    }

    fn intersects(&self, other: &Self) -> bool {
    }
}

struct Position(f64, f64);

impl Position {
    fn displacement(&self, other: &Position) -> Vector {
        Vector(
            other.0 - self.0,
            other.1 - self.1,
        )
    }
}

struct Vector(f64, f64);

impl Vector {
    // taken from: https://stackoverflow.com/questions/13652518/efficiently-find-points-inside-a-circle-sector#13675772
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
