//! Module defining or exporting peripheral types for the ['ahb'], ['apb'] and
//! ['pclk'] modules
//!
//! The `ahb`, `apb` and `pclk` modules each define structs that are
//! generic over a type parameter representing a peripheral. Some peripheral
//! modules already define suitable types for this purpose. For example,
//! [`sercom`] defines the [`Sercom0`], [`Sercom1`], etc. types. But other
//! peripherals are either not yet implemented in the HAL or do not define a
//! suitable type. This module defines a type for such peripherals. If/when a
//! suitable type is added for a given peripheral, the type defined here should
//! be deprecated or removed.
//!
//! [`ahb`]: super::ahb
//! [`apb`]: super::apb
//! [`pclk`]: super::pclk
//! [`sercom`]: crate::sercom
//! [`Sercom0`]: crate::sercom::Sercom0
//! [`Sercom1`]: crate::sercom::Sercom1

use crate::typelevel::Sealed;

#[cfg(feature = "min-samd51g")]
pub use crate::sercom::{Sercom0, Sercom1, Sercom2, Sercom3, Sercom4, Sercom5};
#[cfg(feature = "min-samd51n")]
pub use crate::sercom::{Sercom6, Sercom7};

macro_rules! create_types {
    (
        $(
            $Type:ident
        ),+
    ) => {
        $(
            /// Marker type representing the corresponding peripheral
            ///
            /// This type is defined by and used within the [`clock`](super)
            /// module. See the the [`types`](self) module documentation for
            /// more details.
            pub enum $Type {}
            impl Sealed for $Type {}
        )+
    };
}

create_types!(Ac);
create_types!(Adc0, Adc1);
create_types!(Aes);
#[cfg(any(feature = "same51", feature = "same53", feature = "same54"))]
create_types!(Can0, Can1);
create_types!(Ccl);
create_types!(Cmcc);
create_types!(CM4Trace);
create_types!(Dac);
create_types!(Dmac);
create_types!(Dsu);
create_types!(Eic);
create_types!(
    EvSys, EvSys0, EvSys1, EvSys2, EvSys3, EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, EvSys10,
    EvSys11
);
create_types!(FreqM);
create_types!(FreqMMeasure);
create_types!(FreqMReference);
create_types!(Gclk);
#[cfg(any(feature = "same53", feature = "same54"))]
create_types!(Gmac);
create_types!(Hpb0, Hpb1, Hpb2, Hpb3);
create_types!(Icm);
create_types!(Mclk);
create_types!(NvmCtrl, NvmCtrlSmeeProm, NvmCtrlCache);
#[cfg(feature = "min-samd51j")]
create_types!(I2S, I2S0, I2S1);
create_types!(OscCtrl);
create_types!(Osc32kCtrl);
create_types!(Pac);
create_types!(Pcc);
create_types!(PDec);
create_types!(Pm);
create_types!(Port);
create_types!(Pukcc);
create_types!(Qspi, Qspi2x);
create_types!(RamEcc);
create_types!(RstC);
create_types!(Rtc);
create_types!(Sdhc0);
#[cfg(feature = "min-samd51n")]
create_types!(Sdhc1);
create_types!(SlowClk);
create_types!(SupC);
create_types!(Tc0Tc1, Tc0, Tc1);
create_types!(Tc2Tc3, Tc2, Tc3);
#[cfg(feature = "min-samd51j")]
create_types!(Tc4Tc5, Tc4, Tc5);
#[cfg(feature = "min-samd51n")]
create_types!(Tc6Tc7, Tc6, Tc7);
create_types!(Tcc0Tcc1, Tcc0, Tcc1);
create_types!(Tcc2Tcc3, Tcc2);
#[cfg(feature = "min-samd51j")]
create_types!(Tcc3, Tcc4);
create_types!(Trng);
create_types!(Usb);
create_types!(Wdt);
