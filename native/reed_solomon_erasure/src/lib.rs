#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate reed_solomon;

use rustler::{Env, Term, NifResult, Encoder};
use rustler::types::Binary;
use reed_solomon::Encoder as SolomonEncoder;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

rustler_export_nifs!(
    "Elixir.ReedSolomonErasure.Native",
    [("encode", 2, encode)],
    None
    );

fn encode<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    // Create encoder and decoder with
    let k: usize = try!(args[0].decode());
    let msg: Binary = try!(args[1].decode());
    let enc = SolomonEncoder::new(k);
    let encoded = enc.encode(&msg[..]);
    Ok((atoms::ok(), &encoded[..]).encode(env))
}
