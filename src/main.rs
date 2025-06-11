use sophia::api::MownStr;
use std::collections::HashSet;
use sophia::api::prelude::*;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ResourceId<'a>(IriRef<MownStr<'a>>);

pub struct LdpContainer<'a, G> {
    graph: &'a G,
}

impl<'a, G: Graph> LdpContainer<'a, G> {
    pub fn members(&self) -> HashSet<ResourceId> {
        let mut members: HashSet<ResourceId<'a>> = HashSet::new();
        for result in self.graph.triples_matching(Any, Any, Any) {
            match result {
                Ok(triple) => {
                    let object = triple.o();
                    let iri = object.iri().unwrap();
                    members.insert(ResourceId(iri));
                }
                _ => {}
            }
        }

        members
    }
}

fn main() {
}
