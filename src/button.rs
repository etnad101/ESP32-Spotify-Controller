use esp_idf_svc::hal::gpio::{InterruptType, PinDriver, Pull};

pub struct Button<T> 
where T: esp_idf_svc::hal::gpio::InputPin + esp_idf_svc::hal::gpio::OutputPin {
    held: bool,
    clicked: bool,
    prev: bool,
    pin: PinDriver<'static, T, esp_idf_svc::hal::gpio::Input>,
}

impl<T> Button<T> 
where T: esp_idf_svc::hal::gpio::InputPin + esp_idf_svc::hal::gpio::OutputPin {
    pub fn new(pin: T) -> Self {
        let mut pin = PinDriver::input(pin).unwrap();
        pin.set_pull(Pull::Down).unwrap();
        pin.set_interrupt_type(InterruptType::NegEdge).unwrap();

        Self {
            held: false,
            clicked: false,
            prev: false,
            pin
        }
    }

    pub fn update(&mut self) {
        self.held = self.pin.is_high();
        self.clicked = self.pin.is_high();

        if self.prev {
            self.clicked = false;
        }

        self.prev = self.held;
    }

    pub fn held(&self) -> bool {
        self.held
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}