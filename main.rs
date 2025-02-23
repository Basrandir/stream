use std::convert::Infallible;
use river_layout_toolkit::{run, Layout, GeneratedLayout, Rectangle};

/// A simple layout generator.
struct CustomLayout;

impl Layout for CustomLayout {
    type Error = Infallible;

    const NAMESPACE: &'static str = "stream";

    // ignored for now
    fn user_cmd(
        &mut self,
        _cmd: String,
        _tags: Option<u32>,
        _output: &str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    // Generate a layout whenever the compositor requests one.
    fn generate_layout(
        &mut self,
        view_count: u32,
        usable_width: u32,
        usable_height: u32,
        _tags: u32,
        output: &str,
    ) -> Result<GeneratedLayout, Self::Error> {
        // Choose main area dimensions based on output resolution.
        let (main_width, main_height) = if usable_width >= 3840 && usable_height >= 2160 {
            (1920, 1080)
        } else if usable_width == 1920 && usable_height == 1080 {
            (1280, 720)
        } else {
            (usable_width, usable_height)
        };

        let main_x = (usable_width as i32 - main_width as i32) / 2;
        let main_y = (usable_height as i32 - main_height as i32) / 2;

        let mut views = Vec::new();

        if view_count > 0 {
            views.push(Rectangle {
                x: main_x,
                y: main_y,
                width: main_width,
                height: main_height,
            });
        }

        if view_count > 1 {
            let stack_width = if main_x > 0 { main_x as u32 } else { 0 };
            let stack_count = view_count - 1;
            let each_height = usable_height / stack_count;
            for i in 0..stack_count {
                views.push(Rectangle {
                    x: 0,
                    y: (i * each_height) as i32,
                    width: stack_width,
                    height: each_height,
                });
            }
            let additional = view_count - 1;
            let left_count = additional / 2;
            let right_count = additional - left_count;

            let left_area_width = if main_x > 0 { main_x as u32 } else { 0 };
            let right_area_width = usable_width.saturating_sub((main_x + main_width as i32) as u32);

            views.extend(place_left(left_count, left_area_width, usable_height));
            views.extend(place_right(
                right_count,
                right_area_width,
                usable_height,
                main_x,
                main_width,
            ));
        }

        Ok(GeneratedLayout {
            layout_name: format!("CustomLayout for output {}", output),
            views,
        })
    }
}

fn place_left(left_count: u32, left_area_width: u32, usable_height: u32) -> Vec<Rectangle> {
    let mut placements = Vec::new();

    if left_count > 0 && left_area_width > 0 {
        let each_height = usable_height / left_count;
        for i in 0..left_count {
            placements.push(Rectangle {
                x: 0,
                y: (i * each_height) as i32,
                width: left_area_width,
                height: each_height,
            });
        }
    }

    placements
}

fn place_right(
    right_count: u32,
    right_area_width: u32,
    usable_height: u32,
    main_x: i32,
    main_width: u32,
) -> Vec<Rectangle> {
    let mut placements = Vec::new();

    if right_count > 0 && right_area_width > 0 {
        let each_height = usable_height / right_count;
        for i in 0..right_count {
            placements.push(Rectangle {
                x: (main_x + main_width as i32) as i32,
                y: (i * each_height) as i32,
                width: right_area_width,
                height: each_height,
            })
        }
    }

    placements
}

fn main() -> Result<(), river_layout_toolkit::Error<Infallible>> {
    // This will block and dispatch events with River.
    run(CustomLayout)
}
