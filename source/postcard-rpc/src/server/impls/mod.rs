//! Implementations of various Server traits
//!
//! The implementations in this module typically require feature flags to be set.

#[cfg(feature = "embassy-usb-0_5-server")]
/// Server implementation using `embassy-usb` 0.5.
pub mod embassy_usb_v0_5 {
    use embassy_sync_0_7 as embassy_sync;
    use embassy_usb_0_5 as embassy_usb;
    use embassy_usb_driver_0_2 as embassy_usb_driver;

    include!("embassy_usb_v0_5.rs");
}

#[cfg(feature = "embassy-usb-0_6-server")]
/// Server implementation using `embassy-usb` 0.6.
pub mod embassy_usb_v0_6 {
    use embassy_sync_0_8 as embassy_sync;
    use embassy_usb_0_6 as embassy_usb;
    use embassy_usb_driver_0_2 as embassy_usb_driver;

    include!("embassy_usb_v0_5.rs");
}

#[cfg(all(
    feature = "embedded-io-async-0_6-server",
    feature = "embassy-sync-0_7",
))]
/// Server implementation using `embedded-io-async` 0.6 with `embassy-sync` 0.7.
pub mod embedded_io_async_v0_6_sync_v0_7 {
    use embassy_sync_0_7 as embassy_sync;

    include!("embedded_io_async_v0_6.rs");
}

#[cfg(all(
    feature = "embedded-io-async-0_6-server",
    feature = "embassy-sync-0_8",
))]
/// Server implementation using `embedded-io-async` 0.6 with `embassy-sync` 0.8.
pub mod embedded_io_async_v0_6_sync_v0_8 {
    use embassy_sync_0_8 as embassy_sync;

    include!("embedded_io_async_v0_6.rs");
}

#[cfg(all(
    feature = "embedded-io-async-0_6-server",
    feature = "embassy-sync-0_8",
))]
pub use embedded_io_async_v0_6_sync_v0_8 as embedded_io_async_v0_6;

#[cfg(all(
    feature = "embedded-io-async-0_6-server",
    not(feature = "embassy-sync-0_8"),
    feature = "embassy-sync-0_7",
))]
pub use embedded_io_async_v0_6_sync_v0_7 as embedded_io_async_v0_6;

#[cfg(all(target_os = "linux", feature = "usb-gadget"))]
pub mod usb_gadget;

#[cfg(feature = "test-utils")]
pub mod test_channels;

#[cfg(any(
    feature = "embassy-usb-0_5-server",
    feature = "embassy-usb-0_6-server",
    feature = "embedded-io-async-0_6-server",
))]
pub(crate) mod embassy_shared {
    use crate::server::WireSpawn;
    use embassy_executor::{SpawnError, SpawnToken, Spawner};

    //////////////////////////////////////////////////////////////////////////////
    // SPAWN
    //////////////////////////////////////////////////////////////////////////////

    /// A [`WireSpawn`] impl using the embassy executor
    #[derive(Clone)]
    pub struct EmbassyWireSpawn {
        /// The embassy-executor spawner
        pub spawner: Spawner,
    }

    impl From<Spawner> for EmbassyWireSpawn {
        fn from(value: Spawner) -> Self {
            Self { spawner: value }
        }
    }

    impl WireSpawn for EmbassyWireSpawn {
        type Error = SpawnError;

        type Info = Spawner;

        fn info(&self) -> &Self::Info {
            &self.spawner
        }
    }

    /// Attempt to spawn the given token
    pub fn embassy_spawn<Sp, S: Sized>(sp: &Sp, tok: SpawnToken<S>) -> Result<(), Sp::Error>
    where
        Sp: WireSpawn<Error = SpawnError, Info = Spawner>,
    {
        let info = sp.info();
        info.spawn(tok)
    }
}

#[cfg(feature = "tokio")]
pub(crate) mod tokio_shared {
    use core::convert::Infallible;
    use tokio::runtime;

    use crate::server::WireSpawn;

    //////////////////////////////////////////////////////////////////////////////
    // SPAWN
    //////////////////////////////////////////////////////////////////////////////

    /// A [`WireSpawn`] impl using the embassy executor
    #[derive(Clone)]
    pub struct TokioWireSpawn {
        /// handle to the current tokio runtime
        pub rt: runtime::Handle,
    }

    impl From<runtime::Handle> for TokioWireSpawn {
        fn from(value: runtime::Handle) -> Self {
            Self { rt: value }
        }
    }

    impl WireSpawn for TokioWireSpawn {
        type Error = Infallible;

        type Info = runtime::Handle;

        fn info(&self) -> &Self::Info {
            &self.rt
        }
    }

    /// Attempt to spawn the given token
    pub fn tokio_spawn<Sp, F>(sp: &Sp, fut: F) -> Result<(), Sp::Error>
    where
        Sp: WireSpawn<Error = Infallible, Info = runtime::Handle>,
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let info = sp.info();
        info.spawn(fut); // TODO: store handle?

        Ok(())
    }
}
