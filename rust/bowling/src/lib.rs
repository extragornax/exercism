#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub enum FrameStatus {
    Open,
    Spare,
    Strike,
    Done,
}

#[derive(Debug)]
pub struct Frame {
    first_roll: u16,
    second_roll: u16,
    status: FrameStatus,
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    throws: Vec<Frame>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.throws.push(pins);

        self.second = !self.second;
        if pins == 10 {
            self.second = false;
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {}
}
