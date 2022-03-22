use geo::Coordinate;
use rust_editor::actions::{Action, Redo, Undo};
use uuid::Uuid;

use crate::map::{intersection::Intersection, map::Map};

pub(crate) struct CreateIntersection {
    position: Coordinate<f64>,
    pub(crate) id: Uuid,
}

impl CreateIntersection {
    pub fn new(position: Coordinate<f64>) -> Self {
        CreateIntersection::new_with_id(
            position,
            Uuid::new_v4(),
        )
    }

    pub fn new_with_id(position: Coordinate<f64>, id: Uuid) -> Self {
        CreateIntersection {
            position,
            id
        }
    }
}

impl Undo<Map> for CreateIntersection {
    fn undo(&mut self, map: &mut Map) {
        map.remove_intersection(&self.id);
    }
}

impl Redo<Map> for CreateIntersection {
    fn redo(&mut self, map: &mut Map) {
        map.add_intersection(Intersection::new_with_id(self.position, self.id));
    }
}

impl Action<Map> for CreateIntersection {}
