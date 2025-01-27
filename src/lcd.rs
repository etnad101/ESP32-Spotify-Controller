use esp_idf_svc::{
    hal::{
        delay::Delay,
        gpio::{InputPin, Level, Output, OutputPin, PinDriver},
    },
    sys::EspError,
};
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

// flags for display entry mode
const LCD_ENTRYRIGHT: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// flags for display on/off control
const LCD_DISPLAYON: u8 = 0x04;
const LCD_DISPLAYOFF: u8 = 0x00;
const LCD_CURSORON: u8 = 0x02;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKON: u8 = 0x01;
const LCD_BLINKOFF: u8 = 0x00;

// flags for display/cursor shift
const LCD_DISPLAYMOVE: u8 = 0x08;
const LCD_CURSORMOVE: u8 = 0x00;
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_MOVELEFT: u8 = 0x00;

// flags for function set
const LCD_8BITMODE: u8 = 0x10;
const LCD_4BITMODE: u8 = 0x00;
const LCD_2LINE: u8 = 0x08;
const LCD_1LINE: u8 = 0x00;
const LCD_5x10DOTS: u8 = 0x04;
const LCD_5x8DOTS: u8 = 0x00;

pub struct Lcd<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
where
    T0: InputPin + OutputPin,
    T1: InputPin + OutputPin,
    T2: InputPin + OutputPin,
    T3: InputPin + OutputPin,
    T4: InputPin + OutputPin,
    T5: InputPin + OutputPin,
    T6: InputPin + OutputPin,
    T7: InputPin + OutputPin,
    T8: InputPin + OutputPin,
    T9: InputPin + OutputPin,
    T10: InputPin + OutputPin,
{
    data: u8,
    display_function: u8,
    display_control: u8,
    display_mode: u8,
    rs: PinDriver<'static, T0, Output>,
    rw: PinDriver<'static, T1, Output>,
    e: PinDriver<'static, T2, Output>,
    d0: PinDriver<'static, T3, Output>,
    d1: PinDriver<'static, T4, Output>,
    d2: PinDriver<'static, T5, Output>,
    d3: PinDriver<'static, T6, Output>,
    d4: PinDriver<'static, T7, Output>,
    d5: PinDriver<'static, T8, Output>,
    d6: PinDriver<'static, T9, Output>,
    d7: PinDriver<'static, T10, Output>,
    delay: Delay,
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Lcd<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
where
    T0: InputPin + OutputPin,
    T1: InputPin + OutputPin,
    T2: InputPin + OutputPin,
    T3: InputPin + OutputPin,
    T4: InputPin + OutputPin,
    T5: InputPin + OutputPin,
    T6: InputPin + OutputPin,
    T7: InputPin + OutputPin,
    T8: InputPin + OutputPin,
    T9: InputPin + OutputPin,
    T10: InputPin + OutputPin,
{
    pub fn new(
        p_rs: T0,
        p_rw: T1,
        p_e: T2,
        p_d0: T3,
        p_d1: T4,
        p_d2: T5,
        p_d3: T6,
        p_d4: T7,
        p_d5: T8,
        p_d6: T9,
        p_d7: T10,
        delay: Delay,
    ) -> Self {
        Self {
            data: 0,
            display_function: 0,
            display_control: 0,
            display_mode: 0,
            rs: PinDriver::output(p_rs).unwrap(),
            rw: PinDriver::output(p_rw).unwrap(),
            e: PinDriver::output(p_e).unwrap(),
            d0: PinDriver::output(p_d0).unwrap(),
            d1: PinDriver::output(p_d1).unwrap(),
            d2: PinDriver::output(p_d2).unwrap(),
            d3: PinDriver::output(p_d3).unwrap(),
            d4: PinDriver::output(p_d4).unwrap(),
            d5: PinDriver::output(p_d5).unwrap(),
            d6: PinDriver::output(p_d6).unwrap(),
            d7: PinDriver::output(p_d7).unwrap(),
            delay,
        }
    }

    pub fn start(&mut self) -> Result<(), EspError> {
        let delay = Delay::new_default();
        self.display_function = LCD_8BITMODE | LCD_1LINE | LCD_2LINE | LCD_5x8DOTS;

        self.delay.delay_us(50_000);
        self.rs.set_low()?;
        self.e.set_low()?;

        self.command(LCD_FUNCTIONSET | self.display_function)?;
        delay.delay_us(4500); // wait more than 4.1 ms

        self.command(LCD_FUNCTIONSET | self.display_function)?;
        delay.delay_us(150);

        self.command(LCD_FUNCTIONSET | self.display_function)?;

        self.display_control = LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF;

        self.display()?;

        // self.clear()?;

        // self.display_mode = LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT;

        // self.command(LCD_ENTRYMODESET | self.display_mode)?;

        Ok(())
    }

    pub fn display(&mut self) -> Result<(), EspError> {
        self.display_control |= LCD_DISPLAYON;
        self.command(LCD_DISPLAYCONTROL | self.display_control)?;
        Ok(())
    }
    pub fn clear(&mut self) -> Result<(), EspError> {
        self.command(LCD_CLEARDISPLAY)?;
        self.delay.delay_us(2000);
        Ok(())
    }

    pub fn command(&mut self, value: u8) -> Result<(), EspError> {
        self.send(value, Level::Low)?;
        Ok(())
    }

    pub fn write(&mut self, value: u8) -> Result<(), EspError> {
        self.send(value, Level::High)?;
        Ok(())
    }

    fn send(&mut self, value: u8, level: Level) -> Result<(), EspError> {
        self.rs.set_level(level)?;
        self.write8bits(value)?;

        Ok(())
    }

    fn write8bits(&mut self, value: u8) -> Result<(), EspError> {
        self.d0.set_level(self.level(value & 1))?;
        self.d1.set_level(self.level((value >> 1) & 1))?;
        self.d2.set_level(self.level((value >> 2) & 1))?;
        self.d3.set_level(self.level((value >> 3) & 1))?;
        self.d4.set_level(self.level((value >> 4) & 1))?;
        self.d5.set_level(self.level((value >> 5) & 1))?;
        self.d6.set_level(self.level((value >> 6) & 1))?;
        self.d7.set_level(self.level((value >> 7) & 1))?;

        self.pulse_enable()?;
        Ok(())
    }

    fn pulse_enable(&mut self) -> Result<(), EspError>{
        self.e.set_low()?;
        self.delay.delay_us(1);
        self.e.set_high()?;
        self.delay.delay_us(1);
        self.e.set_low()?;
        self.delay.delay_us(100);
        Ok(())
    }

    fn level(&self, value: u8) -> Level {
        if value == 0 {
            return Level::Low;
        }
        Level::High
    }
}
