use std::io;

pub trait Open {
    fn open() -> Self;
}

pub trait Sink {
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self) -> io::Result<()>;
    fn write(&mut self, data: &[i16]) -> io::Result<()>;
}

/*
 * Allow #[cfg] rules around elements of a list.
 * Workaround until stmt_expr_attributes is stable.
 *
 * This generates 2^n declarations of the list, with every combination possible
 */
macro_rules! declare_backends {
    (pub const $name:ident : $ty:ty = & [ $($tt:tt)* ];) => (
        _declare_backends!($name ; $ty ; []; []; []; $($tt)*);
    );
}

macro_rules! _declare_backends {
    ($name:ident ; $ty:ty ; [ $($yes:meta,)* ] ; [ $($no:meta,)* ] ; [ $($exprs:expr,)* ] ; #[cfg($m:meta)] $e:expr, $($rest:tt)* ) => (
        _declare_backends!($name ; $ty ; [ $m, $($yes,)* ] ; [ $($no,)* ] ; [ $($exprs,)* $e, ] ; $($rest)*);
        _declare_backends!($name ; $ty ; [ $($yes,)* ] ; [ $m, $($no,)* ] ; [ $($exprs,)* ] ; $($rest)*);
    );

    ($name:ident ; $ty:ty ; [ $($yes:meta,)* ] ; [ $($no:meta,)* ] ; [ $($exprs:expr,)* ] ; $e:expr, $($rest:tt)*) => (
        _declare_backends!($name ; $ty ; [ $($yes,)* ] ; [ $($no,)* ] ; [ $($exprs,)* $e, ] ; $($rest)*);
    );

    ($name:ident ; $ty:ty ; [ $($yes:meta,)* ] ; [ $($no:meta,)* ] ; [ $($exprs:expr,)* ] ; #[cfg($m:meta)] $e:expr) => (
        _declare_backends!($name ; $ty ; [ $m, $($yes,)* ] ; [ $($no,)* ] ; [ $($exprs,)* $e, ] ; );
        _declare_backends!($name ; $ty ; [ $($yes,)* ] ; [ $m, $($no,)* ] ; [ $($exprs,)* ] ; );
    );

    ($name:ident ; $ty:ty ; [ $($yes:meta,)* ] ; [ $($no:meta,)* ] ; [ $($exprs:expr,)* ] ; $e:expr ) => (
        _declare_backends!($name ; $ty ; [ $($yes,)* ] ; [ $($no,)* ] ; [ $($exprs,)* $e, ] ; );
    );

    ($name:ident ; $ty:ty ; [ $($yes:meta,)* ] ; [ $($no:meta,)* ] ; [ $($exprs:expr,)* ] ; ) => (
        #[cfg(all($($yes,)* not(any($($no),*))))]
        pub const $name : $ty = &[
            $($exprs,)*
        ];
    )
}

#[allow(dead_code)]
fn mk_sink<S: Sink + Open + 'static>() -> Box<Sink> {
    Box::new(S::open())
}

#[cfg(target_os = "linux")]
mod alsa;
#[cfg(target_os = "linux")]
use self::alsa::AlsaSink;

#[cfg(all(not(target_os = "linux"), feature = "portaudio-backend"))]
mod portaudio;
#[cfg(all(not(target_os = "linux"), feature = "portaudio-backend"))]
use self::portaudio::PortAudioSink;

#[cfg(all(not(target_os = "linux"), feature = "pulseaudio-backend"))]
mod pulseaudio;
#[cfg(all(not(target_os = "linux"), feature = "pulseaudio-backend"))]
use self::pulseaudio::PulseAudioSink;


declare_backends! {
    pub const BACKENDS : &'static [(&'static str, &'static (Fn() -> Box<Sink> + Sync + Send + 'static))] = &[
        #[cfg(target_os = "linux")]
        ("alsa", &mk_sink::<AlsaSink>),
        #[cfg(all(not(target_os = "linux"), feature = "portaudio"))]
        ("portaudio", &mk_sink::<PortAudioSink>),
        #[cfg(all(not(target_os = "linux"), feature = "libpulse-sys"))]
        ("pulseaudio", &mk_sink::<PulseAudioSink>),

    ];
}
