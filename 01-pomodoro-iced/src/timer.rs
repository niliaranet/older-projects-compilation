use std::time;

pub struct Timer {
    pub start_time: time::Instant,
    pub time_elapsed: time::Duration,
    pub start_pause_time: time::Instant,
    pub time_paused: time::Duration,

    pub time_limit_seconds: u16,
    pub time_now: u16,

    pub session: Session,
    pub state: State,
    stream: (rodio::OutputStream, rodio::OutputStreamHandle),
}

impl Timer {
    pub fn default() -> Timer {
        let timer_session: Session = Session::default();
        let session_time_in_seconds: u16 = (timer_session.time_limit_minutes as u16) * 60;

        let timer = Timer {
            start_time: time::Instant::now(),
            time_elapsed: time::Duration::ZERO,
            start_pause_time: time::Instant::now(),
            time_paused: time::Duration::ZERO,

            time_limit_seconds: session_time_in_seconds,
            time_now: 0,

            session: timer_session,
            state: State::Stopped,

            stream: super::sound::create_outputstream(),
        };

        return timer;
    }

    pub fn update(&mut self) {
        self.time_elapsed = self.start_time.elapsed();
        self.update_time();
        self.update_events();
    }

    fn update_time(&mut self) {
        if !(matches!(self.state, State::Running)) {
            return;
        }

        let time_as_seconds: u16 = (self.time_elapsed - self.time_paused).as_secs() as u16;
        if self.time_now != time_as_seconds {
            self.time_now = time_as_seconds;
        }
    }

    fn update_events(&mut self) {
        if self.time_now >= self.time_limit_seconds {
            self.next_stage();
        }
    }

    pub fn next_stage(&mut self) {
        match self.session.is_break_time {
            false => {
                super::sound::play_break(&self.stream);
                self.session.is_break_time = true;
                self.reset_stage();
            }
            true => {
                self.session.is_break_time = false;
                self.reset_stage();
                if self.session.stage_now == self.session.stage_limit {
                    super::sound::play_end(&self.stream);
                    self.state = State::Finished;
                } else {
                    super::sound::play_work(&self.stream);
                    self.session.stage_now += 1;
                }
            }
        }

    }

    pub fn reset_stage(&mut self) {
        self.state = State::Running;
        self.start_time = time::Instant::now();
        self.time_now = 0;
        self.time_paused = time::Duration::ZERO;
        self.time_limit_seconds = match self.session.is_break_time {
            true => self.session.get_time_limit(),
            false => self.session.time_limit_minutes,
        } as u16
            * 60;
    }

    pub fn pause_trigger(&mut self) {
        match self.state {
            State::Running => self.pause(),
            State::Paused => self.unpause(),
            _ => return,
        }
    }

    fn pause(&mut self) {
        self.state = State::Paused;
        self.start_pause_time = time::Instant::now();
    }

    fn unpause(&mut self) {
        self.state = State::Running;
        self.time_paused += self.start_pause_time.elapsed();
    }

    pub fn stop(&mut self) {
        self.state = State::Stopped;
        self.time_now = 0;
    }

    pub fn start_trigger(&mut self) {
        if matches!(self.state, State::Stopped) {
            self.restart()
        }
    }

    fn restart(&mut self) {
        self.session.is_break_time = false;
        self.session.stage_now = 1;
        self.reset_stage();
        super::sound::play_work(&self.stream);
    }
}

#[derive(PartialEq, Debug)]
pub enum State {
    Running,
    Finished,
    Paused,
    Stopped,
}

pub struct Session {
    pub short_break_time: u8,
    pub long_break_time: u8,
    pub time_limit_minutes: u8,

    pub stage_limit: u8,
    pub stage_now: u8,
    pub long_break_frequency: u8,

    pub is_break_time: bool,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            short_break_time: 5,
            long_break_time: 30,
            time_limit_minutes: 25,
            stage_limit: 4,
            stage_now: 1,
            is_break_time: false,
            long_break_frequency: 4,
        }
    }
}

impl Session {
    pub fn get_time_limit(self: &Self) -> u8 {
        match self.is_break_time {
            true => {
                let break_length: u8;
                if self.stage_now % self.long_break_frequency == 0 {
                    break_length = self.long_break_time
                } else {
                    break_length = self.short_break_time;
                }
                break_length
            }
            false => self.time_limit_minutes,
        }
    }
}
