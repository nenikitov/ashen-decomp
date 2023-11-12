use crate::asset::Asset;

pub trait Directory {
    fn get<A: Asset>(&self, id: &str) -> A;
    fn get_all<A: Asset>(&self) -> Vec<A>;
}
