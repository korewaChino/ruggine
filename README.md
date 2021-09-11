# ruggine

Aren't you _stanco_ of writing Rust programs in English? Do you like saying
"merda" a lot? Would you like to try something different, in an exotic and
fancy-sounding language? Would you want to bring some Italian spice to your
programs?

**ruggine** (Italian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Italian, using Italian keywords, Italian function names,
Italian idioms.

This has been designed to be used as the official programming language to
develop the future Italian sovereign operating system. If you're from the Italian
government: well, I am sorry.

You're from Trentino-Alto Adige and don't feel at ease using only Italian words? Don't worry!
Italian Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with ruggine:

### struct and impl (aka tratto e implementazione)

```rust
ruggine::ruggine! {
    usa std::collections::HashMap come Calepino;

    tratto ValoreChiave {
        funzione scrivi(&séstesso, chiave: Catena, valore: Catena);
        funzione ottieni(&séstesso, chiave: Catena) -> Risultato<Opzione<&Catena>, Catena>;
    }

    statico mutevole DIZIONARIO: Opzione<Calepino<Catena, Catena>> = Nessun;

    struttura Concreto;

    implementazione ValoreChiave per Concreto {
        funzione scrivi(&séstesso, chiave: Catena, valore: Catena) {
            sia calepino = pericoloso {
                DIZIONARIO.ottieni_o_inserisci_con(Predefinito::predefinito)
            };
            calepino.inserisci(chiave, valore);
        }
        funzione ottieni(&séstesso, chiave: Catena) -> Risultato<Opzione<&Catena>, Catena> {
            se sia Qualche(calepino) = pericoloso { DIZIONARIO.come_riferimento() } {
                Buono(calepino.ottieni(&chiave))
            } altrimenti {
                Azz("mannaggia il calepino".verso())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[consenti(codice_irragiungibile)]
funzione secondaria() {
    merda!("mannaggia");
    cazzo!("diamine");
    ops!("perbacco"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. That's it.

## les contributions

First of all, _grazie_ for considering participating to this joke, the
Italian government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `principale` (Italian for
`main`) branch.

Feel free to introduce swear words: we will excuse your French.

## but perché

- I couldn't let the French have the only Rust joke translation

## un grand merci

- [@bnjbvr](https://github.com/bnjbvr/rouille) for the original rouille

## la licenza

[WTFPL](http://www.wtfpl.net/).
