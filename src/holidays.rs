pub mod valentines {
    use std::io::Write;
    use std::io::stdout;

    static VALENTINES: &'static [u8] = include_bytes!("../data/ascii/valentines.txt");

    pub struct Valentines {
        last_activation_year: i32,
    }

    impl Valentines {
        fn new() -> Valentines {
            Valentines {
                last_activation_year: i32::min_value(),
            }
        }
    }

    pub fn new() -> Box<::Event+'static> {
        box Valentines::new()
    }

    impl ::Event for Valentines {
        fn activate(&mut self, now: &::time::Tm) {
            let should_activate =
                now.tm_mon == 1 &&
                14 == now.tm_mday &&
                self.last_activation_year < now.tm_year;

            if !should_activate {
                return;
            }

            self.last_activation_year = now.tm_year;
            match stdout().write_all(VALENTINES) {
                Ok(()) => (),
                Err(err) => println!("Some shit broke: {:?}", err),
            }
        }
    }
}

pub mod chinesenewyear {
    use std::io::Write;
    use std::io::stdout;

    static SHEEP: &'static [u8] = include_bytes!("../data/ascii/chinesenewyear/sheep.txt");
    static MONKEY: &'static [u8] = include_bytes!("../data/ascii/chinesenewyear/monkey.txt");
    static ROOSTER: &'static [u8] = include_bytes!("../data/ascii/chinesenewyear/rooster.txt");

    // Weiiiiird
    //   error: cannot refer to the interior of another static, use a constant instead
    //
    // But I'd prefer to not use a constant in this case.
    static START_DATES: ::phf::Map<i32, (i32, i32, &'static &'static [u8])> = phf_map! {
       2015_i32 => (2, 19, &SHEEP),
       2016_i32 => (2, 8, &MONKEY),
       2017_i32 => (1, 28, &ROOSTER),
    };

    pub struct NewYear {
        last_activation_year: i32,
    }

    impl NewYear {
        fn new() -> NewYear {
            NewYear {
                last_activation_year: i32::min_value(),
            }
        }
    }

    pub fn new() -> Box<::Event+'static> {
        box NewYear::new()
    }

    impl ::Event for NewYear {
        fn activate(&mut self, now: &::time::Tm) {
            if self.last_activation_year == now.tm_year {
                // Fast exit
                return;
            }

            let current_year = 1900 + now.tm_year;

            if let Some(&(month, day, art)) = START_DATES.get(&current_year) {
                // tm_mon zero-indexes the month, so add 1.
                let should_activate = month == (1 + now.tm_mon) && day == now.tm_mday;

                if !should_activate {
                    return;
                }

                self.last_activation_year = now.tm_year;
                match stdout().write_all(*art) {
                    Ok(()) => (),
                    Err(err) => println!("Some shit broke: {:?}", err),
                }
            }
        }
    }
}


pub fn register_all(target: &mut Vec<Box<::Event+'static>>) {
    target.push(valentines::new());
    target.push(chinesenewyear::new());
}
