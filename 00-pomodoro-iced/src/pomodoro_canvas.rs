use crate::timer;
use crate::Arrow;
use crate::ColoredKeys;
use crate::Pomodoro;

use iced::alignment;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::{stroke, Geometry, LineCap, Path, Stroke};
use iced::{Color, Point, Renderer, Size, Theme, Vector};
use std::f32::consts::TAU;

impl<Message> canvas::Program<Message> for Pomodoro {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let border_clock = Path::circle(center, radius);
            frame.fill(&border_clock, palette.background.strong.color);

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let width = radius / 100.0;

            let thin_stroke = || -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(palette.background.base.color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(palette.background.base.color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(
                    self.timer.time_now as f32 / self.timer.time_limit_seconds as f32 * TAU,
                );
                frame.stroke(&short_hand, wide_stroke());
            });

            frame.with_save(|frame| {
                frame.rotate(self.timer.time_now as f32 / 60. * TAU);
                frame.stroke(&long_hand, thin_stroke());
            });

            if !(matches!(self.timer.state, timer::State::Stopped)) {
                return;
            }

            let square_width: f32 = frame.width() * 0.9;

            let sq_point = Point::new(-square_width / 2., -frame.height() / 4.);
            let sq_size = Size {
                width: square_width,
                height: frame.height() / 2.,
            };

            let margin: f32 = 5.;

            let inner_point =
                Point::new(-square_width / 2. + margin, -frame.height() / 4. + margin);
            let inner_size = Size {
                width: square_width - margin * 2.,
                height: frame.height() / 2. - margin * 2.,
            };

            let popup = Path::rectangle(sq_point, sq_size);
            let inner_popup = Path::rectangle(inner_point, inner_size);

            frame.fill(&popup, palette.background.base.color);
            frame.fill(&inner_popup, palette.secondary.weak.color);

            let value = self.menu.get_current_value(&self.timer);
            let menu_text_value = self.menu.get_value_name();

            let menu_text = canvas::Text {
                content: String::from(menu_text_value),
                position: Point::new(0., -30.),
                horizontal_alignment: alignment::Horizontal::Center,
                ..Default::default()
            };

            let value_text = canvas::Text {
                content: String::from(format!("{}", value)),
                position: Point::new(0., 5.),
                horizontal_alignment: alignment::Horizontal::Center,
                ..Default::default()
            };

            let default_text_color = palette.background.base.color;

            fn arrow_color(colored_keys: &ColoredKeys, arrow: Arrow, def: Color) -> Color {
                match colored_keys.get_state(arrow) {
                    true => iced::Color {
                        r: 255.,
                        g: 0.,
                        b: 0.,
                        a: 1.,
                    },
                    false => def,
                }
            }

            let left_arrow = canvas::Text {
                content: String::from("<"),
                position: Point::new(-100., -30.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Left, default_text_color),
                ..Default::default()
            };

            let right_arrow = canvas::Text {
                content: String::from(">"),
                position: Point::new(100., -30.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Right, default_text_color),
                ..Default::default()
            };

            let upper_arrow = canvas::Text {
                content: String::from("^"),
                position: Point::new(0., 0.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Up, default_text_color),
                ..Default::default()
            };

            let lower_arrow = canvas::Text {
                content: String::from("^"),
                position: Point::new(0., 0.),
                horizontal_alignment: alignment::Horizontal::Center,
                color: arrow_color(&self.colored_keys, Arrow::Down, default_text_color),
                ..Default::default()
            };

            frame.fill_text(menu_text);
            frame.fill_text(value_text);
            frame.fill_text(right_arrow);
            frame.fill_text(left_arrow);

            frame.with_save(|frame| {
                frame.translate(Vector { x: 0., y: -10. });
                frame.fill_text(upper_arrow);
            });

            frame.with_save(|frame| {
                frame.rotate(0.5 * TAU);
                frame.translate(Vector { x: 0., y: -40. });
                frame.fill_text(lower_arrow);
            });
        });

        vec![clock]
    }
}
