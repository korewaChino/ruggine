use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Azz" => "Err",
        "Buono" => "Ok",
        "Catena" => "String",
        "Dizionario" => "HashMap",
        "Predefinito" => "Default",
        "Errore" => "Error",
        "Opzione" => "Option",
        "Qualche" => "Some",
        "Nessun" => "None",
        "Risultato" => "Result",
        "SéStesso" => "Self",
        "affiggi" => "println",
        "ferma" => "break",
        "asincrono" => "async",
        "aspetta" => "await",
        "ciclo" => "loop",
        "muovi" => "move",
        "cassa" => "crate",
        "codice_irragiungibile" => "unreachable_code",
        "come" => "as",
        "costante" => "const",
        "tratto" => "trait",
        "pericoloso" => "unsafe",
        "da" => "from",
        "dinamico" => "dyn",
        "disimballa" => "unwrap",
        "predefinito" => "default",
        "come_riferimento" => "as_ref",
        "esterna" => "extern",
        "falso" => "false",
        "funzione" => "fn",
        "superiore" => "super",
        "inserisci" => "insert",
        "ottieni" => "get",
        "consenti" => "allow",
        "merda" | "cazzo" | "ops" => "panic",
        "modulo" => "mod",
        "mutevole" => "mut",
        "nuovo" => "new",
        "dove" => "where",
        "per" => "for",
        "ottieni_o_inserisci_con" => "get_or_insert_with",
        "principale" => "main",
        "pubblico" => "pub",
        "che" => None?,
        "ritorna" => "return",
        "implementazione" => "impl",
        "riferimento" => "ref",
        "combacia" => "match",
        "se" => "if",
        "altrimenti" => "else",
        "séstesso" => "self",
        "sia" => "let",
        "statico" => "static",
        "struttura" => "struct",
        "esigi" => "expect",
        "fin" => "while",
        "usa" => "use",
        "verso" => "into",
        "vero" => "true",
        "enumerazione" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ruggine(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
