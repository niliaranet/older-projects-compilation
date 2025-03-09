pub mod action_handler;
pub mod colored_keys;
pub mod menu;
pub mod pomodoro_canvas;
pub mod timer;
pub mod sound;

use colored_keys::ColoredKeys;
use menu::Menu;

use iced::keyboard;
use iced::time;
use iced::widget::canvas::Cache;
use iced::widget::{canvas, column, container, row, text};
use iced::{executor, Application, Command, Element, Length, Settings, Subscription, Theme};

pub fn main() -> iced::Result {
    Pomodoro::run(Settings::default())
}

struct Pomodoro {
    timer: timer::Timer,
    clock: Cache,
    menu: Menu,
    colored_keys: ColoredKeys,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Pause,
    Stop,
    Start,
    ArrowPress(Arrow),
    ArrowRelease(Arrow),
    ResetStage,
    NextStage,
}

#[derive(Debug, Clone)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

impl Application for Pomodoro {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Pomodoro, Command<Self::Message>) {
        (
            Pomodoro {
                timer: crate::timer::Timer::default(),
                clock: Cache::default(),
                menu: Menu::default(),
                colored_keys: ColoredKeys::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("pomodoro")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Tick => self.timer.update(),
            Message::Pause => self.timer.pause_trigger(),
            Message::Stop => self.timer.stop(),
            Message::Start => self.timer.start_trigger(),
            Message::ArrowPress(arrow) => {
                self.colored_keys.press(&arrow);
                self.menu.act(&mut self.timer, &arrow);
            }
            Message::ArrowRelease(arrow) => self.colored_keys.release(arrow),
            Message::ResetStage => self.timer.reset_stage(),
            Message::NextStage => self.timer.next_stage(),
        }

        self.clock.clear();
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let time_text = text(format!(
            "{:02}:{:02} / {:02}:{:02}",
            self.timer.time_now / 60,
            self.timer.time_now % 60,
            self.timer.session.get_time_limit(),
            0,
        ));

        let session_text = text(format!(
            "[{}/{}]  ",
            self.timer.session.stage_now, self.timer.session.stage_limit,
        ));

        let session_state = text(format!(
            "{}",
            match self.timer.session.is_break_time {
                true => "  break",
                false => "working",
            },
        ));

        let session_info = row![session_text, session_state].spacing(200);

        let state = text(match self.timer.state {
            timer::State::Running => "running",
            timer::State::Paused => "paused",
            timer::State::Stopped => "stopped",
            timer::State::Finished => "finished",
        });

        let numbers_n_shit = column![state, time_text, session_info]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .height(75);

        let canvas = canvas(self as &Self).width(Length::Fill).height(200);

        let content = column![canvas, numbers_n_shit];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match &self.timer.state {
            timer::State::Stopped => {
                return Subscription::batch(vec![
                    keyboard::on_key_press(action_handler::handle_menu_keys),
                    keyboard::on_key_release(action_handler::handle_arrow_key_release),
                ]);
            }
            _ => {
                let tick = time::every(time::Duration::from_millis(100)).map(|_| Message::Tick);
                return Subscription::batch(vec![
                    tick,
                    keyboard::on_key_press(action_handler::handle_running_keys),
                ]);
            }
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::ALL[self.menu.theme.clone() as usize].clone()
    }
}
