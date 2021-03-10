//! Embedded HAL Compat
//! A compatibility layer to aleviate (some) of the issues resolving from changes to embedded-hal
// Copyright 2021 Ryan Kurte

#![no_std]

pub use eh0_2;
pub use eh1_0;

/// Compatibility container object.
/// This is generic over different E-H types and will provide adaption
/// depending on the bound type.
pub struct Compat<T> {
    inner: T,
}

/// Convert a type into a compatiblity wrapper object
pub trait IntoCompat<T> {
    fn compat(self) -> Compat<T>;
}

impl<T> IntoCompat<T> for T {
    /// Create an e-h-c wrapper around and e-h object
    /// Available methods depend on the wrapped type
    fn compat(self) -> Compat<T> {
        Compat::new(self)
    }
}

impl<T> Compat<T> {
    /// Create a new compatibility wrapper object
    pub fn new(inner: T) -> Compat<T> {
        Compat { inner }
    }

    /// Fetch a reference to the wrapped object
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Fetch a mutable reference to the wrapped object
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Destroy the compatibility wrapper, returning the wrapped object
    pub fn unwrap(self) -> T {
        self.inner
    }
}

// note that implementations over Compat cannot be generic over word type
// etc. due to orphan rules (ie. what happens if someone else defines a word type?)

// Digital / GPIOs
mod digital {
    use super::Compat;

    impl<T, E> eh1_0::digital::InputPin for Compat<T>
    where
        T: eh0_2::digital::v2::InputPin<Error = E>,
    {
        type Error = E;

        /// Is the input pin high?
        fn try_is_high(&self) -> Result<bool, Self::Error> {
            self.inner.is_high()
        }

        /// Is the input pin low?
        fn try_is_low(&self) -> Result<bool, Self::Error> {
            self.inner.is_low()
        }
    }

    impl<T, E> eh1_0::digital::OutputPin for Compat<T>
    where
        T: eh0_2::digital::v2::OutputPin<Error = E>,
    {
        type Error = E;

        /// Set the output as high
        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            self.inner.set_high()
        }

        /// Set the output as low
        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            self.inner.set_low()
        }
    }
}

/// Delays (blocking)
mod delay {
    use super::Compat;
    use core::convert::Infallible;

    impl<T> eh1_0::blocking::delay::DelayMs<u32> for Compat<T>
    where
        T: eh0_2::blocking::delay::DelayMs<u32>,
    {
        type Error = Infallible;

        fn try_delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
            self.inner.delay_ms(ms);
            Ok(())
        }
    }

    impl<T> eh1_0::blocking::delay::DelayMs<u16> for Compat<T>
    where
        T: eh0_2::blocking::delay::DelayMs<u16>,
    {
        type Error = Infallible;

        fn try_delay_ms(&mut self, ms: u16) -> Result<(), Self::Error> {
            self.inner.delay_ms(ms);
            Ok(())
        }
    }

    impl<T> eh1_0::blocking::delay::DelayUs<u32> for Compat<T>
    where
        T: eh0_2::blocking::delay::DelayUs<u32>,
    {
        type Error = Infallible;

        fn try_delay_us(&mut self, us: u32) -> Result<(), Self::Error> {
            self.inner.delay_us(us);
            Ok(())
        }
    }

    impl<T> eh1_0::blocking::delay::DelayUs<u16> for Compat<T>
    where
        T: eh0_2::blocking::delay::DelayUs<u16>,
    {
        type Error = Infallible;

        fn try_delay_us(&mut self, us: u16) -> Result<(), Self::Error> {
            self.inner.delay_us(us);
            Ok(())
        }
    }
}

/// SPI (blocking)
mod spi {
    use super::Compat;

    impl<T, E> eh1_0::blocking::spi::Write<u8> for Compat<T>
    where
        T: eh0_2::blocking::spi::Write<u8, Error = E>,
    {
        type Error = E;

        fn try_write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(words)
        }
    }

    impl<T, E> eh1_0::blocking::spi::Transfer<u8> for Compat<T>
    where
        T: eh0_2::blocking::spi::Transfer<u8, Error = E>,
    {
        type Error = E;

        fn try_transfer<'a>(&mut self, words: &'a mut [u8]) -> Result<&'a [u8], Self::Error> {
            self.inner.transfer(words)
        }
    }

    impl<T, E> eh1_0::blocking::spi::WriteIter<u8> for Compat<T>
    where
        T: eh0_2::blocking::spi::WriteIter<u8, Error = E>,
    {
        type Error = E;

        fn try_write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            self.inner.write_iter(words)
        }
    }
    // uses blanket impl
    // requires underlying device to manage cs
    impl<T, E> eh1_0::blocking::spi::transactional::Default<u8> for Compat<T>
    where
        T: eh0_2::blocking::spi::Transfer<u8, Error = E>,
        T: eh0_2::blocking::spi::Write<u8, Error = E>,
    {
    }
}

// I2C (blocking)
mod i2c {
    use super::Compat;
    use eh1_0::blocking::i2c::SevenBitAddress;

    impl<T, E> eh1_0::blocking::i2c::Read<SevenBitAddress> for Compat<T>
    where
        T: eh0_2::blocking::i2c::Read<Error = E>,
    {
        type Error = E;

        fn try_read(
            &mut self,
            address: SevenBitAddress,
            words: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.inner.read(address, words)
        }
    }

    impl<T, E> eh1_0::blocking::i2c::Write<SevenBitAddress> for Compat<T>
    where
        T: eh0_2::blocking::i2c::Write<Error = E>,
    {
        type Error = E;

        fn try_write(&mut self, address: SevenBitAddress, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.write(address, words)
        }
    }

    impl<T, E> eh1_0::blocking::i2c::WriteIter<SevenBitAddress> for Compat<T>
    where
        T: eh0_2::blocking::i2c::WriteIter<Error = E>,
    {
        type Error = E;

        fn try_write_iter<B>(
            &mut self,
            address: SevenBitAddress,
            words: B,
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.inner.write(address, words)
        }
    }

    impl<T, E> eh1_0::blocking::i2c::WriteRead<SevenBitAddress> for Compat<T>
    where
        T: eh0_2::blocking::i2c::WriteRead<Error = E>,
    {
        type Error = E;

        fn try_write_read(
            &mut self,
            address: SevenBitAddress,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.inner.write_read(address, bytes, buffer)
        }
    }

    impl<T, E> eh1_0::blocking::i2c::WriteIterRead<SevenBitAddress> for Compat<T>
    where
        T: eh0_2::blocking::i2c::WriteIterRead<Error = E>,
    {
        type Error = E;

        fn try_write_iter_read<B>(
            &mut self,
            address: SevenBitAddress,
            bytes: B,
            buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            self.inner.write_iter_read(address, bytes, buffer)
        }
    }
}

/// Serial (UART etc.)
mod serial {
    use super::Compat;

    impl<T, E> eh1_0::blocking::serial::Write<u8> for Compat<T>
    where
        T: eh0_2::blocking::serial::Write<u8, Error = E>,
    {
        type Error = E;

        fn try_bwrite_all(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            self.inner.bwrite_all(words)
        }

        fn try_bflush(&mut self) -> Result<(), Self::Error> {
            self.inner.bflush()
        }
    }
}
