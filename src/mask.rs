// mask.rs      Pixel operations for alpha mask pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use pix::{Alpha, Channel, Mask};
use crate::Blend;
use crate::lerp::Lerp;

impl<C, A> Blend for Mask<C, A>
    where C:Channel + Lerp, A: Alpha<Chan=C>, A: From<C>
{

    /// Blend pixels with `over` operation (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_fallback<B: Blend>(dst: &mut [Self], src: &[B], _clr: Self)
        where Self: From<B>
    {
        for (bot, top) in dst.iter_mut().zip(src) {
            let s = Into::<Self>::into(*top);
            // FIXME: use clr for src
            *bot = Blend::over(*bot, s);
        }
    }

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self {
        let t = src.alpha().value();
        let value = dst.alpha().value().lerp(t, t);
        Mask::new(value)
    }
}
