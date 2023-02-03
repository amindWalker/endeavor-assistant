use dioxus::prelude::*;
// use wasm_bindgen::prelude::*;
// use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

pub struct LineChart {
    days: Vec<&'static str>,
    progress: Vec<f32>,
}
impl LineChart {
    fn week_chart() -> Self {
        LineChart {
            days: vec!["M", "T", "W", "T", "F", "S", "S"],
            progress: vec![0.2, 0.1, 0.5, 0.6, 1.0, 0.8, 0.7],
        }
    }
}

pub fn Chart(cx: Scope) -> Element {
    let week_chart = LineChart::week_chart();
    cx.render(rsx! {
        svg { class: "@apply rounded-xl max-w-full", width: "100%", height: "600",
            defs {
                linearGradient { id: "backgroundGradient", x1: "0%", y1: "0%", x2: "0%", y2: "100%" }
                stop { offset: "0%", "stop-color": "#dddddd" }
                stop { offset: "100%", "stop-color": "#ffeeee" }
                clipPath { id: "clipPath" }
                path { id: "clipPathPath", fill: "none" }
            }

            rect { id: "my-rect", x: "0", y: "0", width: "100%", height: "100%", fill: "none" }
            week_chart.days.iter().enumerate().map(|(i, day)| {
                    let x = (i as f32 * 100.0) + 50.0;
                    let y = 500.0 - (week_chart.progress[i] * 400.0);
                    rsx! {
                        g {
                            width: "100%", height: "100%",
                            text { x: "{x}", y: "20.0", r#"text-anchor"#: "middle", "{day}" },
                            if i > 0 {
                                let week_chart = LineChart::week_chart();
                                let prev_x = ((i - 1) as f32 * 100.0) + 50.0;
                                let prev_y = 500.0 - (week_chart.progress[i - 1] * 400.0);
                                let c1_x = prev_x + (x - prev_x) / 4.0;
                                let c1_y = prev_y + (y - prev_y) / 4.0;
                                let c2_x = prev_x + 3.0 * (x - prev_x) / 4.0;
                                let c2_y = prev_y + 3.0 * (y - prev_y) / 4.0;
                                rsx! {
                                    path { d: "M {prev_x} {prev_y} C {c1_x} {c1_y} {c2_x} {c2_y} {x} {y}", fill: "none", stroke: "orange", "stroke-width": "2"},
                                }
                            }
                            circle { cx: "{x}", cy: "{y}", r: "8", fill: "orange" },
                        }
                    }
                })
        }
    })
}
