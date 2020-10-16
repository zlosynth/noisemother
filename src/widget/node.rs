extern crate imgui;

use crate::vec2;
use crate::widget::pin_group::PinGroup;

const BLACK: [f32; 3] = [0.1, 0.1, 0.1];
const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

const BACKGROUND_COLOR: [f32; 3] = WHITE;

pub enum Component<'a, F>
where
    F: Fn(imgui::ImString),
{
    PinGroup(PinGroup<'a, F>),
}

pub struct Node<'a, F>
where
    F: Fn(imgui::ImString),
{
    id: &'a imgui::ImStr,
    position: [f32; 2],
    components: Vec<Component<'a, F>>,
}

impl<'a, F> Node<'a, F>
where
    F: Fn(imgui::ImString),
{
    pub fn new(id: &'a imgui::ImStr) -> Self {
        Self {
            id,
            position: [0.0, 0.0],
            components: Vec::new(),
        }
    }

    pub fn position(mut self, position: [f32; 2]) -> Self {
        self.position = position;
        self
    }

    pub fn add_component(mut self, component: Component<'a, F>) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(self, ui: &imgui::Ui<'_>) {
        let position = self.position;
        let size = self.get_size(ui);

        {
            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect(position, vec2::sum(&[position, size]), WHITE)
                .filled(true)
                .build();
        }

        for component in self.components.into_iter() {
            match component {
                Component::PinGroup(pin_group) => {
                    pin_group.position(position).build(ui);
                }
            };
        }
    }

    fn get_size(&self, ui: &imgui::Ui<'_>) -> [f32; 2] {
        let components_size = {
            self.components
                .iter()
                .map(|c| match c {
                    Component::PinGroup(pin_group) => pin_group.get_size(ui),
                })
                .fold([0.0 as f32, 0.0], |a, b| [a[0].max(b[0]), a[1] + b[1]])
        };

        components_size
    }
}
