// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::{
    Align, Button, Checkbox, Column, DynLabel, Label, Padding, ProgressBar, Row, Slider,
};
use druid::{AppLauncher, Data, LensWrap, Widget, WindowDesc};

#[derive(Clone, Data)]
struct DemoState {
    value: f64,
    double: bool,
}

mod lenses {
    pub mod demo_state {
        use super::super::DemoState;
        use druid::Lens;
        pub struct Value;
        pub struct Double;

        impl Lens<DemoState, f64> for Value {
            fn get<'a>(&self, data: &'a DemoState) -> &'a f64 {
                &data.value
            }

            fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut DemoState, f: F) -> V {
                f(&mut data.value)
            }
        }

        impl Lens<DemoState, bool> for Double {
            fn get<'a>(&self, data: &'a DemoState) -> &'a bool {
                &data.double
            }

            fn with_mut<V, F: FnOnce(&mut bool) -> V>(&self, data: &mut DemoState, f: F) -> V {
                f(&mut data.double)
            }
        }
    }
}

fn build_widget() -> impl Widget<DemoState> {
    let mut col = Column::new();
    let label = DynLabel::new(|data: &DemoState, _env| {
        if data.double {
            format!("2x the value: {0:.2}", data.value * 2.0)
        } else {
            format!("actual value: {0:.2}", data.value)
        }
    });
    let mut row = Row::new();
    let checkbox = LensWrap::new(Checkbox::new(), lenses::demo_state::Double);
    let checkbox_label = Label::new("double the value");
    row.add_child(checkbox, 0.0);
    row.add_child(Padding::uniform(5.0, checkbox_label), 1.0);

    let bar = LensWrap::new(ProgressBar::new(), lenses::demo_state::Value);
    let slider = LensWrap::new(Slider::new(), lenses::demo_state::Value);

    let button_1 = Button::sized(
        "increment ",
        |_ctx, data: &mut DemoState, _env| data.value += 0.1,
        200.0,
        100.0,
    );
    let button_2 = Button::new("decrement ", |_ctx, data: &mut DemoState, _env| {
        data.value -= 0.1
    });

    col.add_child(Padding::uniform(5.0, bar), 1.0);
    col.add_child(Padding::uniform(5.0, slider), 1.0);
    col.add_child(Padding::uniform(5.0, label), 1.0);
    col.add_child(Padding::uniform(5.0, row), 1.0);
    col.add_child(Padding::uniform(5.0, Align::right(button_1)), 0.0);
    col.add_child(Padding::uniform(5.0, button_2), 1.0);
    col
}

fn main() {
    let window = WindowDesc::new(build_widget);
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(DemoState {
            value: 0.7f64,
            double: false,
        })
        .expect("launch failed");
}
