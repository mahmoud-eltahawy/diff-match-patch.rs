use core::char;
use diff_match_patch::{self, Diff};
use std::collections::HashMap;

pub fn diff_rebuildtexts(diffs: Vec<diff_match_patch::Diff>) -> Vec<String> {
    let mut text1: String = "".to_string();
    let mut text2: String = "".to_string();
    for x in 0..diffs.len() {
        if let Diff::Keep(_) | Diff::Delete(_) = diffs[x] {
            text1 += diffs[x].text();
        }
        if let Diff::Keep(_) | Diff::Add(_) = diffs[x] {
            text2 += diffs[x].text();
        }
    }

    vec![text1, text2]
}

#[test]
pub fn test_diff_common_prefix() {
    let dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        0,
        dmp.diff_common_prefix(
            &("abc".to_string().chars().collect::<Vec<_>>()),
            &("xyz".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        4,
        dmp.diff_common_prefix(
            &("1234abcdef".to_string().chars().collect::<Vec<_>>()),
            &("1234xyz".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        4,
        dmp.diff_common_prefix(
            &("1234".to_string().chars().collect::<Vec<_>>()),
            &("1234xyz".to_string().chars().collect::<Vec<_>>())
        )
    );
}

#[test]
pub fn test_diff_common_suffix() {
    let dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        0,
        dmp.diff_common_suffix(
            &("abc".to_string().chars().collect::<Vec<_>>()),
            &("xyz".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        4,
        dmp.diff_common_suffix(
            &("abcdef1234".to_string().chars().collect::<Vec<_>>()),
            &("xyz1234".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        4,
        dmp.diff_common_suffix(
            &("1234".to_string().chars().collect::<Vec<_>>()),
            &("xyz1234".to_string().chars().collect::<Vec<_>>())
        )
    );
}

#[test]
pub fn test_diff_common_overlap() {
    let dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        0,
        dmp.diff_common_overlap(
            &("".to_string().chars().collect::<Vec<_>>()),
            &("abcd".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        3,
        dmp.diff_common_overlap(
            &("abc".to_string().chars().collect::<Vec<_>>()),
            &("abcd".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        0,
        dmp.diff_common_overlap(
            &("123456".to_string().chars().collect::<Vec<_>>()),
            &("abcd".to_string().chars().collect::<Vec<_>>())
        )
    );

    assert_eq!(
        3,
        dmp.diff_common_overlap(
            &("123456xxx".to_string().chars().collect::<Vec<_>>()),
            &("xxxabcd".to_string().chars().collect::<Vec<_>>())
        )
    );
}

#[test]
pub fn test_diff_half_match() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.diff_timeout = Some(1.0);
    let temp: Vec<String> = vec![];
    assert_eq!(
        temp,
        dmp.diff_half_match(
            &("1234567890".to_string().chars().collect()),
            &("abcdef".to_string().chars().collect())
        )
    );
    assert_eq!(
        temp,
        dmp.diff_half_match(
            &("12345".to_string().chars().collect()),
            &("23".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("12,90,a,z,345678", ','),
        dmp.diff_half_match(
            &("1234567890".to_string().chars().collect()),
            &("a345678z".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("a,z,12,90,345678", ','),
        dmp.diff_half_match(
            &("a345678z".to_string().chars().collect()),
            &("1234567890".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("abc,z,1234,0,56789", ','),
        dmp.diff_half_match(
            &("abc56789z".to_string().chars().collect()),
            &("1234567890".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("a,xyz,1,7890,23456", ','),
        dmp.diff_half_match(
            &("a23456xyz".to_string().chars().collect()),
            &("1234567890".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("12123,123121,a,z,1234123451234", ','),
        dmp.diff_half_match(
            &("121231234123451234123121".to_string().chars().collect()),
            &("a1234123451234z".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char(",-=-=-=-=-=,x,,x-=-=-=-=-=-=-=", ','),
        dmp.diff_half_match(
            &("x-=-=-=-=-=-=-=-=-=-=-=-=".to_string().chars().collect()),
            &("xx-=-=-=-=-=-=-=".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("-=-=-=-=-=,,,y,-=-=-=-=-=-=-=y", ','),
        dmp.diff_half_match(
            &("-=-=-=-=-=-=-=-=-=-=-=-=y".to_string().chars().collect()),
            &("-=-=-=-=-=-=-=yy".to_string().chars().collect())
        )
    );
    assert_eq!(
        dmp.split_by_char("qHillo,w,x,Hulloy,HelloHe", ','),
        dmp.diff_half_match(
            &("qHilloHelloHew".to_string().chars().collect()),
            &("xHelloHeHulloy".to_string().chars().collect())
        )
    );
}

#[test]
pub fn test_diff_half_match_no_timeout() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.diff_timeout = None;
    let empty_vec: Vec<String> = vec![];
    assert_eq!(
        empty_vec,
        dmp.diff_half_match(
            &("qHilloHelloHew".to_string().chars().collect()),
            &("xHelloHeHulloy".to_string().chars().collect())
        )
    );
}

#[test]
pub fn test_diff_lines_tochars() {
    let dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        (
            "\x01\x02\x01".to_string(),
            "\x02\x01\x02".to_string(),
            vec!["".to_string(), "alpha\n".to_string(), "beta\n".to_string()]
        ),
        dmp.diff_lines_tochars(
            &("alpha\nbeta\nalpha\n"
                .to_string()
                .chars()
                .collect::<Vec<_>>()),
            &("beta\nalpha\nbeta\n"
                .to_string()
                .chars()
                .collect::<Vec<_>>())
        )
    );
    assert_eq!(
        (
            "".to_string(),
            "\x01\x02\x03\x03".to_string(),
            vec![
                "".to_string(),
                "alpha\r\n".to_string(),
                "beta\r\n".to_string(),
                "\r\n".to_string()
            ]
        ),
        dmp.diff_lines_tochars(
            &("".to_string().chars().collect::<Vec<_>>()),
            &("alpha\r\nbeta\r\n\r\n\r\n"
                .to_string()
                .chars()
                .collect::<Vec<_>>())
        )
    );
    assert_eq!(
        (
            "\x01".to_string(),
            "\x02".to_string(),
            vec!["".to_string(), "a".to_string(), "b".to_string()]
        ),
        dmp.diff_lines_tochars(
            &("a".to_string().chars().collect::<Vec<_>>()),
            &("b".to_string().chars().collect::<Vec<_>>())
        )
    );
    let n: u32 = 300;
    let mut line_list: Vec<String> = vec![];
    let mut char_list: Vec<char> = vec![];
    for i in 1..n + 1 {
        line_list.push(i.to_string() + "\n");
        match char::from_u32(i) {
            Some(ch) => {
                char_list.push(ch);
            }
            None => {}
        }
    }
    let chars: String = char_list.into_iter().collect();
    assert_eq!(n as usize, line_list.len());
    let lines = line_list.join("");
    let lines_vec: Vec<char> = lines.chars().collect();
    assert_eq!(n as usize, chars.chars().count());
    line_list.insert(0, "".to_string());
    assert_eq!(
        (chars, "".to_string(), line_list),
        dmp.diff_lines_tochars(&lines_vec, &vec![])
    )
}

#[test]
pub fn test_diff_words_tochars() {
    let mut dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        (
            "\x01\x02\x03\x02\x01".to_string(),
            "\x03\x02\x01\x02\x03".to_string(),
            vec![
                "".to_string(),
                "alpha".to_string(),
                " ".to_string(),
                "beta".to_string()
            ]
        ),
        dmp.diff_words_tochars(
            &"alpha beta alpha".to_string(),
            &"beta alpha beta".to_string()
        )
    );
    assert_eq!(
        (
            "\x01\x02".to_string(),
            "\x03\x02\x01".to_string(),
            vec![
                "".to_string(),
                "alpha".to_string(),
                "\n".to_string(),
                "beta".to_string()
            ]
        ),
        dmp.diff_words_tochars(&"alpha\n".to_string(), &"beta\nalpha".to_string())
    );
    let old_string = "betty bought some butter ".to_string();
    let new_string = "betty sought some butter".to_string();
    let mut diff_arr = vec![
        diff_match_patch::Diff::Keep("betty ".to_string()),
        diff_match_patch::Diff::Delete("b".to_string()),
        diff_match_patch::Diff::Add("s".to_string()),
        diff_match_patch::Diff::Keep("ought some butter".to_string()),
        diff_match_patch::Diff::Delete(" ".to_string()),
    ];
    println!("{:?}", diff_arr);
    assert_eq!(diff_arr, dmp.diff_main(&old_string, &new_string, true));

    diff_arr = vec![
        diff_match_patch::Diff::Keep("betty ".to_string()),
        diff_match_patch::Diff::Delete("bought".to_string()),
        diff_match_patch::Diff::Add("sought".to_string()),
        diff_match_patch::Diff::Keep(" some butter".to_string()),
        diff_match_patch::Diff::Delete(" ".to_string()),
    ];

    let (chars1, chars2, hash_arr) = dmp.diff_words_tochars(&old_string, &new_string);
    let mut res_diffs = dmp.diff_main(&chars1, &chars2, true);
    dmp.diff_chars_tolines(&mut res_diffs, &hash_arr);
    println!("{:?}", &res_diffs);
    assert_eq!(diff_arr, res_diffs);
}

#[test]
pub fn test_diff_chars_tolines() {
    let dmp = diff_match_patch::Dmp::new();
    let mut diffs = vec![
        diff_match_patch::Diff::Keep("\x01\x02\x01".to_string()),
        diff_match_patch::Diff::Add("\x02\x01\x02".to_string()),
    ];
    dmp.diff_chars_tolines(
        &mut diffs,
        &vec!["".to_string(), "alpha\n".to_string(), "beta\n".to_string()],
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("alpha\nbeta\nalpha\n".to_string()),
            diff_match_patch::Diff::Add("beta\nalpha\nbeta\n".to_string())
        ],
        diffs
    );

    let n: u32 = 300;
    let mut line_list: Vec<String> = vec![];
    let mut char_list: Vec<char> = vec![];
    for i in 1..n + 1 {
        line_list.push(i.to_string() + "\n");
        char_list.push(char::from_u32(i).unwrap());
    }
    let chars: String = char_list.into_iter().collect();
    assert_eq!(n as usize, line_list.len());
    let lines = line_list.join("");
    assert_eq!(n as usize, chars.chars().count());
    line_list.insert(0, "".to_string());
    let mut diffs = vec![diff_match_patch::Diff::Delete(chars)];
    dmp.diff_chars_tolines(&mut diffs, &line_list);
    assert_eq!(diffs, vec![diff_match_patch::Diff::Delete(lines)]);
}

#[test]
pub fn diff_lines_tochars_munge() {
    let dmp = diff_match_patch::Dmp::new();

    // Unicode codepoints from 55296 to 57344 are reserved and can't be used as a scalar
    let number_of_lines = 60000;

    let mut text: Vec<char> = Vec::with_capacity(number_of_lines);
    for i in 0..=number_of_lines {
        text.extend(i.to_string().chars());
        if i + 1 < number_of_lines {
            text.extend("\n".chars());
        }
    }

    let mut linearray: Vec<String> = vec!["".to_string()];
    let mut linehash: HashMap<String, i32> = HashMap::new();
    let chars1 = dmp.diff_lines_tochars_munge(&text, &mut linearray, &mut linehash);

    assert_eq!(chars1.chars().count(), number_of_lines);
    assert_eq!(linearray.len() - 1, number_of_lines);
    assert_eq!(linehash.len(), number_of_lines);
}

#[test]
pub fn test_diff_cleanup_merge() {
    let dmp = diff_match_patch::Dmp::new();
    let mut diffs: Vec<diff_match_patch::Diff> = vec![];
    let temp: Vec<diff_match_patch::Diff> = vec![];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(temp, diffs);

    // No change case.
    diffs = vec![
        diff_match_patch::Diff::Keep("a".to_string()),
        diff_match_patch::Diff::Delete("b".to_string()),
        diff_match_patch::Diff::Add("c".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("b".to_string()),
            diff_match_patch::Diff::Add("c".to_string())
        ],
        diffs
    );

    // Merge equalities.
    diffs = vec![
        diff_match_patch::Diff::Keep("a".to_string()),
        diff_match_patch::Diff::Keep("b".to_string()),
        diff_match_patch::Diff::Keep("c".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(vec![diff_match_patch::Diff::Keep("abc".to_string())], diffs);

    // Merge deletions.
    diffs = vec![
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Delete("b".to_string()),
        diff_match_patch::Diff::Delete("c".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![diff_match_patch::Diff::Delete("abc".to_string())],
        diffs
    );

    // Merge insertions.
    diffs = vec![
        diff_match_patch::Diff::Add("a".to_string()),
        diff_match_patch::Diff::Add("b".to_string()),
        diff_match_patch::Diff::Add("c".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(vec![diff_match_patch::Diff::Add("abc".to_string())], diffs);

    // Merge interweave.
    diffs = vec![
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Add("b".to_string()),
        diff_match_patch::Diff::Delete("c".to_string()),
        diff_match_patch::Diff::Add("d".to_string()),
        diff_match_patch::Diff::Keep("e".to_string()),
        diff_match_patch::Diff::Keep("f".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("ac".to_string()),
            diff_match_patch::Diff::Add("bd".to_string()),
            diff_match_patch::Diff::Keep("ef".to_string())
        ],
        diffs
    );

    // Prefix and suffix detection.
    diffs = vec![
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Add("abc".to_string()),
        diff_match_patch::Diff::Delete("dc".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("d".to_string()),
            diff_match_patch::Diff::Add("b".to_string()),
            diff_match_patch::Diff::Keep("c".to_string())
        ],
        diffs
    );

    // Prefix and suffix detection with equalities.
    diffs = vec![
        diff_match_patch::Diff::Keep("x".to_string()),
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Add("abc".to_string()),
        diff_match_patch::Diff::Delete("dc".to_string()),
        diff_match_patch::Diff::Keep("y".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("xa".to_string()),
            diff_match_patch::Diff::Delete("d".to_string()),
            diff_match_patch::Diff::Add("b".to_string()),
            diff_match_patch::Diff::Keep("cy".to_string())
        ],
        diffs
    );

    // Slide edit left.
    diffs = vec![
        diff_match_patch::Diff::Keep("a".to_string()),
        diff_match_patch::Diff::Add("ba".to_string()),
        diff_match_patch::Diff::Keep("c".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Add("ab".to_string()),
            diff_match_patch::Diff::Keep("ac".to_string())
        ],
        diffs
    );

    // Slide edit right.
    diffs = vec![
        diff_match_patch::Diff::Keep("c".to_string()),
        diff_match_patch::Diff::Add("ab".to_string()),
        diff_match_patch::Diff::Keep("a".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("ca".to_string()),
            diff_match_patch::Diff::Add("ba".to_string())
        ],
        diffs
    );

    // # Slide edit left recursive.
    diffs = vec![
        diff_match_patch::Diff::Keep("a".to_string()),
        diff_match_patch::Diff::Delete("b".to_string()),
        diff_match_patch::Diff::Keep("c".to_string()),
        diff_match_patch::Diff::Delete("ac".to_string()),
        diff_match_patch::Diff::Keep("x".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Keep("acx".to_string())
        ],
        diffs
    );

    // # Slide edit right recursive.
    diffs = vec![
        diff_match_patch::Diff::Keep("x".to_string()),
        diff_match_patch::Diff::Delete("ca".to_string()),
        diff_match_patch::Diff::Keep("c".to_string()),
        diff_match_patch::Diff::Delete("b".to_string()),
        diff_match_patch::Diff::Keep("a".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("xca".to_string()),
            diff_match_patch::Diff::Delete("cba".to_string())
        ],
        diffs
    );

    // # Empty merge.
    diffs = vec![
        diff_match_patch::Diff::Delete("b".to_string()),
        diff_match_patch::Diff::Add("ab".to_string()),
        diff_match_patch::Diff::Keep("c".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Add("a".to_string()),
            diff_match_patch::Diff::Keep("bc".to_string())
        ],
        diffs
    );

    // # Empty equality.
    diffs = vec![
        diff_match_patch::Diff::Keep("".to_string()),
        diff_match_patch::Diff::Add("a".to_string()),
        diff_match_patch::Diff::Keep("b".to_string()),
    ];
    dmp.diff_cleanup_merge(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Add("a".to_string()),
            diff_match_patch::Diff::Keep("b".to_string())
        ],
        diffs
    );
}

#[test]
pub fn test_diff_cleanup_semantic_lossless() {
    // Slide diffs to match logical boundaries.
    // Null case.
    let dmp = diff_match_patch::Dmp::new();
    let mut diffs: Vec<diff_match_patch::Diff> = vec![];
    let temp: Vec<diff_match_patch::Diff> = vec![];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(temp, diffs);

    // Blank lines.
    diffs = vec![
        diff_match_patch::Diff::Keep("AAA\r\n\r\nBBB".to_string()),
        diff_match_patch::Diff::Add("\r\nDDD\r\n\r\nBBB".to_string()),
        diff_match_patch::Diff::Keep("\r\nEEE".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("AAA\r\n\r\n".to_string()),
            diff_match_patch::Diff::Add("BBB\r\nDDD\r\n\r\n".to_string()),
            diff_match_patch::Diff::Keep("BBB\r\nEEE".to_string())
        ],
        diffs
    );

    // # Line boundaries.
    diffs = vec![
        diff_match_patch::Diff::Keep("AAA\r\nBBB".to_string()),
        diff_match_patch::Diff::Add(" DDD\r\nBBB".to_string()),
        diff_match_patch::Diff::Keep(" EEE".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("AAA\r\n".to_string()),
            diff_match_patch::Diff::Add("BBB DDD\r\n".to_string()),
            diff_match_patch::Diff::Keep("BBB EEE".to_string())
        ],
        diffs
    );

    // # Word boundaries.
    diffs = vec![
        diff_match_patch::Diff::Keep("The c".to_string()),
        diff_match_patch::Diff::Add("ow and the c".to_string()),
        diff_match_patch::Diff::Keep("at.".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("The ".to_string()),
            diff_match_patch::Diff::Add("cow and the ".to_string()),
            diff_match_patch::Diff::Keep("cat.".to_string())
        ],
        diffs
    );

    // # Alphanumeric boundaries.
    diffs = vec![
        diff_match_patch::Diff::Keep("The-c".to_string()),
        diff_match_patch::Diff::Add("ow-and-the-c".to_string()),
        diff_match_patch::Diff::Keep("at.".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("The-".to_string()),
            diff_match_patch::Diff::Add("cow-and-the-".to_string()),
            diff_match_patch::Diff::Keep("cat.".to_string())
        ],
        diffs
    );

    // # Hitting the start.
    diffs = vec![
        diff_match_patch::Diff::Keep("a".to_string()),
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Keep("ax".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("a".to_string()),
            diff_match_patch::Diff::Keep("aax".to_string())
        ],
        diffs
    );

    // # Hitting the end.
    diffs = vec![
        diff_match_patch::Diff::Keep("xa".to_string()),
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Keep("a".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("xaa".to_string()),
            diff_match_patch::Diff::Delete("a".to_string())
        ],
        diffs
    );

    // # Sentence boundaries.
    diffs = vec![
        diff_match_patch::Diff::Keep("The xxx. The ".to_string()),
        diff_match_patch::Diff::Add("zzz. The ".to_string()),
        diff_match_patch::Diff::Keep("yyy.".to_string()),
    ];
    dmp.diff_cleanup_semantic_lossless(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("The xxx.".to_string()),
            diff_match_patch::Diff::Add(" The zzz.".to_string()),
            diff_match_patch::Diff::Keep(" The yyy.".to_string())
        ],
        diffs
    );
}

#[test]
pub fn test_diff_cleanup_semantic() {
    let dmp = diff_match_patch::Dmp::new();

    //  Null case.
    let mut diffs: Vec<diff_match_patch::Diff> = vec![];
    let temp: Vec<diff_match_patch::Diff> = vec![];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(diffs, temp);

    // No elimination #1.
    diffs = vec![
        diff_match_patch::Diff::Delete("ab".to_string()),
        diff_match_patch::Diff::Add("cd".to_string()),
        diff_match_patch::Diff::Keep("c12".to_string()),
        diff_match_patch::Diff::Delete("e".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("ab".to_string()),
            diff_match_patch::Diff::Add("cd".to_string()),
            diff_match_patch::Diff::Keep("c12".to_string()),
            diff_match_patch::Diff::Delete("e".to_string())
        ],
        diffs
    );

    // No elimination #2.
    diffs = vec![
        diff_match_patch::Diff::Delete("abc".to_string()),
        diff_match_patch::Diff::Add("ABC".to_string()),
        diff_match_patch::Diff::Keep("1234".to_string()),
        diff_match_patch::Diff::Delete("wxyz".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Add("ABC".to_string()),
            diff_match_patch::Diff::Keep("1234".to_string()),
            diff_match_patch::Diff::Delete("wxyz".to_string())
        ],
        diffs
    );

    // Simple elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Keep("b".to_string()),
        diff_match_patch::Diff::Delete("c".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Add("b".to_string())
        ],
        diffs
    );

    // Backpass elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("ab".to_string()),
        diff_match_patch::Diff::Keep("cd".to_string()),
        diff_match_patch::Diff::Delete("e".to_string()),
        diff_match_patch::Diff::Keep("f".to_string()),
        diff_match_patch::Diff::Add("g".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abcdef".to_string()),
            diff_match_patch::Diff::Add("cdfg".to_string())
        ],
        diffs
    );

    // Multiple eliminations.
    diffs = vec![
        diff_match_patch::Diff::Add("1".to_string()),
        diff_match_patch::Diff::Keep("A".to_string()),
        diff_match_patch::Diff::Delete("B".to_string()),
        diff_match_patch::Diff::Add("2".to_string()),
        diff_match_patch::Diff::Keep("_".to_string()),
        diff_match_patch::Diff::Add("1".to_string()),
        diff_match_patch::Diff::Keep("A".to_string()),
        diff_match_patch::Diff::Delete("B".to_string()),
        diff_match_patch::Diff::Add("2".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("AB_AB".to_string()),
            diff_match_patch::Diff::Add("1A2_1A2".to_string())
        ],
        diffs
    );

    // Word boundaries.
    diffs = vec![
        diff_match_patch::Diff::Keep("The c".to_string()),
        diff_match_patch::Diff::Delete("ow and the c".to_string()),
        diff_match_patch::Diff::Keep("at.".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("The ".to_string()),
            diff_match_patch::Diff::Delete("cow and the ".to_string()),
            diff_match_patch::Diff::Keep("cat.".to_string())
        ],
        diffs
    );

    // No overlap elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("abcxx".to_string()),
        diff_match_patch::Diff::Add("xxdef".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abcxx".to_string()),
            diff_match_patch::Diff::Add("xxdef".to_string())
        ],
        diffs
    );

    // Overlap elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("abcxxx".to_string()),
        diff_match_patch::Diff::Add("xxxdef".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Keep("xxx".to_string()),
            diff_match_patch::Diff::Add("def".to_string())
        ],
        diffs
    );

    // Reverse overlap elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("xxxabc".to_string()),
        diff_match_patch::Diff::Add("defxxx".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Add("def".to_string()),
            diff_match_patch::Diff::Keep("xxx".to_string()),
            diff_match_patch::Diff::Delete("abc".to_string())
        ],
        diffs
    );

    // Two overlap eliminations.
    diffs = vec![
        diff_match_patch::Diff::Delete("abcd1212".to_string()),
        diff_match_patch::Diff::Add("1212efghi".to_string()),
        diff_match_patch::Diff::Keep("----".to_string()),
        diff_match_patch::Diff::Delete("A3".to_string()),
        diff_match_patch::Diff::Add("3BC".to_string()),
    ];
    dmp.diff_cleanup_semantic(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abcd".to_string()),
            diff_match_patch::Diff::Keep("1212".to_string()),
            diff_match_patch::Diff::Add("efghi".to_string()),
            diff_match_patch::Diff::Keep("----".to_string()),
            diff_match_patch::Diff::Delete("A".to_string()),
            diff_match_patch::Diff::Keep("3".to_string()),
            diff_match_patch::Diff::Add("BC".to_string())
        ],
        diffs
    );
}

#[test]
pub fn test_diff_cleanup_efficiency() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.edit_cost = 4;
    // Null case.
    let mut diffs: Vec<diff_match_patch::Diff> = vec![];
    let temp: Vec<diff_match_patch::Diff> = vec![];
    dmp.diff_cleanup_efficiency(&mut diffs);
    assert_eq!(temp, diffs);

    // No elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("ab".to_string()),
        diff_match_patch::Diff::Add("12".to_string()),
        diff_match_patch::Diff::Keep("wxyz".to_string()),
        diff_match_patch::Diff::Delete("cd".to_string()),
        diff_match_patch::Diff::Add("34".to_string()),
    ];
    dmp.diff_cleanup_efficiency(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("ab".to_string()),
            diff_match_patch::Diff::Add("12".to_string()),
            diff_match_patch::Diff::Keep("wxyz".to_string()),
            diff_match_patch::Diff::Delete("cd".to_string()),
            diff_match_patch::Diff::Add("34".to_string())
        ],
        diffs
    );

    // Four-edit elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("ab".to_string()),
        diff_match_patch::Diff::Add("12".to_string()),
        diff_match_patch::Diff::Keep("xyz".to_string()),
        diff_match_patch::Diff::Delete("cd".to_string()),
        diff_match_patch::Diff::Add("34".to_string()),
    ];
    dmp.diff_cleanup_efficiency(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abxyzcd".to_string()),
            diff_match_patch::Diff::Add("12xyz34".to_string())
        ],
        diffs
    );

    // Three-edit elimination.
    diffs = vec![
        diff_match_patch::Diff::Add("12".to_string()),
        diff_match_patch::Diff::Keep("x".to_string()),
        diff_match_patch::Diff::Delete("cd".to_string()),
        diff_match_patch::Diff::Add("34".to_string()),
    ];
    dmp.diff_cleanup_efficiency(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("xcd".to_string()),
            diff_match_patch::Diff::Add("12x34".to_string())
        ],
        diffs
    );

    // Backpass elimination.
    diffs = vec![
        diff_match_patch::Diff::Delete("ab".to_string()),
        diff_match_patch::Diff::Add("12".to_string()),
        diff_match_patch::Diff::Keep("xy".to_string()),
        diff_match_patch::Diff::Add("34".to_string()),
        diff_match_patch::Diff::Keep("z".to_string()),
        diff_match_patch::Diff::Delete("cd".to_string()),
        diff_match_patch::Diff::Add("56".to_string()),
    ];
    dmp.diff_cleanup_efficiency(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abxyzcd".to_string()),
            diff_match_patch::Diff::Add("12xy34z56".to_string())
        ],
        diffs
    );

    // High cost elimination.
    dmp.edit_cost = 5;
    diffs = vec![
        diff_match_patch::Diff::Delete("ab".to_string()),
        diff_match_patch::Diff::Add("12".to_string()),
        diff_match_patch::Diff::Keep("wxyz".to_string()),
        diff_match_patch::Diff::Delete("cd".to_string()),
        diff_match_patch::Diff::Add("34".to_string()),
    ];
    dmp.diff_cleanup_efficiency(&mut diffs);
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("abwxyzcd".to_string()),
            diff_match_patch::Diff::Add("12wxyz34".to_string())
        ],
        diffs
    );
}

#[test]
pub fn test_diff_text() {
    let mut dmp = diff_match_patch::Dmp::new();
    let mut diffs: Vec<diff_match_patch::Diff> = vec![
        diff_match_patch::Diff::Keep("jump".to_string()),
        diff_match_patch::Diff::Delete("s".to_string()),
        diff_match_patch::Diff::Add("ed".to_string()),
        diff_match_patch::Diff::Keep(" over ".to_string()),
        diff_match_patch::Diff::Delete("the".to_string()),
        diff_match_patch::Diff::Add("a".to_string()),
        diff_match_patch::Diff::Keep(" lazy".to_string()),
    ];
    assert_eq!(
        "jumps over the lazy".to_string(),
        dmp.diff_text1(&mut diffs)
    );
    assert_eq!("jumped over a lazy".to_string(), dmp.diff_text2(&mut diffs));
}

#[test]
pub fn test_diff_text2_u16() {
    let dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        dmp.diff_text2_from_delta_u16("🅰", "-2\t+%F0%9F%85%B1"),
        dmp.diff_text2_from_delta_u16("🅰", "=1\t-1\t+%ED%B5%B1")
    );
}

#[test]
pub fn test_diff_delta() {
    let mut dmp = diff_match_patch::Dmp::new();
    let mut diffs = vec![
        diff_match_patch::Diff::Keep("jump".to_string()),
        diff_match_patch::Diff::Delete("s".to_string()),
        diff_match_patch::Diff::Add("ed".to_string()),
        diff_match_patch::Diff::Keep(" over ".to_string()),
        diff_match_patch::Diff::Delete("the".to_string()),
        diff_match_patch::Diff::Add("a".to_string()),
        diff_match_patch::Diff::Keep(" lazy".to_string()),
        diff_match_patch::Diff::Add("old dog".to_string()),
    ];
    let mut text1 = dmp.diff_text1(&mut diffs);
    assert_eq!("jumps over the lazy".to_string(), text1);
    let mut delta = dmp.diff_todelta(&mut diffs);
    assert_eq!("=4\t-1\t+ed\t=6\t-3\t+a\t=5\t+old dog".to_string(), delta);

    // Convert delta string into a diff.
    assert_eq!(diffs, dmp.diff_from_delta(&text1, &delta));

    // # Generates error (19 != 20).
    // try:
    //     self.dmp.diff_fromDelta(text1 + "x", delta)
    //     self.assertFalse(True)
    // except ValueError:
    //     # Exception expected.
    //     pass

    // # Generates error (19 != 18).
    // try:
    //     self.dmp.diff_fromDelta(text1[1:], delta)
    //     self.assertFalse(True)
    // except ValueError:
    //     # Exception expected.
    //     pass

    // # Generates error (%c3%xy invalid Unicode).
    // # Note: Python 3 can decode this.
    // #try:
    // #  self.dmp.diff_fromDelta("", "+%c3xy")
    // #  self.assertFalse(True)
    // #except ValueError:
    // #  # Exception expected.
    // #  pass

    // Test deltas with special characters.
    diffs = vec![
        diff_match_patch::Diff::Keep("\u{0680} \x00 \t %".to_string()),
        diff_match_patch::Diff::Delete("\u{0681} \x01 \n ^".to_string()),
        diff_match_patch::Diff::Add("\u{0682} \x02 \\ |".to_string()),
    ];
    text1 = dmp.diff_text1(&mut diffs);
    assert_eq!("\u{0680} \x00 \t %\u{0681} \x01 \n ^".to_string(), text1);

    delta = dmp.diff_todelta(&mut diffs);
    assert_eq!("=7\t-7\t+%DA%82 %02 %5C %7C".to_string(), delta);
    // Convert delta string into a diff.
    assert_eq!(diffs, dmp.diff_from_delta(&text1, &delta));

    // Verify pool of unchanged characters.
    diffs = vec![diff_match_patch::Diff::Add(
        "A-Z a-z 0-9 - _ . ! ~ * ' ( ) ; / ? : @ & = + $ , # ".to_string(),
    )];
    let text2 = dmp.diff_text2(&mut diffs);
    assert_eq!(
        "A-Z a-z 0-9 - _ . ! ~ * \' ( ) ; / ? : @ & = + $ , # ".to_string(),
        text2
    );

    delta = dmp.diff_todelta(&mut diffs);
    assert_eq!(
        "+A-Z a-z 0-9 - _ . ! ~ * \' ( ) ; / ? : @ & = + $ , # ".to_string(),
        delta
    );

    // Convert delta string into a diff.
    assert_eq!(diffs, dmp.diff_from_delta(&("".to_string()), &delta));

    // 160 kb string.
    let mut a = "abcdefghij".to_string();
    for _i in 0..14 {
        a += a.clone().as_str();
    }
    diffs = vec![diff_match_patch::Diff::Add(a.clone())];
    delta = dmp.diff_todelta(&mut diffs);
    assert_eq!('+'.to_string() + a.as_str(), delta);

    // Convert delta string into a diff.
    assert_eq!(diffs, dmp.diff_from_delta(&"".to_string(), &delta));

    // Emoji
    diffs = dmp.diff_main("☺️🖖🏿", "☺️😃🖖🏿", false);
    delta = dmp.diff_todelta_unit(&mut diffs, diff_match_patch::LengthUnit::UTF16);
    assert_eq!(delta, "=2\t+%F0%9F%98%83\t=4");

    diffs = dmp.diff_main("☺️🖖🏿", "☺️😃🖖🏿", false);
    let mut patches = dmp.patch_make2(&mut diffs);
    let (patched_text_vec, _) = dmp.patch_apply(&mut patches, "☺️🖖🏿");
    let patched_text: String = patched_text_vec.into_iter().collect();
    assert_eq!(patched_text, "☺️😃🖖🏿");
}

#[test]
pub fn test_diff_delta_surrogates() {
    let dmp = diff_match_patch::Dmp::new();

    // Inserting similar surrogate pair at beginning
    let mut diffs = dmp.diff_main("🅰🅱", "🅱🅰🅱", false);
    let mut expected_diffs = vec![
        diff_match_patch::Diff::Add("🅱".to_string()),
        diff_match_patch::Diff::Keep("🅰🅱".to_string()),
    ];
    assert_eq!(diffs, expected_diffs);

    // Inserting similar surrogate pair in the middle
    diffs = dmp.diff_main("🅱🅱", "🅱🅰🅱", false);
    expected_diffs = vec![
        diff_match_patch::Diff::Keep("🅱".to_string()),
        diff_match_patch::Diff::Add("🅰".to_string()),
        diff_match_patch::Diff::Keep("🅱".to_string()),
    ];
    assert_eq!(diffs, expected_diffs);

    // Deleting similar surrogate pair at the beginning
    diffs = dmp.diff_main("🅱🅰🅱", "🅰🅱", false);
    expected_diffs = vec![
        diff_match_patch::Diff::Delete("🅱".to_string()),
        diff_match_patch::Diff::Keep("🅰🅱".to_string()),
    ];
    assert_eq!(diffs, expected_diffs);

    // Deleting similar surrogate pair in the middle
    diffs = dmp.diff_main("🅰🅲🅱", "🅰🅱", false);
    expected_diffs = vec![
        diff_match_patch::Diff::Keep("🅰".to_string()),
        diff_match_patch::Diff::Delete("🅲".to_string()),
        diff_match_patch::Diff::Keep("🅱".to_string()),
    ];
    assert_eq!(diffs, expected_diffs);

    // Swapping surrogate pairs
    diffs = dmp.diff_main("🅰", "🅱", false);
    expected_diffs = vec![
        diff_match_patch::Diff::Delete("🅰".to_string()),
        diff_match_patch::Diff::Add("🅱".to_string()),
    ];
    assert_eq!(diffs, expected_diffs);
}

#[test]
pub fn test_diff_to_delta_unit() {
    let mut dmp = diff_match_patch::Dmp::new();

    // UTF16
    let mut diffs = dmp.diff_main("🅰", "🅱", false);
    let mut delta = dmp.diff_todelta_unit(&mut diffs, diff_match_patch::LengthUnit::UTF16);
    assert_eq!(delta, "-2\t+%F0%9F%85%B1");

    // Scalar
    let mut diffs = dmp.diff_main("🅰", "🅱", false);
    delta = dmp.diff_todelta_unit(&mut diffs, diff_match_patch::LengthUnit::UnicodeScalar);
    assert_eq!(delta, "-1\t+%F0%9F%85%B1");
}

#[test]
pub fn test_diff_from_delta_unit() {
    let mut dmp = diff_match_patch::Dmp::new();

    // UTF16
    let mut delta = "-2\t=2\t+%F0%9F%85%B1";
    let mut diffs = dmp.diff_from_delta_unit("🅰🅲", delta, diff_match_patch::LengthUnit::UTF16);
    assert_eq!(dmp.diff_text2(&mut diffs), "🅲🅱");

    // Scalar
    delta = "-1\t=1\t+%F0%9F%85%B1";
    diffs = dmp.diff_from_delta_unit("🅰🅲", delta, diff_match_patch::LengthUnit::UnicodeScalar);
    assert_eq!(dmp.diff_text2(&mut diffs), "🅲🅱");
}

#[test]
pub fn test_diff_from_delta_split_surrogates() {
    let mut dmp = diff_match_patch::Dmp::new();

    assert_eq!(
        dmp.diff_from_delta_unit(
            "🅰",
            "-2\t+%F0%9F%85%B1",
            diff_match_patch::LengthUnit::UTF16
        ),
        dmp.diff_from_delta_unit(
            "🅰",
            "=1\t-1\t+%ED%B5%B1",
            diff_match_patch::LengthUnit::UTF16
        )
    );
}

#[test]
pub fn test_diff_xindex() {
    let mut dmp = diff_match_patch::Dmp::new();

    // Translate a location in text1 to text2.
    let mut diffs = vec![
        diff_match_patch::Diff::Delete("a".to_string()),
        diff_match_patch::Diff::Add("1234".to_string()),
        diff_match_patch::Diff::Keep("xyz".to_string()),
    ];
    assert_eq!(5, dmp.diff_xindex(&diffs, 2));

    // Translation on deletion.
    diffs = vec![
        diff_match_patch::Diff::Keep("a".to_string()),
        diff_match_patch::Diff::Delete("1234".to_string()),
        diff_match_patch::Diff::Keep("xyz".to_string()),
    ];
    assert_eq!(1, dmp.diff_xindex(&diffs, 3));
}

#[test]
pub fn test_diff_levenshtein() {
    let mut dmp = diff_match_patch::Dmp::new();
    assert_eq!(
        4,
        dmp.diff_levenshtein(&mut vec![
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Add("1234".to_string()),
            diff_match_patch::Diff::Keep("xyz".to_string())
        ])
    );
    // Levenshtein with leading equality.
    assert_eq!(
        4,
        dmp.diff_levenshtein(&mut vec![
            diff_match_patch::Diff::Keep("xyz".to_string()),
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Add("1234".to_string())
        ])
    );
    // # Levenshtein with middle equality.
    assert_eq!(
        7,
        dmp.diff_levenshtein(&mut vec![
            diff_match_patch::Diff::Delete("abc".to_string()),
            diff_match_patch::Diff::Keep("xyz".to_string()),
            diff_match_patch::Diff::Add("1234".to_string())
        ])
    );
}

#[test]
pub fn test_diff_bisect() {
    let mut dmp = diff_match_patch::Dmp::new();
    let a = "cat".to_string();
    let b = "map".to_string();
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("c".to_string()),
            diff_match_patch::Diff::Add("m".to_string()),
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("t".to_string()),
            diff_match_patch::Diff::Add("p".to_string())
        ],
        dmp.diff_bisect(
            &a.chars().collect::<Vec<_>>(),
            &b.chars().collect::<Vec<_>>()
        )
    );
}

#[test]
pub fn test_diff_bisect_timeout() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.diff_timeout = Some(0.0);

    let a = "cat".to_string();
    let b = "map".to_string();

    let expected = vec![
        diff_match_patch::Diff::Delete("cat".to_string()),
        diff_match_patch::Diff::Add("map".to_string()),
    ];

    let result = dmp.diff_bisect(
        &a.chars().collect::<Vec<_>>(),
        &b.chars().collect::<Vec<_>>(),
    );

    assert_eq!(expected, result);
}

#[test]
pub fn test_diff_main() {
    let new_dmp = diff_match_patch::Dmp::new();
    let temp: Vec<diff_match_patch::Diff> = Vec::new();
    assert_eq!(temp, new_dmp.diff_main("", "", true));
    assert_eq!(
        vec![diff_match_patch::Diff::Keep("abc".to_string())],
        new_dmp.diff_main("abc", "abc", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("ab".to_string()),
            diff_match_patch::Diff::Add("123".to_string()),
            diff_match_patch::Diff::Keep("c".to_string())
        ],
        new_dmp.diff_main("abc", "ab123c", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("123".to_string()),
            diff_match_patch::Diff::Keep("bc".to_string())
        ],
        new_dmp.diff_main("a123bc", "abc", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Add("123".to_string()),
            diff_match_patch::Diff::Keep("b".to_string()),
            diff_match_patch::Diff::Add("456".to_string()),
            diff_match_patch::Diff::Keep("c".to_string())
        ],
        new_dmp.diff_main("abc", "a123b456c", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("123".to_string()),
            diff_match_patch::Diff::Keep("b".to_string()),
            diff_match_patch::Diff::Delete("456".to_string()),
            diff_match_patch::Diff::Keep("c".to_string())
        ],
        new_dmp.diff_main("a123b456c", "abc", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("a".to_string()),
            diff_match_patch::Diff::Add("b".to_string())
        ],
        new_dmp.diff_main("a", "b", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("Apple".to_string()),
            diff_match_patch::Diff::Add("Banana".to_string()),
            diff_match_patch::Diff::Keep("s are a".to_string()),
            diff_match_patch::Diff::Add("lso".to_string()),
            diff_match_patch::Diff::Keep(" fruit.".to_string())
        ],
        new_dmp.diff_main("Apples are a fruit.", "Bananas are also fruit.", true)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("a".to_string()),
            diff_match_patch::Diff::Add("\u{0680}".to_string()),
            diff_match_patch::Diff::Keep("x".to_string()),
            diff_match_patch::Diff::Delete("\t".to_string()),
            diff_match_patch::Diff::Add("\n".to_string())
        ],
        new_dmp.diff_main("ax\t", "\u{0680}x\n", false)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("1".to_string()),
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("y".to_string()),
            diff_match_patch::Diff::Keep("b".to_string()),
            diff_match_patch::Diff::Delete("2".to_string()),
            diff_match_patch::Diff::Add("xab".to_string())
        ],
        new_dmp.diff_main("1ayb2", "abxab", false)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Add("xaxcx".to_string()),
            diff_match_patch::Diff::Keep("abc".to_string()),
            diff_match_patch::Diff::Delete("y".to_string())
        ],
        new_dmp.diff_main("abcy", "xaxcxabc", false)
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Delete("ABCD".to_string()),
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Delete("=".to_string()),
            diff_match_patch::Diff::Add("-".to_string()),
            diff_match_patch::Diff::Keep("bcd".to_string()),
            diff_match_patch::Diff::Delete("=".to_string()),
            diff_match_patch::Diff::Add("-".to_string()),
            diff_match_patch::Diff::Keep("efghijklmnopqrs".to_string()),
            diff_match_patch::Diff::Delete("EFGHIJKLMNOefg".to_string())
        ],
        new_dmp.diff_main(
            "ABCDa=bcd=efghijklmnopqrsEFGHIJKLMNOefg",
            "a-bcd-efghijklmnopqrs",
            false
        )
    );
    assert_eq!(
        vec![
            diff_match_patch::Diff::Add(" ".to_string()),
            diff_match_patch::Diff::Keep("a".to_string()),
            diff_match_patch::Diff::Add("nd".to_string()),
            diff_match_patch::Diff::Keep(" [[Pennsylvania]]".to_string()),
            diff_match_patch::Diff::Delete(" and [[New".to_string())
        ],
        new_dmp.diff_main(
            "a [[Pennsylvania]] and [[New",
            " and [[Pennsylvania]]",
            false
        )
    );

    // Test the linemode speedup.
    // Must be long to pass the 100 char cutoff.
    let mut a = "1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n";
    let mut b = "abcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\n";
    assert_eq!(
        new_dmp.diff_main(a, b, true),
        new_dmp.diff_main(a, b, false)
    );

    a = "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
    b = "abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij";
    assert_eq!(
        new_dmp.diff_main(a, b, true),
        new_dmp.diff_main(a, b, false)
    );
    a = "1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n";
    b = "abcdefghij\n1234567890\n1234567890\n1234567890\nabcdefghij\n1234567890\n1234567890\n1234567890\nabcdefghij\n1234567890\n1234567890\n1234567890\nabcdefghij\n";
    let texts_linemode = diff_rebuildtexts(new_dmp.diff_main(a, b, true));
    let texts_textmode = diff_rebuildtexts(new_dmp.diff_main(a, b, false));
    assert_eq!(texts_linemode, texts_textmode);
}

#[test]
pub fn test_match_apphabet() {
    let mut dmp = diff_match_patch::Dmp::new();
    let mut s: HashMap<char, i32> = HashMap::new();
    s.insert('a', 4);
    s.insert('b', 2);
    s.insert('c', 1);
    assert_eq!(s, dmp.match_alphabet(&("abc".chars().collect::<Vec<_>>())));
    s.insert('a', 37);
    s.insert('b', 18);
    s.insert('c', 8);
    assert_eq!(
        s,
        dmp.match_alphabet(&("abcaba".chars().collect::<Vec<_>>()))
    );
}

#[test]
pub fn test_match_bitap() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.match_distance = 100;
    dmp.match_threshold = 0.5;
    assert_eq!(
        5,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("fgh".chars().collect::<Vec<_>>()),
            5
        )
    );
    assert_eq!(
        5,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("fgh".chars().collect::<Vec<_>>()),
            0
        )
    );

    // Fuzzy matches.
    assert_eq!(
        4,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("efxhi".chars().collect::<Vec<_>>()),
            0
        )
    );

    assert_eq!(
        2,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("cdefxyhijk".chars().collect::<Vec<_>>()),
            5
        )
    );

    assert_eq!(
        -1,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("bxy".chars().collect::<Vec<_>>()),
            1
        )
    );

    // Overflow.
    assert_eq!(
        2,
        dmp.match_bitap(
            &("123456789xx0".chars().collect::<Vec<_>>()),
            &("3456789x0".chars().collect::<Vec<_>>()),
            2
        )
    );

    assert_eq!(
        0,
        dmp.match_bitap(
            &("abcdef".chars().collect::<Vec<_>>()),
            &("xxabc".chars().collect::<Vec<_>>()),
            4
        )
    );

    assert_eq!(
        3,
        dmp.match_bitap(
            &("abcdef".chars().collect::<Vec<_>>()),
            &("defyy".chars().collect::<Vec<_>>()),
            4
        )
    );

    assert_eq!(
        0,
        dmp.match_bitap(
            &("abcdef".chars().collect::<Vec<_>>()),
            &("xabcdefy".chars().collect::<Vec<_>>()),
            0
        )
    );

    // Threshold test.
    dmp.match_threshold = 0.4;
    assert_eq!(
        4,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("efxyhi".chars().collect::<Vec<_>>()),
            1
        )
    );

    dmp.match_threshold = 0.3;
    assert_eq!(
        -1,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("efxyhi".chars().collect::<Vec<_>>()),
            1
        )
    );

    dmp.match_threshold = 0.0;
    assert_eq!(
        1,
        dmp.match_bitap(
            &("abcdefghijk".chars().collect::<Vec<_>>()),
            &("bcdef".chars().collect::<Vec<_>>()),
            1
        )
    );
    dmp.match_threshold = 0.5;

    // Multiple select.
    assert_eq!(
        0,
        dmp.match_bitap(
            &("abcdexyzabcde".chars().collect::<Vec<_>>()),
            &("abccde".chars().collect::<Vec<_>>()),
            3
        )
    );

    assert_eq!(
        8,
        dmp.match_bitap(
            &("abcdexyzabcde".chars().collect::<Vec<_>>()),
            &("abccde".chars().collect::<Vec<_>>()),
            5
        )
    );

    // Distance test.
    dmp.match_distance = 10;
    assert_eq!(
        -1,
        dmp.match_bitap(
            &("abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>()),
            &("abcdefg".chars().collect::<Vec<_>>()),
            24
        )
    );

    assert_eq!(
        0,
        dmp.match_bitap(
            &("abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>()),
            &("abcdxxefg".chars().collect::<Vec<_>>()),
            1
        )
    );

    dmp.match_distance = 1000;
    assert_eq!(
        0,
        dmp.match_bitap(
            &("abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>()),
            &("abcdefg".chars().collect::<Vec<_>>()),
            24
        )
    );
}

#[test]
pub fn test_match_main() {
    let mut dmp = diff_match_patch::Dmp::new();
    assert_eq!(0, dmp.match_main("abcdef", "abcdef", 1000));

    assert_eq!(-1, dmp.match_main("", "abcdef", 1));

    assert_eq!(3, dmp.match_main("abcdef", "", 3));

    assert_eq!(3, dmp.match_main("abcdef", "de", 3));

    assert_eq!(3, dmp.match_main("abcdef", "defy", 4));

    assert_eq!(0, dmp.match_main("abcdef", "abcdefy", 0));

    dmp.match_threshold = 0.7;
    assert_eq!(
        4,
        dmp.match_main(
            "I am the very model of a modern major general.",
            " that berry ",
            5
        )
    );
    dmp.match_threshold = 0.5;
}

#[test]
pub fn test_patch_obj() {
    let mut patch = diff_match_patch::Patch::new(vec![], 0, 0, 0, 0);
    patch.start1 = 20;
    patch.start2 = 21;
    patch.length1 = 18;
    patch.length2 = 17;
    patch.diffs = vec![
        diff_match_patch::Diff::Keep("jump".to_string()),
        diff_match_patch::Diff::Delete("s".to_string()),
        diff_match_patch::Diff::Add("ed".to_string()),
        diff_match_patch::Diff::Keep(" over ".to_string()),
        diff_match_patch::Diff::Delete("the".to_string()),
        diff_match_patch::Diff::Add("a".to_string()),
        diff_match_patch::Diff::Keep("\nlaz".to_string()),
    ];
    assert_eq!(
        "@@ -21,18 +22,17 @@\n jump\n-s\n+ed\n  over \n-the\n+a\n %0Alaz\n".to_string(),
        patch.to_string()
    );
}

#[test]
pub fn test_patch_from_text() {
    let mut dmp = diff_match_patch::Dmp::new();
    let diffs: Vec<diff_match_patch::Patch> = vec![];
    assert_eq!(diffs, dmp.patch_from_text("".to_string()));

    let strp = "@@ -21,18 +22,17 @@\n jump\n-s\n+ed\n  over \n-the\n+a\n %0Alaz\n".to_string();
    assert_eq!(strp, dmp.patch_from_text(strp.clone())[0].to_string());

    assert_eq!(
        "@@ -1,1 +1,1 @@\n-a\n+b\n".to_string(),
        dmp.patch_from_text("@@ -1 +1 @@\n-a\n+b\n".to_string())[0].to_string()
    );

    assert_eq!(
        "@@ -1,3 +0,0 @@\n-abc\n".to_string(),
        dmp.patch_from_text("@@ -1,3 +0,0 @@\n-abc\n".to_string())[0].to_string()
    );

    assert_eq!(
        "@@ -0,0 +1,3 @@\n+abc\n".to_string(),
        dmp.patch_from_text("@@ -0,0 +1,3 @@\n+abc\n".to_string())[0].to_string()
    );
}

#[test]
pub fn test_patch_to_text() {
    let mut dmp = diff_match_patch::Dmp::new();
    let mut strp = "@@ -21,18 +22,17 @@\n jump\n-s\n+ed\n  over \n-the\n+a\n  laz\n".to_string();
    let mut p = dmp.patch_from_text(strp.clone());
    assert_eq!(strp, dmp.patch_to_text(&mut p));

    strp = "@@ -1,9 +1,9 @@\n-f\n+F\n oo+fooba\n@@ -7,8 +7,8 @@\n obar\n-,\n+.\n tes\n".to_string();
    p = dmp.patch_from_text(strp.clone());
    assert_eq!(strp, dmp.patch_to_text(&mut p));
}

#[test]
pub fn test_patch_add_context() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.patch_margin = 4;
    let mut p =
        dmp.patch_from_text("@@ -21,4 +21,10 @@\n-jump\n+somersault\n".to_string())[0].clone();
    dmp.patch_add_context(
        &mut p,
        &mut ("The quick brown fox jumps over the lazy dog."
            .chars()
            .collect::<Vec<_>>()),
    );
    assert_eq!(
        p.to_string(),
        "@@ -17,12 +17,18 @@\n fox \n-jump\n+somersault\n s ov\n".to_string()
    );

    // Same, but not enough trailing context.
    p = dmp.patch_from_text("@@ -21,4 +21,10 @@\n-jump\n+somersault\n".to_string())[0].clone();
    dmp.patch_add_context(
        &mut p,
        &mut ("The quick brown fox jumps.".chars().collect::<Vec<_>>()),
    );
    assert_eq!(
        p.to_string(),
        "@@ -17,10 +17,16 @@\n fox \n-jump\n+somersault\n s.\n".to_string()
    );

    // Same, but not enough leading context.
    let mut p = dmp.patch_from_text("@@ -3 +3,2 @@\n-e\n+at\n".to_string())[0].clone();
    dmp.patch_add_context(
        &mut p,
        &mut ("The quick brown fox jumps.".chars().collect::<Vec<_>>()),
    );
    assert_eq!(
        p.to_string(),
        "@@ -1,7 +1,8 @@\n Th\n-e\n+at\n  qui\n".to_string()
    );

    // # Same, but with ambiguity.
    p = dmp.patch_from_text("@@ -3 +3,2 @@\n-e\n+at\n".to_string())[0].clone();
    dmp.patch_add_context(
        &mut p,
        &mut ("The quick brown fox jumps.  The quick brown fox crashes."
            .chars()
            .collect::<Vec<_>>()),
    );
    assert_eq!(
        p.to_string(),
        "@@ -1,27 +1,28 @@\n Th\n-e\n+at\n  quick brown fox jumps. \n".to_string()
    );
}

#[test]
pub fn test_patch_make() {
    let mut dmp = diff_match_patch::Dmp::new();
    // Null case.
    let mut patches = dmp.patch_make1("", "");
    assert_eq!("".to_string(), dmp.patch_to_text(&mut patches));

    let text1 = "The quick brown fox jumps over the lazy dog.";
    let text2 = "That quick brown fox jumped over a lazy dog.";
    // Text2+Text1 inputs.
    let mut expected_patch = "@@ -1,8 +1,7 @@\n Th\n-at\n+e\n  qui\n@@ -21,17 +21,18 @@\n jump\n-ed\n+s\n  over \n-a\n+the\n  laz\n".to_string();
    // The second patch must be "-21,17 +21,18", not "-22,17 +21,18" due to rolling context.
    patches = dmp.patch_make1(text2, text1);
    assert_eq!(expected_patch, dmp.patch_to_text(&mut patches));

    // Text1+Text2 inputs.
    expected_patch = "@@ -1,11 +1,12 @@\n Th\n-e\n+at\n  quick b\n@@ -22,18 +22,17 @@\n jump\n-s\n+ed\n  over \n-the\n+a\n  laz\n".to_string();
    patches = dmp.patch_make1(text1, text2);
    assert_eq!(expected_patch, dmp.patch_to_text(&mut patches));

    // Diff input.
    let mut diffs = dmp.diff_main(text1, text2, false);
    patches = dmp.patch_make2(&mut diffs);
    assert_eq!(expected_patch, dmp.patch_to_text(&mut patches));

    // Text1+Diff inputs.
    patches = dmp.patch_make4(text1, &mut diffs);
    assert_eq!(expected_patch, dmp.patch_to_text(&mut patches));

    // Text1+Text2+Diff inputs (deprecated).
    patches = dmp.patch_make3(text1, text2, &mut diffs);
    assert_eq!(expected_patch, dmp.patch_to_text(&mut patches));

    // Character encoding.
    patches = dmp.patch_make1("`1234567890-=[]\\;',./", "~!@#$%^&*()_+{}|:\"<>?");
    assert_eq!("@@ -1,21 +1,21 @@\n-%601234567890-=%5B%5D%5C;',./\n+~!@#$%25%5E&*()_+%7B%7D%7C:%22%3C%3E?\n".to_string(), dmp.patch_to_text(&mut patches));

    // Character decoding.
    diffs = vec![
        diff_match_patch::Diff::Delete("`1234567890-=[]\\;',./".to_string()),
        diff_match_patch::Diff::Add("~!@#$%^&*()_+{}|:\"<>?".to_string()),
    ];
    assert_eq!(diffs, dmp.patch_from_text("@@ -1,21 +1,21 @@\n-%601234567890-=%5B%5D%5C;',./\n+~!@#$%25%5E&*()_+%7B%7D%7C:%22%3C%3E?\n".to_string())[0].diffs);

    // Long string with repeats.
    let mut text1 = "".to_string();
    for _x in 0..100 {
        text1 += "abcdef";
    }
    let text2 = text1.clone() + "123";
    expected_patch = "@@ -573,28 +573,31 @@\n cdefabcdefabcdefabcdefabcdef\n+123\n".to_string();
    patches = dmp.patch_make1(text1.as_str(), text2.as_str());
    assert_eq!(expected_patch, dmp.patch_to_text(&mut patches));
}

#[test]
pub fn test_patch_splitmax() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.match_maxbits = 32;
    let mut patches = dmp.patch_make1(
        "abcdefghijklmnopqrstuvwxyz01234567890",
        "XabXcdXefXghXijXklXmnXopXqrXstXuvXwxXyzX01X23X45X67X89X0",
    );
    dmp.patch_splitmax(&mut patches);
    assert_eq!("@@ -1,32 +1,46 @@\n+X\n ab\n+X\n cd\n+X\n ef\n+X\n gh\n+X\n ij\n+X\n kl\n+X\n mn\n+X\n op\n+X\n qr\n+X\n st\n+X\n uv\n+X\n wx\n+X\n yz\n+X\n 012345\n@@ -25,13 +39,18 @@\n zX01\n+X\n 23\n+X\n 45\n+X\n 67\n+X\n 89\n+X\n 0\n".to_string(), dmp.patch_to_text(&mut patches));

    patches = dmp.patch_make1(
        "abcdef1234567890123456789012345678901234567890123456789012345678901234567890uvwxyz",
        "abcdefuvwxyz",
    );
    let old_totext = dmp.patch_to_text(&mut patches);
    dmp.patch_splitmax(&mut patches);
    assert_eq!(old_totext, dmp.patch_to_text(&mut patches));

    patches = dmp.patch_make1(
        "1234567890123456789012345678901234567890123456789012345678901234567890",
        "abc",
    );
    dmp.patch_splitmax(&mut patches);
    assert_eq!("@@ -1,32 +1,4 @@\n-1234567890123456789012345678\n 9012\n@@ -29,32 +1,4 @@\n-9012345678901234567890123456\n 7890\n@@ -57,14 +1,3 @@\n-78901234567890\n+abc\n", dmp.patch_to_text(&mut patches));

    patches = dmp.patch_make1(
        "abcdefghij , h : 0 , t : 1 abcdefghij , h : 0 , t : 1 abcdefghij , h : 0 , t : 1",
        "abcdefghij , h : 1 , t : 1 abcdefghij , h : 1 , t : 1 abcdefghij , h : 0 , t : 1",
    );
    dmp.patch_splitmax(&mut patches);
    assert_eq!("@@ -2,32 +2,32 @@\n bcdefghij , h : \n-0\n+1\n  , t : 1 abcdef\n@@ -29,32 +29,32 @@\n bcdefghij , h : \n-0\n+1\n  , t : 1 abcdef\n".to_string(), dmp.patch_to_text(&mut patches));
}

#[test]
pub fn test_patch_add_padding() {
    // Both edges full.
    let mut dmp = diff_match_patch::Dmp::new();
    let mut patches = dmp.patch_make1("", "test");
    assert_eq!(
        "@@ -0,0 +1,4 @@\n+test\n".to_string(),
        dmp.patch_to_text(&mut patches)
    );
    dmp.patch_add_padding(&mut patches);
    assert_eq!(
        "@@ -1,8 +1,12 @@\n %01%02%03%04\n+test\n %01%02%03%04\n".to_string(),
        dmp.patch_to_text(&mut patches)
    );

    // Both edges partial.
    patches = dmp.patch_make1("XY", "XtestY");
    assert_eq!(
        "@@ -1,2 +1,6 @@\n X\n+test\n Y\n".to_string(),
        dmp.patch_to_text(&mut patches)
    );
    dmp.patch_add_padding(&mut patches);
    assert_eq!(
        "@@ -2,8 +2,12 @@\n %02%03%04X\n+test\n Y%01%02%03\n".to_string(),
        dmp.patch_to_text(&mut patches)
    );

    // Both edges none.
    patches = dmp.patch_make1("XXXXYYYY", "XXXXtestYYYY");
    assert_eq!(
        "@@ -1,8 +1,12 @@\n XXXX\n+test\n YYYY\n".to_string(),
        dmp.patch_to_text(&mut patches)
    );
    dmp.patch_add_padding(&mut patches);
    assert_eq!(
        "@@ -5,8 +5,12 @@\n XXXX\n+test\n YYYY\n".to_string(),
        dmp.patch_to_text(&mut patches)
    );
}

#[test]
pub fn test_patch_apply() {
    let mut dmp = diff_match_patch::Dmp::new();
    dmp.match_distance = 1000;
    dmp.match_threshold = 0.5;
    dmp.patch_delete_threshold = 0.5;
    // Null case.
    let mut patches = dmp.patch_make1("", "");
    let mut results = dmp.patch_apply(&mut patches, "Hello world.");
    assert_eq!(("Hello world.".chars().collect(), vec![]), results);

    // Exact match.
    patches = dmp.patch_make1(
        "The quick brown fox jumps over the lazy dog.",
        "That quick brown fox jumped over a lazy dog.",
    );
    results = dmp.patch_apply(&mut patches, "The quick brown fox jumps over the lazy dog.");
    assert_eq!(
        (
            "That quick brown fox jumped over a lazy dog."
                .chars()
                .collect(),
            vec![true, true]
        ),
        results
    );

    // Partial match.
    results = dmp.patch_apply(
        &mut patches,
        "The quick red rabbit jumps over the tired tiger.",
    );
    assert_eq!(
        (
            "That quick red rabbit jumped over a tired tiger."
                .chars()
                .collect(),
            vec![true, true]
        ),
        results
    );

    // Failed match.
    results = dmp.patch_apply(
        &mut patches,
        "I am the very model of a modern major general.",
    );
    assert_eq!(
        (
            "I am the very model of a modern major general."
                .chars()
                .collect(),
            vec![false, false]
        ),
        results
    );

    // Big delete, small change.
    patches = dmp.patch_make1(
        "x1234567890123456789012345678901234567890123456789012345678901234567890y",
        "xabcy",
    );
    results = dmp.patch_apply(
        &mut patches,
        "x123456789012345678901234567890-----++++++++++-----123456789012345678901234567890y",
    );
    assert_eq!(("xabcy".chars().collect(), vec![true, true]), results);

    // Big delete, big change 1.
    patches = dmp.patch_make1(
        "x1234567890123456789012345678901234567890123456789012345678901234567890y",
        "xabcy",
    );
    results = dmp.patch_apply(
        &mut patches,
        "x12345678901234567890---------------++++++++++---------------12345678901234567890y",
    );
    assert_eq!(
        (
            "xabc12345678901234567890---------------++++++++++---------------12345678901234567890y"
                .chars()
                .collect(),
            vec![false, true]
        ),
        results
    );

    // Big delete, big change 2.
    dmp.patch_delete_threshold = 0.6;
    patches = dmp.patch_make1(
        "x1234567890123456789012345678901234567890123456789012345678901234567890y",
        "xabcy",
    );
    results = dmp.patch_apply(
        &mut patches,
        "x12345678901234567890---------------++++++++++---------------12345678901234567890y",
    );
    assert_eq!(("xabcy".chars().collect(), vec![true, true]), results);
    dmp.patch_delete_threshold = 0.5;

    // Compensate for failed patch.
    dmp.match_threshold = 0.0;
    dmp.match_distance = 0;
    patches = dmp.patch_make1(
        "abcdefghijklmnopqrstuvwxyz--------------------1234567890",
        "abcXXXXXXXXXXdefghijklmnopqrstuvwxyz--------------------1234567YYYYYYYYYY890",
    );
    results = dmp.patch_apply(
        &mut patches,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ--------------------1234567890",
    );
    assert_eq!(
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ--------------------1234567YYYYYYYYYY890"
                .chars()
                .collect(),
            vec![false, true]
        ),
        results
    );
    dmp.match_threshold = 0.5;
    dmp.match_distance = 1000;

    // No side effects.
    patches = dmp.patch_make1("", "test");
    let mut patchstr = dmp.patch_to_text(&mut patches);
    results = dmp.patch_apply(&mut patches, "");
    assert_eq!(patchstr, dmp.patch_to_text(&mut patches));

    // No side effects with major delete.
    patches = dmp.patch_make1("The quick brown fox jumps over the lazy dog.", "Woof");
    patchstr = dmp.patch_to_text(&mut patches);
    dmp.patch_apply(&mut patches, "The quick brown fox jumps over the lazy dog.");
    assert_eq!(patchstr, dmp.patch_to_text(&mut patches));

    // Edge exact match.
    patches = dmp.patch_make1("", "test");
    dmp.patch_apply(&mut patches, "");
    assert_eq!(("test".chars().collect(), vec![true]), results);

    // Near edge exact match.
    patches = dmp.patch_make1("XY", "XtestY");
    results = dmp.patch_apply(&mut patches, "XY");
    assert_eq!(("XtestY".chars().collect(), vec![true]), results);

    // Edge partial match.
    patches = dmp.patch_make1("y", "y123");
    results = dmp.patch_apply(&mut patches, "x");
    assert_eq!(("x123".chars().collect(), vec![true]), results);

    // Applying "delete" patch on an empty text.
    patches = dmp.patch_make1("test", "");
    results = dmp.patch_apply(&mut patches, "");
    assert_eq!(("".chars().collect(), vec![true]), results);
}
