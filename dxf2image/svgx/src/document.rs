use std::collections::HashMap;

use crate::nodes::Node;

use svg::node::element::Group;

pub struct Document(svg::Document, HashMap<String, Group>);

impl Document {
    pub fn new() -> Self {
        Self(svg::Document::new(), HashMap::new())
    }

    pub fn viewbox(self, x: f64, y: f64, width: f64, height: f64) -> Self {
        let doc = self
            .0
            .set("width", width)
            .set("height", height)
            .set("viewbox", (x, y, width, height));
        Self(doc, self.1)
    }

    pub fn layers(self) -> Vec<Group> {
        self.0.get_inner().get_children().iter();
        return vec![];
    }

    pub fn add_layer(&mut self, layer_name: String) {
        if !self.1.contains_key(&layer_name) {
            let group = Group::new()
                .set("groupmode", "layer")
                .set("label", layer_name.clone());

            self.1.insert(layer_name.clone(), group);
        }
    }

    pub fn add(&mut self, node: impl Node, layer_name: Option<String>) {
        if (layer_name.is_some()) {
            let layer_name = layer_name.unwrap();
            let layer_group = self
                .1
                .get_mut(&layer_name)
                .unwrap()
                .to_owned()
                .add(node.to_element());

            self.1.insert(layer_name.clone(), layer_group);
        } else {
            self.0 = self.0.to_owned().add(node.to_element());
        }
    }

    pub fn save(self, path: &str) -> anyhow::Result<()> {
        let mut document = self.0.to_owned();

        self.1
            .into_values()
            .collect::<Vec<Group>>()
            .iter()
            .for_each(|layer| {
                document = document.clone().add(layer.to_owned());
            });

        svg::save(path, &document).map_err(|e| e.into())
    }
}
