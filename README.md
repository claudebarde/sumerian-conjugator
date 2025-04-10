# SUMERIAN CONJUGATOR

> ⚠️ Please note that this is a work in progress ⚠️

This tool helps you conjugate verbs in Sumerian by only providing the required elements and the verbal stem. The code takes care of putting the various prefixes and suffixes together while following the phonological rules of the Sumerian language described in **A Descriptive Grammar of Sumerian** by _Abraham Hendrik Jagersma_.

## How to conjugate a verb?

Starting from the stem, you can chain different methods available on the `FiniteVerbForm` struct to construct the final verb.
For example:

```rust
let stem = "zu".to_string();
let verb =
    FiniteVerbalForm::from_stem(stem.clone())
        .is_perfective()
        .is_transitive()
        .set_subject(Some(Person::ThirdSingHuman))
        .set_object(Some(Person::ThirdSingNonHuman))?
        .set_ventive(Some(Ventive))
        .print();
// => produces "munzu"
```

```rust
let stem = "kar".to_string();
let verb = FiniteVerbalForm::from_stem(stem.clone())
    .is_perfective()
    .is_transitive()
    .set_subject(Person::ThirdSingHuman)
    .set_object(Person::ThirdSingNonHuman)?
    .set_comitative(Some(Person::FirstSing))
    .set_ventive()
    .print();
// => produces "bandankar"
```

Finally, the `print` method puts all the elements together and output a string with the final verb.

> WARNING: for now, you must set the perfective and transitive properties of the verb before the subject and object to get the right order of prefixes and suffixes.

## Tests

Every verb form appearing in the tests is found in the Sumerian grammar mentioned earlier.  
Each is tested against the code to check that it creates reliable results.
You can run the test with `cargo test`.

## Goal

The library is meant to be compiled to WASM and used in a web interface (to be announced).  
You can compile it by running this command (after installing `wasm-pack`):

```bash
wasm-pack build --target web
```
