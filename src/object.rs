

pub(crate) struct Object {
    pub description: String,
    pub tag: String,
    pub location: Option<usize>,
}

impl Object {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tag == tag
    }
}

pub(crate) const FIELD: usize = 0;
pub(crate) const CAVE: usize = 1;
pub(crate) const SILVER: usize = 2;
pub(crate) const GOLD: usize = 3;
pub(crate) const GUARD: usize = 4;
pub(crate) const PLAYER: usize = 5;

pub(crate) fn build_objects() -> Vec<Object> {
    vec![
        Object { description: "an open field".into(), tag: "field"   .into(), location: None  },
        Object { description: "a little cave".into(), tag: "cave"    .into(), location: None  },
        Object { description: "a silver coin".into(), tag: "silver"  .into(), location: Some(FIELD) },
        Object { description: "a gold coin"  .into(), tag: "gold"    .into(), location: Some(CAVE) },
        Object { description: "a burly guard".into(), tag: "guard"   .into(), location: Some(FIELD) },
        Object { description: "yourself"     .into(), tag: "yourself".into(), location: Some(FIELD) }
    ]
}