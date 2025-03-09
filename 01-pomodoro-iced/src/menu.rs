use crate::timer::Timer;
use crate::Arrow;

pub struct Menu {
    state: MenuState,
    pub theme: u8,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            state: MenuState::TimeLimit,
            theme: 2,
        }
    }
}

#[derive(Clone)]
pub enum MenuState {
    TimeLimit,
    StageLimit,
    ShortBreakTime,
    LongBreakTime,
    LongBreakFrequency,
    Theme,
}

impl MenuState {
    fn get_rules(self: &Self) -> Rules {
        match self {
            Self::TimeLimit => Rules {
                min: 5,
                max: 60,
                change: 5,
            },
            Self::StageLimit => Rules {
                min: 1,
                max: 30,
                change: 1,
            },
            Self::ShortBreakTime => Rules {
                min: 1,
                max: 30,
                change: 1,
            },
            Self::LongBreakTime => Rules {
                min: 5,
                max: 60,
                change: 5,
            },
            Self::LongBreakFrequency => Rules {
                min: 1,
                max: 20,
                change: 1,
            },
            Self::Theme => Rules {
                min: 0,
                max: 20,
                change: 1,
            },
        }
    }
}

struct Rules {
    min: u8,
    max: u8,
    change: u8,
}

impl Menu {
    pub fn act(self: &mut Self, timer: &mut Timer, arrow: &Arrow) {
        match arrow {
            Arrow::Right | Arrow::Left => self.change_state(arrow),
            Arrow::Up | Arrow::Down => self.modify_value(timer, arrow),
        }
    }

    fn modify_value(self: &mut Self, timer: &mut Timer, arrow: &Arrow) {
        match &self.state {
            MenuState::TimeLimit => apply_mod(
                &mut timer.session.time_limit_minutes,
                self.state.get_rules(),
                arrow,
            ),
            MenuState::StageLimit => apply_mod(&mut timer.session.stage_limit, self.state.get_rules(), arrow),
            MenuState::ShortBreakTime => apply_mod(&mut timer.session.short_break_time, self.state.get_rules(), arrow),
            MenuState::LongBreakTime => apply_mod(&mut timer.session.long_break_time, self.state.get_rules(), arrow),
            MenuState::LongBreakFrequency => apply_mod(&mut timer.session.long_break_frequency, self.state.get_rules(), arrow),
            MenuState::Theme => apply_mod(&mut self.theme, self.state.get_rules(), arrow),
        }
    }

    fn change_state(self: &mut Self, arrow: &Arrow) {

        let near_states = match &self.state {
            MenuState::TimeLimit => (MenuState::TimeLimit, MenuState::StageLimit),
            MenuState::StageLimit => (MenuState::TimeLimit, MenuState::ShortBreakTime),
            MenuState::ShortBreakTime => (MenuState::StageLimit, MenuState::LongBreakTime),
            MenuState::LongBreakTime => (MenuState::ShortBreakTime, MenuState::LongBreakFrequency),
            MenuState::LongBreakFrequency => (MenuState::LongBreakTime, MenuState::Theme),
            MenuState::Theme => (MenuState::LongBreakFrequency, MenuState::Theme),
        };

        self.state = match arrow {
            Arrow::Left => near_states.0,
            Arrow::Right => near_states.1,
            _ => self.state.clone(),
        };
    }

    pub fn get_current_value(self: &Self, timer: &Timer) -> String {
        String::from(format!(
            "{}",
            match self.state {
                MenuState::TimeLimit => timer.session.time_limit_minutes,
                MenuState::Theme => self.theme as u8,
                MenuState::StageLimit => timer.session.stage_limit,
                MenuState::ShortBreakTime => timer.session.short_break_time,
                MenuState::LongBreakTime => timer.session.long_break_time,
                MenuState::LongBreakFrequency => timer.session.long_break_frequency,
            }
        ))
    }

    pub fn get_value_name(self: &Self) -> String {
        String::from(match self.state {
            MenuState::TimeLimit => "pomodoro length",
            MenuState::StageLimit => "pomodoros",
            MenuState::ShortBreakTime => "short break length",
            MenuState::LongBreakTime => "long break length",
            MenuState::LongBreakFrequency => "long break frequency",
            MenuState::Theme => "theme",
        })
    }
}

fn apply_mod(value: &mut u8, rules: Rules, arrow: &Arrow) {
    match arrow {
        Arrow::Up => {
            if *value != rules.max {
                *value += rules.change;
            }
        }
        Arrow::Down => {
            if *value != rules.min {
                *value -= rules.change
            }
        }
        _ => (),
    };
}
