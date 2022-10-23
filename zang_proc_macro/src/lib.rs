use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Xato" => "Err",
        "Ok" => "Ok",
        "Zanjir" => "String",
        "Lugat" => "HashMap",
        "Standard" => "Default",
        "Nuqson" => "Error",
        "Variant" => "Option",
        "Qandaydir" => "Some",
        "Hech_nima" => "None",
        "Natija" => "Result",
        "Men" => "Self",
        "men" => "self",
        "liniyachopetish" => "println",
        "sindir" => "break",
        "asinxron" => "async",
        "kut" => "await",
        "halqa" => "loop",
        "harakat" => "move",
        "sandiq" => "crate",
        "yetib_bolmas_kod" => "unreachable_code",
        "xuddi" => "as",
        "konstanta" => "const",
        "convention" => "trait",
        "xavfli" => "unsafe",
        "ichida" => "in",
        "dan" => "from",
        "dinamik" => "dyn",
        "ochish" => "unwrap",
        "standard" => "default",
        "oy" => "io",
        "tashqi" => "extern",
        "yoq" => "false",
        "funksiya" => "fn",
        "super" => "super",
        "kiritmoq" => "insert",
        "olish" => "get",
        "ruxsat" => "allow",
        "topalon" | "vahima"  => "panic",
        "modul" => "mod",
        "uzgaruvchi" => "mut",
        "yangi" => "new",
        "qayerda" => "where",
        "uchun" => "for",
        "asosiy" => "main",
        "ommaviy" => "pub",
        "hechnima" => None?,
        "qaytar" => "return",
        "amal" => "impl",
        "mos" => "match",
        "agar" => "if",
        "bomasam" => "else",
        "ozi" => "self",
        "belgi" => "let",
        "statik" => "static",
        "struktura" => "struct",
        "umid" => "expect",
        "davr" => "while",
        "foydalan" => "use",
        "vers" => "into",
        "rost" => "true",
        "royxat" => "enum",
        // TODO: Add more

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
pub fn zang(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
