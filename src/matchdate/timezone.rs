static UTC_OFFSET_BY_ABBR: &[(&str, &str)] = &[
    ("ACDT", "+10:30"),
    ("ACST", "+09:30"),
    ("ACT", "+09:30"), // Australian Central Time
    // ("ACT", "-05:00"), // Acre Time
    ("ACWST", "+08:45"),
    ("ADT", "-03:00"),
    ("AEDT", "+11:00"),
    ("AEST", "+10:00"),
    ("AFT", "+04:30"),
    ("AKDT", "-08:00"),
    ("AKST", "-09:00"),
    ("ALMT", "+06:00"),
    ("AMST", "-03:00"),
    ("AMT", "+04:00"), // Armenia Time
    // ("AMT", "-04:00"), // Amazon Time
    ("ANAT", "+12:00"),
    ("AQTT", "+05:00"),
    ("ART", "-03:00"),
    ("AST", "+03:00"),
    ("AST", "-04:00"),
    ("AWST", "+08:00"),
    ("AZOST", "+00:00"),
    ("AZOT", "-01:00"),
    ("AZT", "+04:00"),
    ("BIOT", "+06:00"),
    ("BIT", "-12:00"),
    ("BNT", "+08:00"),
    ("BOT", "-04:00"),
    ("BRST", "-02:00"),
    ("BRT", "-03:00"),
    ("BST", "+01:00"), // British Summer Time
    // ("BST", "+06:00"), // Bangladesh Standard Time
    // ("BST", "+11:00"), // Bougainville Standard Time
    ("BTT", "+06:00"),
    ("CAT", "+02:00"),
    ("CCT", "+06:30"),
    // ("CDT", "-04:00"), // Cuba Daylight Time
    ("CDT", "-05:00"), // Central Daylight Time
    ("CEST", "+02:00"),
    ("CET", "+01:00"),
    ("CHADT", "+13:45"),
    ("CHAST", "+12:45"),
    ("CHOST", "+09:00"),
    ("CHOT", "+08:00"),
    ("CHST", "+10:00"),
    ("CHUT", "+10:00"),
    ("CIST", "-08:00"),
    ("CKT", "-10:00"),
    ("CLST", "-03:00"),
    ("CLT", "-04:00"),
    ("COST", "-04:00"),
    ("COT", "-05:00"),
    // ("CST", "+08:00"), // China Standard Time
    // ("CST", "-05:00"), // Cuba Standard Time
    ("CST", "-06:00"), // Central Standard Time
    ("CVT", "-01:00"),
    ("CWST", "+08:45"),
    ("CXT", "+07:00"),
    ("DAVT", "+07:00"),
    ("DDUT", "+10:00"),
    ("DFT", "+01:00"),
    ("EASST", "-05:00"),
    ("EAST", "-06:00"),
    ("EAT", "+03:00"),
    ("ECT", "-05:00"),
    ("EDT", "-04:00"),
    ("EEST", "+03:00"),
    ("EET", "+02:00"),
    ("EGST", "+00:00"),
    ("EGT", "-01:00"),
    ("EST", "-05:00"),
    ("FET", "+03:00"),
    ("FJT", "+12:00"),
    ("FKST", "-03:00"),
    ("FKT", "-04:00"),
    ("FNT", "-02:00"),
    ("GALT", "-06:00"),
    ("GAMT", "-09:00"),
    ("GET", "+04:00"),
    ("GFT", "-03:00"),
    ("GILT", "+12:00"),
    ("GIT", "-09:00"),
    ("GMT", "+00:00"),
    // ("GST", "+04:00"), // Gulf Standard Time
    ("GST", "-02:00"), // South Georgia Time
    ("GYT", "-04:00"),
    ("HAEC", "+02:00"),
    ("HDT", "-09:00"),
    ("HKT", "+08:00"),
    ("HMT", "+05:00"),
    ("HOVST", "+08:00"),
    ("HOVT", "+07:00"),
    ("HST", "-10:00"),
    ("ICT", "+07:00"),
    ("IDLW", "-12:00"),
    ("IDT", "+03:00"),
    ("IOT", "+06:00"),
    ("IRDT", "+04:30"),
    ("IRKT", "+08:00"),
    ("IRST", "+03:30"),
    ("IST", "+01:00"), // Irish Standard Time
    // ("IST", "+02:00"), // Israel Standard Time
    // ("IST", "+05:30"), // India Standard Time
    ("JST", "+09:00"),
    ("KALT", "+02:00"),
    ("KGT", "+06:00"),
    ("KOST", "+11:00"),
    ("KRAT", "+07:00"),
    ("KST", "+09:00"),
    ("LHST", "+10:30"),
    ("LHST", "+11:00"),
    ("LINT", "+14:00"),
    ("MAGT", "+12:00"),
    ("MART", "−09:30"),
    ("MAWT", "+05:00"),
    ("MDT", "-06:00"),
    ("MEST", "+02:00"),
    ("MET", "+01:00"),
    ("MHT", "+12:00"),
    ("MIST", "+11:00"),
    ("MIT", "−09:30"),
    ("MMT", "+06:30"),
    ("MSK", "+03:00"),
    // ("MST", "+08:00"), // Malaysian Standard Time
    ("MST", "-07:00"), // Mountain Standard Time
    ("MUT", "+04:00"),
    ("MVT", "+05:00"),
    ("MYT", "+08:00"),
    ("NCT", "+11:00"),
    ("NDT", "−02:30"),
    ("NFT", "+11:00"),
    ("NOVT", "+07:00"),
    ("NPT", "+05:45"),
    ("NST", "−03:30"),
    ("NT", "−03:30"),
    ("NUT", "-11:00"),
    ("NZDT", "+13:00"),
    ("NZST", "+12:00"),
    ("OMST", "+06:00"),
    ("ORAT", "+05:00"),
    ("PDT", "-07:00"),
    ("PET", "-05:00"),
    ("PETT", "+12:00"),
    ("PGT", "+10:00"),
    ("PHOT", "+13:00"),
    ("PHST", "+08:00"),
    ("PHT", "+08:00"),
    ("PKT", "+05:00"),
    ("PMDT", "-02:00"),
    ("PMST", "-03:00"),
    ("PONT", "+11:00"),
    ("PST", "-08:00"),
    ("PWT", "+09:00"),
    ("PYST", "-03:00"),
    ("PYT", "-04:00"),
    ("RET", "+04:00"),
    ("ROTT", "-03:00"),
    ("SAKT", "+11:00"),
    ("SAMT", "+04:00"),
    ("SAST", "+02:00"),
    ("SBT", "+11:00"),
    ("SCT", "+04:00"),
    ("SDT", "-10:00"),
    ("SGT", "+08:00"),
    ("SLST", "+05:30"),
    ("SRET", "+11:00"),
    ("SRT", "-03:00"),
    ("SST", "+08:00"), // Singapore Standard Time
    // ("SST", "-11:00"), // Samoa Standard Time
    ("SYOT", "+03:00"),
    ("TAHT", "-10:00"),
    ("TFT", "+05:00"),
    ("THA", "+07:00"),
    ("TJT", "+05:00"),
    ("TKT", "+13:00"),
    ("TLT", "+09:00"),
    ("TMT", "+05:00"),
    ("TOT", "+13:00"),
    ("TRT", "+03:00"),
    ("TST", "+08:00"),
    ("TVT", "+12:00"),
    ("ULAST", "+09:00"),
    ("ULAT", "+08:00"),
    ("UTC", "+00:00"),
    ("UYST", "-02:00"),
    ("UYT", "-03:00"),
    ("UZT", "+05:00"),
    ("VET", "-04:00"),
    ("VLAT", "+10:00"),
    ("VOLT", "+03:00"),
    ("VOST", "+06:00"),
    ("VUT", "+11:00"),
    ("WAKT", "+12:00"),
    ("WAST", "+02:00"),
    ("WAT", "+01:00"),
    ("WEST", "+01:00"),
    ("WET", "+00:00"),
    ("WGST", "-02:00"),
    ("WGT", "-03:00"),
    ("WIB", "+07:00"),
    ("WIT", "+09:00"),
    ("WITA", "+08:00"),
    ("WST", "+08:00"),
    ("YAKT", "+09:00"),
    ("YEKT", "+05:00"),
];

pub fn utc_offset(value: &str) -> Option<String> {
    if let Some(offset) = utc_offset_by_abbr(value) {
        return Some(offset);
    } else if let Some(offset) = utc_offset_by_offset(value) {
        return Some(offset);
    }

    None
}

fn utc_offset_by_abbr(abbr: &str) -> Option<String> {
    UTC_OFFSET_BY_ABBR.iter().find_map(|(abbr_, offset)| {
        if abbr_ == &abbr {
            Some(offset.to_string())
        } else {
            None
        }
    })
}

fn utc_offset_by_offset(offset: &str) -> Option<String> {
    let operator = match offset.chars().next() {
        Some(c) if ['+', '-'].contains(&c) => c,
        _ => {
            return None;
        }
    };

    // support +xx and +xxxx
    let offset = format!("{:0<4}", offset[1..].replace(':', ""));

    if offset.len() != 4 {
        return None;
    }

    // validate that number is between 0000 and 1200
    match offset.parse::<i32>() {
        Ok(number) if (0..=1200).contains(&number) => {
            Some(format!("{}{}:{}", operator, &offset[0..2], &offset[2..4]))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_utc_offset() {
        // invalid
        assert_eq!(utc_offset("FOO"), None);
        assert_eq!(utc_offset("+FOO"), None);
        assert_eq!(utc_offset("+1205"), None);
        assert_eq!(utc_offset("-1205"), None);

        // valid abbreviation
        assert_eq!(utc_offset("CEST"), Some("+02:00".to_string()));
        assert_eq!(utc_offset("UTC"), Some("+00:00".to_string()));

        // valid number
        assert_eq!(utc_offset("+01"), Some("+01:00".to_string()));
        assert_eq!(utc_offset("+01:00"), Some("+01:00".to_string()));
        assert_eq!(utc_offset("+0100"), Some("+01:00".to_string()));
        assert_eq!(utc_offset("-0145"), Some("-01:45".to_string()));
    }
}
