#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tuku() {
        // I had it
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            "tuku".to_string(),
            false,
            true,
            Person::FirstSing,
            Some(Person::ThirdSingNonHuman),
            None,
            Some(Preformative::I),
            None,
            false,
            false,
            false,
            dimensional_prefixes.clone(),
        );
        assert_eq!(Ok(String::from("iʔtuku")), verb);
        // I have it
        let verb = build_verb(
            "tuku".to_string(),
            false,
            false,
            Person::FirstSing,
            Some(Person::ThirdSingNonHuman),
            None,
            Some(Preformative::I),
            None,
            false,
            false,
            false,
            dimensional_prefixes.clone(),
        );
        assert_eq!(Ok(String::from("ibtuktukun")), verb);
        // You had me
        let verb = build_verb(
            "tuku".to_string(),
            false,
            true,
            Person::SecondSing,
            Some(Person::FirstSing),
            None,
            Some(Preformative::I),
            None,
            false,
            false,
            false,
            dimensional_prefixes.clone(),
        );
        assert_eq!(Ok(String::from("iitukun")), verb);
        // They have it
        let verb = build_verb(
            "tuku".to_string(),
            false,
            false,
            Person::ThirdPlurHuman,
            Some(Person::ThirdSingNonHuman),
            None,
            Some(Preformative::I),
            None,
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("ibtuktukuš")), verb);
    }

    #[test]
    fn ak() {
        let stem = "ak".to_string();
        // He made it 22.2 (8)
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            Some(Person::ThirdSingNonHuman),
            Some(true),
            None,
            None,
            false,
            true,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("munak")), verb);
        // It was made with it
        let dimensional_prefixes = DimensionalPrefixes::with_comitative();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            None,
            Some(false),
            Some(Preformative::A),
            Some(Person::ThirdSingNonHuman),
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("abdaak")), verb);
        // He made it into it
        let dimensional_prefixes = DimensionalPrefixes::with_terminative();
        let verb = build_verb(
            stem,
            false,
            true,
            Person::ThirdSingHuman,
            Some(Person::ThirdSingNonHuman),
            Some(true),
            Some(Preformative::I),
            Some(Person::ThirdSingNonHuman),
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("ibšinak")), verb);
    }

    #[test]
    fn bala() {
        // I cross it
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            "bala".to_string(),
            false,
            false,
            Person::FirstSing,
            Some(Person::ThirdSingNonHuman),
            Some(true),
            Some(Preformative::I),
            None,
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("ibbalan")), verb);
    }

    #[test]
    fn ĝar() {
        // He placed it from it 16.2.1 (7)
        let dimensional_prefixes = DimensionalPrefixes::with_ablative();
        let stem = "ĝar".to_string();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            Some(Person::ThirdSingNonHuman),
            Some(true),
            Some(Preformative::I),
            Some(Person::ThirdSingNonHuman),
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("ibtanĝar")), verb);
    }

    #[test]
    fn ĝen() -> Result<(), String> {
        let stem = "ĝen".to_string();
        // He came for him 22.6 (68a)
        let dimensional_prefixes = DimensionalPrefixes::with_terminative();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            None,
            Some(false),
            None,
            Some(Person::ThirdSingHuman),
            false,
            true,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("munšiĝen")), verb);
        // He came for it 22.6 (68b)
        let dimensional_prefixes = DimensionalPrefixes::with_terminative();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            None,
            Some(false),
            Some(Preformative::I),
            Some(Person::ThirdSingNonHuman),
            false,
            true,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("imšiĝen")), verb);
        // He came to it 17.2.1 (4)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Some(Person::ThirdSingHuman))
            .set_indirect_object(Some(Person::ThirdSingNonHuman))
            .set_preformative(Some(Preformative::I))
            .set_ventive(Some(Ventive))
            .print();
        assert_eq!(Ok(String::from("immaĝen")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn kar() -> Result<(), String> {
        // He took it away from me 22.2 (12)
        let stem = "kar".to_string();
        let dimensional_prefixes = DimensionalPrefixes::with_comitative();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            Some(Person::ThirdSingNonHuman),
            Some(true),
            None,
            Some(Person::FirstSing),
            false,
            true,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("muʔdankar")), verb);
        // He took her away from him 21.2 (4)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Some(Person::ThirdSingHuman))
            .set_object(Some(Person::ThirdSingHuman))?
            .set_initial_person_prefix(Some(Person::ThirdSingHuman))
            .set_middle_prefix(Some(MiddlePrefix))
            .set_comitative(true)
            .print();
        assert_eq!(Ok(String::from("bandankar")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    // #[test]
    // fn gub() {
    //     // They were caused to stand 21.2 (6)
    //     let stem = "gub".to_string();
    //     let dimensional_prefixes = DimensionalPrefixes::all_false();
    //     let verb = build_verb(
    //         stem.clone(),
    //         false,
    //         true,
    //         Person::ThirdPlurNonHuman,
    //         None,
    //         Some(false),
    //         None,
    //         None,
    //         false,
    //         false,
    //         true,
    //         dimensional_prefixes,
    //     );
    //     assert_eq!(Ok(String::from("bannēgub")), verb);
    // }

    #[test]
    fn hul() {
        // He was happy about it 22.6 (67b)
        let dimensional_prefixes = DimensionalPrefixes::with_comitative();
        let verb = build_verb(
            "hul".to_string(),
            false,
            true,
            Person::ThirdSingHuman,
            None,
            Some(false),
            Some(Preformative::I),
            Some(Person::ThirdSingNonHuman),
            false,
            true,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("imdahul")), verb);
    }

    #[test]
    fn me() {
        // I am not
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            "me".to_string(),
            false,
            true,
            Person::FirstSing,
            None,
            Some(false),
            Some(Preformative::I),
            None,
            true,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("nuumen")), verb);
    }

    #[test]
    fn rig() {
        // It was cleared 21.2 (1a)
        let stem = "rig".to_string();
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingNonHuman,
            None,
            Some(true),
            None,
            None,
            false,
            false,
            true,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("babrig")), verb);
        // It was cleared away
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingNonHuman,
            None,
            Some(true),
            Some(Preformative::I),
            None,
            false,
            true,
            true,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("immabrig")), verb);
    }

    #[test]
    fn sug() {
        // They stood for it 16.2.1 (2)
        let dimensional_prefixes =
            DimensionalPrefixes::with_indirect_object(Person::ThirdSingNonHuman);
        let verb = build_verb(
            "sug".to_string(),
            false,
            true,
            Person::ThirdPlurHuman,
            None,
            Some(false),
            None,
            None,
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("basugeš")), verb);
    }

    #[test]
    fn šum() -> Result<(), String> {
        // He gave to it 17.2.1 (1)
        let stem = "šum".to_string();
        let dimensional_prefixes =
            DimensionalPrefixes::with_indirect_object(Person::ThirdSingNonHuman);
        let verb = build_verb(
            stem.clone(),
            false,
            true,
            Person::ThirdSingHuman,
            None,
            Some(true),
            None,
            None,
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("banšum")), verb);
        // He gave it to it 17.2.1 (3)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective()
            .is_transitive()
            .set_subject(Some(Person::ThirdSingHuman))
            .set_indirect_object(Some(Person::ThirdSingNonHuman))
            .set_preformative(Some(Preformative::I))
            .set_ventive(Some(Ventive))
            .print();
        assert_eq!(Ok(String::from("immašume")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn ti() -> Result<(), String> {
        let stem = "ti".to_string();
        // He will let it approach towards you 17.2.1 (6)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective()
            .is_transitive()
            .set_subject(Some(Person::ThirdSingHuman))
            .set_object(Some(Person::ThirdSingNonHuman))?
            .set_terminative()
            .set_initial_person_prefix(Some(Person::SecondSing))
            .set_middle_prefix(Some(MiddlePrefix))
            .print();
        assert_eq!(Ok(String::from("baašibtie")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn ug() {
        // He is dying
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            "ug".to_string(),
            false,
            false,
            Person::ThirdSingHuman,
            None,
            Some(false),
            Some(Preformative::I),
            None,
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("uuged")), verb);
    }

    #[test]
    fn zig() {
        // They were raised from these
        let dimensional_prefixes = DimensionalPrefixes::with_ablative();
        let verb = build_verb(
            "zig".to_string(),
            false,
            true,
            Person::ThirdPlurNonHuman,
            None,
            Some(false),
            Some(Preformative::I),
            Some(Person::ThirdPlurNonHuman),
            false,
            false,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("ibtazig")), verb); // 22.1 (1)
    }

    #[test]
    fn zu() {
        // He truly knows 22.1 (1)
        let dimensional_prefixes = DimensionalPrefixes::all_false();
        let verb = build_verb(
            "zu".to_string(),
            false,
            true,
            Person::ThirdSingHuman,
            Some(Person::ThirdSingNonHuman),
            Some(true),
            None,
            None,
            false,
            true,
            false,
            dimensional_prefixes,
        );
        assert_eq!(Ok(String::from("munzu")), verb);
    }
}
