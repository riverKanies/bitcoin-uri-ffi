# bitcoin-uri-ffi overview

## Motivation + Plan

Although payjoin-ffi depends on bitcoin_uri, it doesn’t export generic bitcoin_uri functions; it’s payjoin specific. We want to generalize it, then have payjoin-ffi depend on and re-export bitcoin-ffi.
We want to get wallet developers using bitcoin-uri(-ffi) early and then it's easy to add payjoin later.

so:

We want to make a bindings library suite to parse and serialize bitcoin URIs based on bitcoin_uri:

See https://github.com/payjoin/bitcoin_uri for the core crate.

We'll start with BIP 21 base Params (amount, label, message, etc.) and extend to frequently used
extensions (pj, lightning, sp, etc).

BIP 321 would be nice to support but it's not widely used yet, so the core bitcoin_uri crate
would need to be extended to support that.

## bitcoin-uri-ffi

Our base ffi bindings will follow `payjoin-ffi` bindings style. We'll have a basic feature set
that wraps the core types so that they can be bound to by multiple languages. Those core ffi types (Uri)
will be wrapped by uniffi types enabling bindings with decorators once we get flutter support, since
production apps are using flutter but not uniffi yet.

See: https://github.com/LtbLightning/payjoin-ffi/

## bitcoin-uri-flutter

bitcoin-uri-flutter should depend bitcoin-uri-ffi and provide a Flutter interface to the Uri type.

See: https://github.com/LtbLightning/payjoin-uri-flutter/

## Example usage

See [bullbitcoin-mobile](https://github.com/SatoshiPortal/bullbitcoin-mobile) using payjoin-flutter
and the dart Uri parser that requires hacks to work with BIP 21 uris.

See the [WIP Cake wallet integration](https://github.com/cake-tech/cake_wallet/pull/1949) for another example using payjoin-flutter


## First steps

1. Build payjoin-ffi and payjoin-flutter bindings.

### Payjoin-ffi

This one is super straightforward since only a rust library needs to be compiled. We're not actually using uniffi bindings yet. This exists just to provide wrappers payjoin-flutter can depend on that can
be shared by Uniffi and wasm bindings in the future.

```
cargo build
```

### Payjoin-flutter

Mobile environments are a pain. I'm using android studio emulators for most all testing. I run the payjoin-flutter/example project in two emulators, one for sender and one for receiver.

```
cd payjoin-flutter/example
flutter run
```

Payjoin flutter is a bit trickier.

the `makefile` contains the actual generation code. [payjoin-flutter/contrib/clean-and-gen-bindings.sh](https://github.com/LtbLightning/payjoin-flutter/blob/main/contrib/clean-and-gen-bindings.sh) is a script to clean up gen'd files and re-make them once you make rust changes.

After generating bindings, we wrap those bindings with nice ergonomics in payjoin-flutter/lib.



