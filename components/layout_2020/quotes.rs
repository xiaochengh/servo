/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// Quotes data is obtained from ICU CLDR data file /tmp/cldr-common-46.0.zip.

// TODO(xiaochengh): This file should better be moved to elsewhere and maintained automatically.
// Or even better, extend the icu create to provide the data directly.

use std::collections::HashMap;
use std::sync::OnceLock;

use icu_locid::Locale;

#[derive(Clone, Copy, Debug)]
struct QuotesData(char, char, char, char);

#[derive(Clone, Copy, Debug)]
pub struct QuotePair {
    pub opening: char,
    pub closing: char,
}

type QuotesMap = HashMap<&'static str, QuotesData>;

static QUOTES_MAP: OnceLock<QuotesMap> = OnceLock::new();

static DEFAULT_QUOTES: QuotesData = QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}');

fn create_quotes_map() -> QuotesMap {
    let input = [
        (
            "aa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ab",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "af",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ak",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "an",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ann",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "apc",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "arn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "as",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "asa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "az",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ba",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bal",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bem",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bew",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bez",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bgc",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bgn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bho",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "blt",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "brx",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "bss",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "byn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cad",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cch",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ccp",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ce",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ceb",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cgg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cho",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "chr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cic",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ckb",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "co",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "csw",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "cy",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "da",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "dav",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "dje",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "doi",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "dv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "dz",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ebu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ee",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "en",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "eo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "es",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "fil",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "fo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "frr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "fur",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "fy",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ga",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gaa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gd",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gez",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gl",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "guz",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "gv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ha",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "haw",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "hi",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "hnj",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "id",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ig",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ii",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "io",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "iu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "jbo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "jmc",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "jv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kaa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kaj",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kam",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kcg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kde",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kea",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ken",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kgp",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "khq",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ki",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kl",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kln",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "km",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ko",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kok",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kpe",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ks",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ksb",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ksh",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ku",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kw",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "kxv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "la",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lkt",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lmo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ln",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lrc",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ltg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "luo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "lv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mai",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mas",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mdf",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mer",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mfe",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mgh",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mgo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mhn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mi",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mic",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ml",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mni",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "moh",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ms",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mt",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "mus",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "my",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "myv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "naq",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nb",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nd",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nds",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ne",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nqo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nus",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ny",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nyn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "oc",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "om",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "or",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "os",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "osa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "pa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "pap",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "pcm",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "pis",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "prg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ps",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "pt",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "qu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "quc",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "raj",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "rhg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "rif",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "rm",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "rof",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "rwk",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "saq",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sat",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sbp",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "scn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sd",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "se",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "seh",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ses",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "shn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "si",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sid",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "skr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sma",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "smj",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "smn",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sms",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "so",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ss",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ssy",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "su",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "sw",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "szl",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ta",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "te",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "teo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tg",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "th",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tig",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "to",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tok",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tpi",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "trv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "trw",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ts",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tt",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "twq",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tyv",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "tzm",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ug",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "vai",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ve",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "vec",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "vi",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "vmw",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "vo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "vun",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "wa",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "wae",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "wal",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "wbp",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "wo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "xh",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "xnr",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "xog",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "yi",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "yo",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "yrl",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "za",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "zh",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "zu",
            QuotesData('\u{201c}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "agq",
            QuotesData('\u{201e}', '\u{201d}', '\u{201a}', '\u{2019}'),
        ),
        (
            "ff",
            QuotesData('\u{201e}', '\u{201d}', '\u{201a}', '\u{2019}'),
        ),
        ("am", QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}')),
        (
            "az-Arab",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "az-Cyrl",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        ("fa", QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}')),
        (
            "fr-CH",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "gsw",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "jgo",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "kkj",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "mzn",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "sdh",
            QuotesData('\u{ab}', '\u{bb}', '\u{2039}', '\u{203a}'),
        ),
        (
            "ar",
            QuotesData('\u{201d}', '\u{201c}', '\u{2019}', '\u{2018}'),
        ),
        (
            "lld",
            QuotesData('\u{201d}', '\u{201c}', '\u{2019}', '\u{2018}'),
        ),
        (
            "ms-Arab",
            QuotesData('\u{201d}', '\u{201c}', '\u{2019}', '\u{2018}'),
        ),
        (
            "syr",
            QuotesData('\u{201d}', '\u{201c}', '\u{2019}', '\u{2018}'),
        ),
        (
            "ur",
            QuotesData('\u{201d}', '\u{201c}', '\u{2019}', '\u{2018}'),
        ),
        (
            "ast",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        (
            "blo",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("bm", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        ("br", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        ("ca", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "dyo",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("el", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "es-US",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("eu", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "ewo",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("ie", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        ("it", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "kab",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("kk", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "lij",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("mg", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "mua",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        (
            "nnh",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        (
            "pt-PT",
            QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}'),
        ),
        ("sc", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        ("sg", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        ("sq", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        ("ti", QuotesData('\u{ab}', '\u{bb}', '\u{201c}', '\u{201d}')),
        (
            "bas",
            QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}'),
        ),
        ("be", QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}')),
        ("cv", QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}')),
        ("ky", QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}')),
        ("ru", QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}')),
        (
            "sah",
            QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}'),
        ),
        ("uk", QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201c}')),
        (
            "bg",
            QuotesData('\u{201e}', '\u{201c}', '\u{201e}', '\u{201c}'),
        ),
        (
            "lt",
            QuotesData('\u{201e}', '\u{201c}', '\u{201e}', '\u{201c}'),
        ),
        (
            "bs-Cyrl",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "cs",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "de",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "dsb",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "et",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "hr",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "hsb",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "is",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "lb",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "luy",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "mk",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "sk",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "sl",
            QuotesData('\u{201e}', '\u{201c}', '\u{201a}', '\u{2018}'),
        ),
        (
            "bs",
            QuotesData('\u{201e}', '\u{201d}', '\u{2018}', '\u{2019}'),
        ),
        (
            "dua",
            QuotesData('\u{ab}', '\u{bb}', '\u{2018}', '\u{2019}'),
        ),
        (
            "el-POLYTON",
            QuotesData('\u{ab}', '\u{bb}', '\u{2018}', '\u{2019}'),
        ),
        (
            "ksf",
            QuotesData('\u{ab}', '\u{bb}', '\u{2018}', '\u{2019}'),
        ),
        ("no", QuotesData('\u{ab}', '\u{bb}', '\u{2018}', '\u{2019}')),
        ("rw", QuotesData('\u{ab}', '\u{bb}', '\u{2018}', '\u{2019}')),
        (
            "fi",
            QuotesData('\u{201d}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "he",
            QuotesData('\u{201d}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "lag",
            QuotesData('\u{201d}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "rn",
            QuotesData('\u{201d}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "sn",
            QuotesData('\u{201d}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "sv",
            QuotesData('\u{201d}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "fr-CA",
            QuotesData('\u{ab}', '\u{bb}', '\u{201d}', '\u{201c}'),
        ),
        ("fr", QuotesData('\u{ab}', '\u{bb}', '\u{ab}', '\u{bb}')),
        ("hy", QuotesData('\u{ab}', '\u{bb}', '\u{ab}', '\u{bb}')),
        ("yav", QuotesData('\u{ab}', '\u{bb}', '\u{ab}', '\u{bb}')),
        ("hu", QuotesData('\u{201e}', '\u{201d}', '\u{bb}', '\u{ab}')),
        (
            "ia",
            QuotesData('\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}'),
        ),
        (
            "nso",
            QuotesData('\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}'),
        ),
        (
            "ti-ER",
            QuotesData('\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}'),
        ),
        (
            "tn",
            QuotesData('\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}'),
        ),
        (
            "ja",
            QuotesData('\u{300c}', '\u{300d}', '\u{300e}', '\u{300f}'),
        ),
        (
            "yue",
            QuotesData('\u{300c}', '\u{300d}', '\u{300e}', '\u{300f}'),
        ),
        (
            "zh-Hant",
            QuotesData('\u{300c}', '\u{300d}', '\u{300e}', '\u{300f}'),
        ),
        ("ka", QuotesData('\u{201e}', '\u{201c}', '\u{ab}', '\u{bb}')),
        (
            "nl",
            QuotesData('\u{2018}', '\u{2019}', '\u{2018}', '\u{2019}'),
        ),
        (
            "nmg",
            QuotesData('\u{201e}', '\u{201d}', '\u{ab}', '\u{bb}'),
        ),
        ("pl", QuotesData('\u{201e}', '\u{201d}', '\u{ab}', '\u{bb}')),
        ("ro", QuotesData('\u{201e}', '\u{201d}', '\u{ab}', '\u{bb}')),
        (
            "shi",
            QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201d}'),
        ),
        (
            "zgh",
            QuotesData('\u{ab}', '\u{bb}', '\u{201e}', '\u{201d}'),
        ),
        (
            "sr",
            QuotesData('\u{201e}', '\u{201d}', '\u{2019}', '\u{2019}'),
        ),
        (
            "st",
            QuotesData('\u{201c}', '\u{2019}', '\u{201c}', '\u{201d}'),
        ),
        (
            "tk",
            QuotesData('\u{201c}', '\u{201d}', '\u{201c}', '\u{201d}'),
        ),
        (
            "uz",
            QuotesData('\u{201c}', '\u{201d}', '\u{2019}', '\u{2018}'),
        ),
    ];
    HashMap::from(input)
}

fn quotes_data_for_lang(lang: &str) -> QuotesData {
    // All valid language codes are at least two bytes long.
    if lang.len() < 2 {
        return DEFAULT_QUOTES;
    }

    let quotes_map = QUOTES_MAP.get_or_init(create_quotes_map);

    // Found an exact match for the requested lang.
    if let Some(quotes_data) = quotes_map.get(lang) {
        return *quotes_data;
    }

    // Try parsing lang as a Locale and canonicalizing the subtags, then see if
    // we can match it with region or script subtags, if present, or just the
    // primary language tag.
    let locale = match lang.parse::<Locale>() {
        Err(_) => return DEFAULT_QUOTES,
        Ok(locale) => locale,
    };

    let lang = locale.id.language.to_string();

    if let Some(quotes_data) = quotes_map.get(lang.as_str()) {
        return *quotes_data;
    }

    if let Some(quotes_data) = locale
        .id
        .region
        .and_then(|region| quotes_map.get(format!("{lang}-{region}").as_str()))
    {
        return *quotes_data;
    }

    if let Some(quotes_data) = locale
        .id
        .script
        .and_then(|script| quotes_map.get(format!("{lang}-{script}").as_str()))
    {
        return *quotes_data;
    }

    DEFAULT_QUOTES
}

pub fn quotes_for_lang(lang: &str, depth: usize) -> QuotePair {
    let quotes_data = quotes_data_for_lang(lang);
    match depth {
        0 => QuotePair {
            opening: quotes_data.0,
            closing: quotes_data.1,
        },
        _ => QuotePair {
            opening: quotes_data.2,
            closing: quotes_data.3,
        },
    }
}
