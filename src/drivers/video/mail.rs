pub struct MailData(Mail);

impl MailData {
    pub fn set(mut self, data: u32) -> Mail {
        self.0 .0 = (self.0 .0 & 0b1111) + (data << 4);
        self.0
    }

    pub fn set_noshift(mut self, data: u32) -> Mail {
        self.0 .0 = (self.0 .0 & 0b1111) + (data & !(0b1111));
        self.0
    }

    pub fn get(self) -> u32 {
        self.0 .0 >> 4
    }
}

pub struct MailChannel(Mail);

impl MailChannel {
    pub fn set(mut self, channel: u8) -> Mail {
        self.0 .0 = (self.0 .0 & !(0b1111)) + (channel as u32 & 0b1111);
        self.0
    }

    pub fn get(self) -> u8 {
        (self.0 .0 & 0b1111) as u8
    }
}

#[derive(Debug)]
pub struct Mail(u32);

impl Mail {
    pub fn new() -> Mail {
        Mail(0)
    }

    pub fn data(self) -> MailData {
        MailData(self)
    }

    pub fn chan(self) -> MailChannel {
        MailChannel(self)
    }
}

impl Default for Mail {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Mail> for u32 {
    fn from(val: Mail) -> Self {
        val.0
    }
}

impl From<u32> for Mail {
    fn from(val: u32) -> Self {
        Mail(val)
    }
}

