use crate::prelude::*;
use crate::{v_stack, Buffer, Icon, IconButton, Label};

#[derive(RenderOnce)]
pub struct MultiBuffer {
    buffers: Vec<Buffer>,
}

impl<V: 'static> Component<V> for MultiBuffer {
    type Rendered = Div<V>;

    fn render(self, view: &mut V, cx: &mut ViewContext<V>) -> Self::Rendered {
        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .children(self.buffers.clone().into_iter().map(|buffer| {
                v_stack()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_4()
                            .bg(cx.theme().colors().editor_subheader_background)
                            .child(Label::new("main.rs"))
                            .child(IconButton::new("arrow_up_right", Icon::ArrowUpRight)),
                    )
                    .child(buffer)
            }))
    }
}

impl MultiBuffer {
    pub fn new(buffers: Vec<Buffer>) -> Self {
        Self { buffers }
    }
}

use gpui::{Div, RenderOnce};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{hello_world_rust_buffer_example, Story};
    use gpui::{Div, Render};

    pub struct MultiBufferStory;

    impl Render<Self> for MultiBufferStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, MultiBuffer>(cx))
                .child(Story::label(cx, "Default"))
                .child(MultiBuffer::new(vec![
                    hello_world_rust_buffer_example(cx),
                    hello_world_rust_buffer_example(cx),
                    hello_world_rust_buffer_example(cx),
                    hello_world_rust_buffer_example(cx),
                    hello_world_rust_buffer_example(cx),
                ]))
        }
    }
}
