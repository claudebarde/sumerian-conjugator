#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn ak() -> Result<(), String> {
        let stem = "ak".to_string();

        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("munak")), verb);
        // It was made with it
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_comitative(Some(Person::ThirdSingNonHuman))
            .set_preformative(Some(Preformative::A))
            .print();
        assert_eq!(Ok(String::from("abdaak")), verb);
        // He made it into it
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_preformative(Some(Preformative::I))
            .set_terminative(Some(Person::ThirdSingNonHuman))
            .print();
        assert_eq!(Ok(String::from("ibšinak")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn bala() -> Result<(), String> {
        // I cross it
        let stem = "bala".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::FirstSing)
            .set_object(Person::ThirdSingNonHuman)?
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("ibbalan")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn daḫ() -> Result<(), String> {
        let stem = "daḫ".to_string();
        // I shall add for you 17.2.4 (33)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::FirstSing)
            .set_indirect_object(Person::SecondSing)
            .set_preformative(Some(Preformative::A))
            .print();
        assert_eq!(Ok(String::from("aradaḫen")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn e() -> Result<(), String> {
        // May he say it to him! 25.4.1 (49)
        let stem = "ʔe".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_indirect_object(Person::ThirdSingHuman)
            .set_preformative(Some(Preformative::I))
            .set_modal()
            .print();

        assert_eq!(Ok(String::from("ḫēnnabʔee")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn è() -> Result<(), String> {
        // He must let it come out of him 16.2.2 (17)
        let stem = "ʔè".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(Some(IpfvStem::EdMarker))
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_ablative(Some(Person::ThirdSingHuman))
            .set_ventive()
            .set_modal()
            // .set_ed_marker(Some("d".to_string()))
            .print();

        assert_eq!(Ok(String::from("ḫamuntaʔède")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn ĝal() -> Result<(), String> {
        let stem = "ĝal".to_string();
        // This is with him 16.2.2 (14a)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_comitative(Some(Person::ThirdSingHuman))
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("indaĝal")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn ĝar() -> Result<(), String> {
        // He placed it from it 16.2.1 (7)
        let stem = "ĝar".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_ablative(Some(Person::ThirdSingNonHuman))
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("ibtanĝar")), verb);
        // They were placed on it 21.2 (7)
        // let verb = FiniteVerbalForm::from_stem(stem.clone())
        //     .is_perfective()
        //     .is_transitive()
        //     .set_subject(Person::ThirdSingNonHuman))
        //     .set_indirect_object(Some(Person::ThirdSingNonHuman))
        //     .set_middle_prefix(Some(MiddlePrefix))
        //     .print();
        // assert_eq!(Ok(String::from("baaĝar")), verb);

        // match verb {
        //     Err(err) => {
        //         panic!("Test failed: `{}` returned an error: {}", stem, err);
        //     }
        //     Ok(_) => Ok(()),
        // }

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn ĝen() -> Result<(), String> {
        let stem = "ĝen".to_string();
        // He came for him 22.6 (68a)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_terminative(Some(Person::ThirdSingHuman))
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("munšiĝen")), verb);
        // He came for it 22.6 (68b)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_terminative(Some(Person::ThirdSingNonHuman))
            .set_ventive()
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("imšiĝen")), verb);
        // He came to it 17.2.1 (4)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_indirect_object(Person::ThirdSingNonHuman)
            .set_preformative(Some(Preformative::I))
            .set_ventive()
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
    fn gi() -> Result<(), String> {
        let stem = "gi".to_string();
        // He should send him to me 16.2.5 (31)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(Some(IpfvStem::Reduplicate))
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingHuman)?
            .set_terminative(Some(Person::FirstSing))
            .set_modal()
            .print();
        assert_eq!(Ok(String::from("ḫamuʔšingi-gie")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn gub() -> Result<(), String> {
        // They were caused to stand 21.2 (6)
        let stem = "gub".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdPlurNonHuman)
            .set_indirect_object(Person::ThirdPlurHuman)
            .set_middle_prefix(Some(MiddlePrefix))
            .print();
        assert_eq!(Ok(String::from("bannēgub")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn hul() -> Result<(), String> {
        let stem = "hul".to_string();
        // He was happy about it 22.6 (67b)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_comitative(Some(Person::ThirdSingNonHuman))
            .set_preformative(Some(Preformative::I))
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("imdahul")), verb);

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
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_comitative(Some(Person::FirstSing))
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("muʔdankar")), verb);
        // He took her away from him 21.2 (4)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingHuman)?
            .set_middle_prefix(Some(MiddlePrefix))
            .set_comitative(Some(Person::ThirdSingHuman))
            .print();
        assert_eq!(Ok(String::from("bandankar")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn kuř() -> Result<(), String> {
        let stem = "kuř".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingNonHuman)
            .set_middle_prefix(Some(MiddlePrefix))
            .set_comitative(Some(Person::SecondSing))
            .print();
        assert_eq!(Ok(String::from("baadakuř")), verb);

        // He let her enter it for him 17.2.2 (15)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_indirect_object(Person::ThirdSingHuman)
            .set_ventive()
            .set_locative_in(None)
            .print();
        assert_eq!(Ok(String::from("munnaninkuř")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn lá() -> Result<(), String> {
        let stem = "lá".to_string();
        // It was weighed out for him 17.2.2 (8)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdSingNonHuman)
            .set_indirect_object(Person::ThirdSingHuman)
            .set_preformative(Some(Preformative::A))
            .print();
        assert_eq!(Ok(String::from("annalá")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn me() -> Result<(), String> {
        let stem = "me".to_string();
        // I am not
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::FirstSing)
            .set_negative()
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("nuumen")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn rig() -> Result<(), String> {
        // It was cleared 21.2 (1a)
        let stem = "rig".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingNonHuman)
            .set_middle_prefix(Some(MiddlePrefix))
            .print();
        assert_eq!(Ok(String::from("babrig")), verb);
        // It was cleared away
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingNonHuman)
            .set_middle_prefix(Some(MiddlePrefix))
            .set_ventive()
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("immabrig")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn řu() -> Result<(), String> {
        let stem = "řu".to_string();
        // He erected them in it for him 20.1 (1a)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdPlurNonHuman)?
            .set_indirect_object(Person::ThirdSingHuman)
            .set_locative_in(None)
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("munnaninřu")), verb);
        // He erected it on it 20.1 (1b)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_locative_on(Some(Person::ThirdSingNonHuman))
            .print();
        assert_eq!(Ok(String::from("binřu")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn sa() -> Result<(), String> {
        // He bought her from them 16.2.3 (18)
        let stem = "sa".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingHuman)?
            .set_preformative(Some(Preformative::I))
            .set_terminative(Some(Person::ThirdPlurHuman))
            .print();
        assert_eq!(Ok(String::from("innēšinsa")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }
    #[test]
    fn sig_sig() -> Result<(), String> {
        let stem = "sig-sig".to_string();
        // He put them in it 20.1 (2a)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_ventive()
            .set_locative_in(None)
            .print();
        assert_eq!(Ok(String::from("mininsig-sig")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn sug() -> Result<(), String> {
        let stem = "sug".to_string();
        // They stood for it 16.2.1 (2)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdPlurHuman)
            .set_indirect_object(Person::ThirdSingNonHuman)
            .print();
        assert_eq!(Ok(String::from("basugeš")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn šum() -> Result<(), String> {
        // He gave to it 17.2.1 (1)
        let stem = "šum".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_indirect_object(Person::ThirdSingNonHuman)
            .print();
        assert_eq!(Ok(String::from("banšum")), verb);
        // He gave it to it 17.2.1 (3)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_indirect_object(Person::ThirdSingNonHuman)
            .set_preformative(Some(Preformative::I))
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("immašume")), verb);

        // He gave this to him 17.2.2 (9)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_preformative(Some(Preformative::I))
            .set_indirect_object(Person::ThirdSingHuman)
            .print();
        assert_eq!(Ok(String::from("innanšum")), verb);

        // She gave it to him 17.2.2 (14)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_indirect_object(Person::ThirdSingHuman)
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("munnanšum")), verb);
        // He will give it to you 17.2.4 (36)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_indirect_object(Person::SecondSing)
            .print();
        assert_eq!(Ok(String::from("rabšume")), verb);
        // Let him give it to you 17.2.4 (37)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_indirect_object(Person::SecondSing)
            .set_modal()
            .print();
        assert_eq!(Ok(String::from("ḫarabšume")), verb);
        // I did not give her to you 17.2.4 (38)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::FirstSing)
            .set_object(Person::ThirdSingHuman)?
            .set_indirect_object(Person::SecondSing)
            .set_negative()
            .print();
        assert_eq!(Ok(String::from("nuraʔšum")), verb);

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
            .is_imperfective(None)
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_terminative(Some(Person::SecondSing))
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
    fn tuku() -> Result<(), String> {
        let stem = "tuku".to_string();
        // I had it
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::FirstSing)
            .set_object(Person::ThirdSingNonHuman)?
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("iʔtuku")), verb);
        // I have it
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(Some(IpfvStem::Other(String::from("tuktuku"))))
            .is_transitive()
            .set_subject(Person::FirstSing)
            .set_object(Person::ThirdSingNonHuman)?
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("ibtuktukun")), verb);
        // You had me
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::SecondSing)
            .set_object(Person::FirstSing)?
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("iitukun")), verb);
        // They have it
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(Some(IpfvStem::Other(String::from("tuktuku"))))
            .is_transitive()
            .set_subject(Person::ThirdPlurHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("ibtuktukuš")), verb);

        // You do not have it with me 16.2.5 (30)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::SecondSing)
            .set_object(Person::ThirdSingNonHuman)?
            .set_comitative(Some(Person::FirstSing))
            .set_negative()
            // .set_ventive()
            .print();
        assert_eq!(Ok(String::from("numuʔdaatuku")), verb);
        // He has it with us 16.2.6 (34)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_comitative(Some(Person::FirstPlur))
            .print();
        assert_eq!(Ok(String::from("mēdantuku")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn ug() -> Result<(), String> {
        // He is dying
        let stem = "ug".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_imperfective(Some(IpfvStem::EdMarker))
            .is_intransitive()
            .set_subject(Person::ThirdSingHuman)
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("uuged")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn zig() -> Result<(), String> {
        let stem = "zig".to_string();
        // They were raised from these
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_intransitive()
            .set_subject(Person::ThirdPlurNonHuman)
            .set_ablative(Some(Person::ThirdPlurNonHuman))
            .set_preformative(Some(Preformative::I))
            .print();
        assert_eq!(Ok(String::from("ibtazig")), verb); // 22.1 (1)

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }

    #[test]
    fn zu() -> Result<(), String> {
        // He truly knows 22.1 (1)
        let stem = "zu".to_string();
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("munzu")), verb);
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::ThirdSingHuman)
            .set_object(Person::ThirdSingNonHuman)?
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("munzu")), verb);
        // I have not known it from you 16.2.4 (27)
        let verb = FiniteVerbalForm::from_stem(stem.clone())
            .is_perfective()
            .is_transitive()
            .set_subject(Person::FirstSing)
            .set_object(Person::ThirdSingNonHuman)?
            .set_comitative(Some(Person::SecondSing))
            .set_negative()
            .set_ventive()
            .print();
        assert_eq!(Ok(String::from("numuudaʔzu")), verb);

        match verb {
            Err(err) => {
                panic!("Test failed: `{}` returned an error: {}", stem, err);
            }
            Ok(_) => Ok(()),
        }
    }
}
