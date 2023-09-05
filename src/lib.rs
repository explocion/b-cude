#![no_std]

use heapless::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct TypeId(pub u8);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    MissingSoP,
    BufferNotEnough,
    MissingEoP,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State<const CAPACITY: usize> {
    Idle,
    Started,
    Sized(u8),
    Typed(u8, TypeId, Vec<u8, CAPACITY>),
    Succeeded(TypeId, Vec<u8, CAPACITY>),
}

#[allow(non_upper_case_globals)]
pub struct Receiver<const SoP: u8, const EoP: u8, const CAPACITY: usize> {
    state: State<CAPACITY>,
}

#[allow(non_upper_case_globals)]
impl<const SoP: u8, const EoP: u8, const CAPACITY: usize> Receiver<SoP, EoP, CAPACITY> {
    #[inline]
    pub fn new() -> Self {
        Self { state: State::Idle }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.state = State::Idle;
    }

    pub fn next(&mut self, byte: u8) -> Result<(), Error> {
        match &mut self.state {
            State::Idle => {
                if byte == SoP {
                    self.state = State::Started;
                    Ok(())
                } else {
                    Err(Error::MissingSoP)
                }
            }
            State::Started => {
                if byte as usize <= CAPACITY {
                    self.state = State::Sized(byte);
                    Ok(())
                } else {
                    Err(Error::BufferNotEnough)
                }
            }
            State::Sized(expected) => {
                self.state = State::Typed(*expected, TypeId(byte), Vec::new());
                Ok(())
            }
            State::Typed(expected, id, buffer) => {
                if (*expected as usize) < CAPACITY {
                    buffer.push(byte).unwrap();
                    Ok(())
                } else if byte == EoP {
                    self.state = State::Succeeded(*id, buffer.clone());
                    Ok(())
                } else {
                    Err(Error::MissingEoP)
                }
            }
            _ => Ok(()),
        }
    }

    #[inline]
    pub fn packet(&self) -> Option<(TypeId, &[u8])> {
        match &self.state {
            State::Succeeded(id, buffer) => Some((*id, buffer)),
            _ => None,
        }
    }
}
