ruggine::ruggine! {
    esterna cassa ruggine;

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

    pubblico(cassa) funzione opzione(i: u32) -> Opzione<Risultato<u32, Catena>> {
        se i % 2 == 1 {
            se i == 42 {
                Qualche(Azz(Catena::da("merda")))
            } altrimenti {
                Qualche(Buono(33))
            }
        } altrimenti {
            Nessun
        }
    }

    asincrono funzione esempio() {
    }

    asincrono funzione esempio2() {
        esempio().aspetta;
    }

    funzione principale() {
        sia mutevole x = 31;

        combacia x {
            42 => {
                affiggi!("pasta al sugo")
            }
            _ => affiggi!("mamma mia")
        }

        per i in 0..10 {
            sia val = ciclo {
                ferma i;
            };

            fin che x < val {
                x += 1;
            }

            x = se sia Qualche(risultato) = opzione(i) {
                risultato.disimballa()
            } altrimenti {
                12
            };
        }

        //secondaire();
    }

    #[consenti(codice_irragiungibile)]
    funzione secondaria() {
        merda!("mannaggia");
        cazzo!("diamine");
        ops!("perbacco"); // in SFW contexts
    }
}
