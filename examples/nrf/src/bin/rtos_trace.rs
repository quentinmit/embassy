#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::task::Poll;
use futures::StreamExt;

use embassy_executor::executor::Spawner;
use embassy_executor::time::{Duration, Instant, Timer, Ticker};
use embassy_nrf::Peripherals;

// N.B. systemview_target cannot be used at the same time as defmt_rtt.

use rtos_trace;
use systemview_target::SystemView;
use panic_probe as _;
#[cfg(feature = "log")]
use log::*;

static LOGGER: systemview_target::SystemView = systemview_target::SystemView::new();
rtos_trace::global_trace!{SystemView}

struct TraceInfo();

impl rtos_trace::RtosTraceApplicationCallbacks for TraceInfo {
    fn system_description() {}
    fn sysclock() -> u32 {
        64000000
    }
}
rtos_trace::global_application_callbacks!{TraceInfo}

#[embassy_executor::task]
async fn run1() {
    loop {
        #[cfg(feature = "log")]
        info!("DING DONG");
        #[cfg(not(feature = "log"))]
        rtos_trace::trace::marker(13);
        Timer::after(Duration::from_ticks(16000)).await;
    }
}

#[embassy_executor::task]
async fn run2() {
    loop {
        Timer::at(Instant::from_ticks(0)).await;
    }
}

#[embassy_executor::task]
async fn run3() {
    futures::future::poll_fn(|cx| {
        cx.waker().wake_by_ref();
        Poll::<()>::Pending
    })
    .await;
}

#[embassy_executor::task]
async fn respawned() {
    Timer::after(Duration::from_millis(100)).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner, _p: Peripherals) {
    LOGGER.init();
    #[cfg(feature = "log")]
    {
        ::log::set_logger(&LOGGER).ok();
        ::log::set_max_level(::log::LevelFilter::Trace);
    }

    spawner.spawn(run1()).unwrap();
    spawner.spawn(run2()).unwrap();
    spawner.spawn(run3()).unwrap();
    let mut ticker = Ticker::every(Duration::from_millis(1000));
    loop {
        spawner.spawn(respawned()).unwrap();
        ticker.next().await;
    }
}
