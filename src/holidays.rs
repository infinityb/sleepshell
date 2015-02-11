use std::num::Int;
use std::old_io::stdio::stdout;

static VALENTINES: &'static [u8] = include_bytes!("../ascii/valentines.txt");

pub struct Valentines {
    last_activation_year: i32,
}

impl Valentines {
    pub fn new() -> Valentines {
        Valentines {
            last_activation_year: Int::min_value(),
        }
    }
}

impl ::Event for Valentines {
    fn activate(&mut self, now: &::time::Tm) {
        let should_activate = now.tm_mon == 1 &&
            13 <= now.tm_mday && now.tm_mday <= 15 &&
            self.last_activation_year < now.tm_year;

        if !should_activate {
            return;
        }

        self.last_activation_year = now.tm_year;
        match stdout().into_inner().write_all(VALENTINES) {
            Ok(()) => (),
            Err(err) => println!("Some shit broke: {:?}", err),
        }
    }
}
