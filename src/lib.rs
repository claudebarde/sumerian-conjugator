mod lexicon;
mod tests;
mod utils;
use crate::lexicon::{find_verb, VerbAspect, VerbForm};
use crate::utils::starts_with_consonant;

/*
    Slot 1 Modal prefix (􀈏a), negative particle, prefix of anteriority, stem (in imperative forms)
    Slot 2 Finite-marker prefix, modal prefixes (all the other)
    Slot 3 Coordinator prefix
    Slot 4 Ventive (cislocative) prefix
    Slot 5 Middle prefix or the 3.SG.NH pronominal prefix /b/ (specifying the person,
    gender and number of the first in the sequence of adverbial prefixes)
    Slot 6 Initial Pronominal Prefix (= IPP) (specifying the person, gender and number
    of the first in the sequence of adverbial prefixes)
    Slot 7 Adverbial I: dative prefix
    Slot 8 Adverbial II: comitative prefix
    Slot 9 Adverbial III: ablative or terminative prefix
    Slot 10 Adverbial IV: locative1, locative2, or locative3 prefix
    Slot 11 Final Pronominal Prefix (= FPP) (referring to A or P, depending on the tense,
    or locative3)
    Slot 12 stem
    Slot 13 present-future marker (in intransitive verbs)
    Slot 14 pronominal suffix (referring A, S, or P depending on the tense)
    Slot 15 subordinator
*/

#[derive(Debug, Clone)]
pub struct FiniteVerbalForm {
    pub is_perfective: bool,
    pub is_transitive: bool,
    pub slot_1: Option<FirstPrefix>,
    pub slot_2: Option<Preformative>, // finite marker prefix
    pub slot_3: Option<Coordinator>,
    pub slot_4: Option<Ventive>,
    pub slot_5: Option<MiddlePrefix>,
    pub slot_6: Option<InitialPersonPrefix>,
    pub slot_7: Option<IndirectObjectPrefix>,
    pub slot_8: Option<ComitativePrefix>,
    pub slot_9: Option<AdverbialPrefix>,
    pub slot_10: Option<LocativePrefix>,
    pub slot_11: Option<FinalPersonPrefix>,
    pub slot_12: Stem,
    pub slot_13: Option<String>, // ed_marker
    pub slot_14: Option<FinalPersonSuffix>,
    pub slot_15: bool,
}
impl FiniteVerbalForm {
    pub fn new(stem: String, is_perfective: bool, is_transitive: bool) -> Self {
        FiniteVerbalForm {
            is_perfective,
            is_transitive,
            slot_1: None,
            slot_2: None,
            slot_3: None,
            slot_4: None,
            slot_5: None,
            slot_6: None,
            slot_7: None,
            slot_8: None,
            slot_9: None,
            slot_10: None,
            slot_11: None,
            slot_12: stem.clone(),
            slot_13: None,
            slot_14: None,
            slot_15: false,
        }
    }

    pub fn from_stem(stem: String) -> Self {
        FiniteVerbalForm {
            is_perfective: false,
            is_transitive: false,
            slot_1: None,
            slot_2: None,
            slot_3: None,
            slot_4: None,
            slot_5: None,
            slot_6: None,
            slot_7: None,
            slot_8: None,
            slot_9: None,
            slot_10: None,
            slot_11: None,
            slot_12: stem.clone(),
            slot_13: None,
            slot_14: None,
            slot_15: false,
        }
    }
    pub fn is_transitive(&mut self) -> &mut Self {
        self.is_transitive = true;
        self
    }
    pub fn is_intransitive(&mut self) -> &mut Self {
        self.is_transitive = false;
        self
    }
    pub fn is_perfective(&mut self) -> &mut Self {
        self.is_perfective = true;
        self
    }
    pub fn is_imperfective(&mut self) -> &mut Self {
        self.is_perfective = false;
        self
    }
    pub fn set_first_prefix(&mut self, prefix: Option<FirstPrefix>) -> &mut Self {
        self.slot_1 = prefix;
        self
    }
    pub fn set_preformative(&mut self, preformative: Option<Preformative>) -> &mut Self {
        self.slot_2 = preformative;
        self
    }
    pub fn set_coordinator(&mut self, coordinator: Option<Coordinator>) -> &mut Self {
        self.slot_3 = coordinator;
        self
    }
    pub fn set_ventive(&mut self, ventive: Option<Ventive>) -> &mut Self {
        self.slot_4 = ventive;
        self
    }
    pub fn set_terminative(&mut self) -> &mut Self {
        self.slot_9 = Some(AdverbialPrefix::Terminative);
        self
    }
    pub fn set_ablative(&mut self) -> &mut Self {
        self.slot_9 = Some(AdverbialPrefix::Ablative);
        self
    }
    pub fn set_middle_prefix(&mut self, middle_prefix: Option<MiddlePrefix>) -> &mut Self {
        self.slot_5 = middle_prefix;
        self
    }
    pub fn set_initial_person_prefix(
        &mut self,
        initial_person_prefix: Option<Person>,
    ) -> &mut Self {
        match initial_person_prefix {
            Some(prefix) => match prefix {
                Person::FirstSing => self.slot_6 = Some(InitialPersonPrefix::FirstSing),
                Person::SecondSing => self.slot_6 = Some(InitialPersonPrefix::SecondSing),
                Person::ThirdSingHuman => self.slot_6 = Some(InitialPersonPrefix::ThirdSingHuman),
                Person::ThirdSingNonHuman => {
                    self.slot_6 = Some(InitialPersonPrefix::ThirdSingNonHuman)
                }
                Person::FirstPlur => self.slot_6 = Some(InitialPersonPrefix::FirstPlur),
                Person::SecondPlur => self.slot_6 = Some(InitialPersonPrefix::SecondPlur),
                Person::ThirdPlurHuman => self.slot_6 = Some(InitialPersonPrefix::ThirdPlurHuman),
                Person::ThirdPlurNonHuman => {
                    self.slot_6 = Some(InitialPersonPrefix::ThirdPlurNonHuman)
                }
            },
            None => self.slot_6 = None,
        }
        self
    }
    pub fn set_indirect_object(&mut self, dative_prefix: Option<Person>) -> &mut Self {
        match dative_prefix {
            Some(prefix) => match prefix {
                Person::FirstSing => self.slot_7 = Some(IndirectObjectPrefix::FirstSing),
                Person::SecondSing => self.slot_7 = Some(IndirectObjectPrefix::SecondSing),
                Person::ThirdSingHuman => self.slot_7 = Some(IndirectObjectPrefix::ThirdSingHuman),
                Person::ThirdSingNonHuman => {
                    self.slot_7 = Some(IndirectObjectPrefix::ThirdSingNonHuman)
                }
                Person::FirstPlur => self.slot_7 = Some(IndirectObjectPrefix::FirstPlur),
                Person::SecondPlur => self.slot_7 = Some(IndirectObjectPrefix::SecondPlur),
                Person::ThirdPlurHuman => self.slot_7 = Some(IndirectObjectPrefix::ThirdPlurHuman),
                Person::ThirdPlurNonHuman => {
                    self.slot_7 = Some(IndirectObjectPrefix::ThirdPlurNonHuman)
                }
            },
            None => self.slot_7 = None,
        }
        self
    }
    pub fn set_comitative(&mut self, comitative_prefix: bool) -> &mut Self {
        self.slot_8 = if comitative_prefix {
            Some(ComitativePrefix)
        } else {
            None
        };
        self
    }
    pub fn set_adverbial_prefix(&mut self, adverbial_prefix: Option<AdverbialPrefix>) -> &mut Self {
        self.slot_9 = adverbial_prefix;
        self
    }
    pub fn set_locative_prefix(&mut self, locative_prefix: Option<LocativePrefix>) -> &mut Self {
        self.slot_10 = locative_prefix;
        self
    }
    pub fn set_final_person_prefix(&mut self, final_person_prefix: Option<Person>) -> &mut Self {
        match final_person_prefix {
            Some(prefix) => match prefix {
                Person::FirstSing => self.slot_11 = Some(FinalPersonPrefix::FirstSingHuman),
                Person::SecondSing => self.slot_11 = Some(FinalPersonPrefix::SecondSingHuman),
                Person::ThirdSingHuman => self.slot_11 = Some(FinalPersonPrefix::ThirdSingHuman),
                Person::ThirdSingNonHuman => {
                    self.slot_11 = Some(FinalPersonPrefix::ThirdSingNonHuman)
                }
                Person::FirstPlur => self.slot_11 = Some(FinalPersonPrefix::FirstSingHuman),
                Person::SecondPlur => self.slot_11 = Some(FinalPersonPrefix::SecondSingHuman),
                Person::ThirdPlurHuman => self.slot_11 = Some(FinalPersonPrefix::ThirdSingHuman),
                Person::ThirdPlurNonHuman => {
                    self.slot_11 = Some(FinalPersonPrefix::ThirdSingNonHuman)
                }
            },
            None => self.slot_11 = None,
        }
        self
    }
    pub fn set_ed_marker(&mut self, ed_marker: Option<String>) -> &mut Self {
        self.slot_13 = ed_marker;
        self
    }
    pub fn set_final_person_suffix(&mut self, final_person_suffix: Option<Person>) -> &mut Self {
        match final_person_suffix {
            Some(prefix) => match prefix {
                Person::FirstSing => self.slot_14 = Some(FinalPersonSuffix::FirstSingHuman),
                Person::SecondSing => self.slot_14 = Some(FinalPersonSuffix::SecondSingHuman),
                Person::ThirdSingHuman => self.slot_14 = Some(FinalPersonSuffix::ThirdSingHuman),
                Person::ThirdSingNonHuman => {
                    self.slot_14 = Some(FinalPersonSuffix::ThirdSingNonHuman)
                }
                Person::FirstPlur => self.slot_14 = Some(FinalPersonSuffix::FirstPlurHuman),
                Person::SecondPlur => self.slot_14 = Some(FinalPersonSuffix::SecondPlurHuman),
                Person::ThirdPlurHuman => self.slot_14 = Some(FinalPersonSuffix::ThirdPlurHuman),
                Person::ThirdPlurNonHuman => {
                    self.slot_14 = Some(FinalPersonSuffix::ThirdPlurNonHuman)
                }
            },
            None => self.slot_14 = None,
        }
        self
    }
    pub fn set_subordinator(&mut self, subordinator: bool) -> &mut Self {
        self.slot_15 = subordinator;
        self
    }
    pub fn set_subject(&mut self, subject: Option<Person>) -> &mut Self {
        if !self.is_transitive || (self.is_transitive && !self.is_perfective) {
            self.slot_14 = match subject {
                Some(person) => match person {
                    Person::FirstSing => Some(FinalPersonSuffix::FirstSingHuman),
                    Person::SecondSing => Some(FinalPersonSuffix::SecondSingHuman),
                    Person::ThirdSingHuman => Some(FinalPersonSuffix::ThirdSingHuman),
                    Person::ThirdSingNonHuman => Some(FinalPersonSuffix::ThirdSingNonHuman),
                    Person::FirstPlur => Some(FinalPersonSuffix::FirstSingHuman),
                    Person::SecondPlur => Some(FinalPersonSuffix::SecondSingHuman),
                    Person::ThirdPlurHuman => Some(FinalPersonSuffix::ThirdSingHuman),
                    Person::ThirdPlurNonHuman => Some(FinalPersonSuffix::ThirdSingNonHuman),
                },
                None => None,
            };
        } else {
            self.slot_11 = match subject {
                Some(person) => match person {
                    Person::FirstSing => Some(FinalPersonPrefix::FirstSingHuman),
                    Person::SecondSing => Some(FinalPersonPrefix::SecondSingHuman),
                    Person::ThirdSingHuman => Some(FinalPersonPrefix::ThirdSingHuman),
                    Person::ThirdSingNonHuman => Some(FinalPersonPrefix::ThirdSingNonHuman),
                    _ => todo!("subject not implemented for plurals"),
                },
                None => None,
            };
        }
        self
    }
    pub fn set_object(&mut self, object: Option<Person>) -> Result<&mut Self, String> {
        if self.is_transitive && self.is_perfective {
            self.slot_14 = match object {
                Some(person) => match person {
                    Person::FirstSing => Some(FinalPersonSuffix::FirstSingHuman),
                    Person::SecondSing => Some(FinalPersonSuffix::SecondSingHuman),
                    Person::ThirdSingHuman => Some(FinalPersonSuffix::ThirdSingHuman),
                    Person::ThirdSingNonHuman => Some(FinalPersonSuffix::ThirdSingNonHuman),
                    Person::FirstPlur => Some(FinalPersonSuffix::FirstSingHuman),
                    Person::SecondPlur => Some(FinalPersonSuffix::SecondSingHuman),
                    Person::ThirdPlurHuman => Some(FinalPersonSuffix::ThirdSingHuman),
                    Person::ThirdPlurNonHuman => Some(FinalPersonSuffix::ThirdSingNonHuman),
                },
                None => None,
            };
        } else if self.is_transitive && !self.is_perfective {
            self.slot_11 = match object {
                Some(person) => match person {
                    Person::FirstSing => Some(FinalPersonPrefix::FirstSingHuman),
                    Person::SecondSing => Some(FinalPersonPrefix::SecondSingHuman),
                    Person::ThirdSingHuman => Some(FinalPersonPrefix::ThirdSingHuman),
                    Person::ThirdSingNonHuman => Some(FinalPersonPrefix::ThirdSingNonHuman),
                    _ => todo!("object not implemented for plurals"),
                },
                None => None,
            };
        } else {
            return Err("Cannot set object for intransitive verb".to_string());
        }
        Ok(self)
    }

    pub fn print_subordinator(&self) -> String {
        if self.slot_15 {
            return "a".to_string();
        }
        "".to_string()
    }

    pub fn print(&mut self) -> Result<String, String> {
        let final_verb: &mut [String; 15] = &mut core::array::from_fn(|_| String::new());
        // 1- Populates the different slots with the respective morphems

        // STEM
        if self.slot_12.is_empty() {
            return Err("Stem cannot be empty".to_string());
        } else {
            final_verb.add_stem(self.slot_12.clone());
        }
        // SUFFIXES
        match self.slot_13.clone() {
            Some(marker) => final_verb.add_ed_marker(marker),
            None => (),
        };
        let has_final_ps_suffix = match self.clone().slot_14 {
            Some(suffix) => {
                final_verb.add_final_ps_suffix(suffix.output(&self));
                true
            }
            None => false,
        };

        // PREFIXES
        // NEGATIVE PREFIX
        match self.clone().slot_1 {
            Some(prefix) => match prefix {
                FirstPrefix::Negative => final_verb.add_negative_prefix(),
                _ => todo!(),
            },
            None => (),
        };
        // PREFORMATIVE MARKER
        let has_preformative = match self.slot_2.clone() {
            Some(preformative) => {
                final_verb.add_preformative_prefix(preformative.output());
                true
            }
            None => false,
        };
        // VENTIVE
        let has_ventive = if self.clone().slot_4.is_some() {
            final_verb.add_ventive("mu".to_string());
            true
        } else {
            false
        };
        // MIDDLE PREFIX
        let has_middle_prefix = if self.clone().slot_5.is_some() {
            final_verb.add_middle_prefix("ba".to_string());
            true
        } else {
            false
        };
        // INITIAL PERSON PREFIX
        let has_initial_person_prefix = match self.clone().slot_6 {
            Some(prefix) => {
                let prefix_output = match prefix {
                    InitialPersonPrefix::FirstSing => "ʔ".to_string(),
                    InitialPersonPrefix::SecondSing => "e".to_string(),
                    InitialPersonPrefix::ThirdSingHuman => "n".to_string(),
                    InitialPersonPrefix::ThirdSingNonHuman => "b".to_string(),
                    InitialPersonPrefix::FirstPlur => "mē".to_string(),
                    InitialPersonPrefix::SecondPlur => "enē".to_string(),
                    InitialPersonPrefix::ThirdPlurHuman => "nnē".to_string(),
                    InitialPersonPrefix::ThirdPlurNonHuman => "b".to_string(),
                };
                final_verb.add_initial_person_prefix(prefix_output);
                true
            }
            None => false,
        };
        // INDIRECT OBJECT MARKER
        let has_indirect_object = match self.clone().slot_7 {
            Some(prefix) => {
                let prefix_output = match prefix {
                    IndirectObjectPrefix::FirstSing => "ma".to_string(),
                    IndirectObjectPrefix::SecondSing => "ra".to_string(),
                    IndirectObjectPrefix::ThirdSingHuman => "nna".to_string(),
                    IndirectObjectPrefix::ThirdSingNonHuman => "ba".to_string(),
                    IndirectObjectPrefix::FirstPlur => "mē".to_string(),
                    IndirectObjectPrefix::SecondPlur => "ra".to_string(),
                    IndirectObjectPrefix::ThirdPlurHuman => "nnē".to_string(),
                    IndirectObjectPrefix::ThirdPlurNonHuman => "ba".to_string(),
                };
                final_verb.add_indirect_object(prefix_output);
                true
            }
            None => false,
        };
        // COMITATIVE
        if self.clone().slot_8.is_some() {
            final_verb.add_comitative("da".to_string());
        }
        match self.clone().slot_9 {
            // ABLATIVE
            Some(AdverbialPrefix::Ablative) => {
                final_verb.add_ablative();
            }
            // TERMINATIVE
            Some(AdverbialPrefix::Terminative) => {
                final_verb.add_terminative();
            }
            _ => (),
        }
        // FINAL PERSON PREFIX
        match self.clone().slot_11 {
            Some(prefix) => {
                let prefix_output = match prefix {
                    FinalPersonPrefix::SecondSingHuman => {
                        // 13.2.4 The prefix {e} contracts with a preceding vowel, lengthening that vowel
                        match final_verb.find_previous_morphem(10) {
                            Some(morphem) => {
                                if morphem.ends_with("a") {
                                    String::from("a")
                                } else if morphem.ends_with("i") {
                                    String::from("i")
                                } else if morphem.ends_with("u") {
                                    String::from("u")
                                } else {
                                    prefix.output()
                                }
                            }
                            None => prefix.output(),
                        }
                    }
                    _ => prefix.output(),
                };
                final_verb.add_final_ps_prefix(prefix_output);
            }
            None => (),
        };

        // 2- Updates the morphems according to their phonologic environments
        if has_ventive && has_middle_prefix {
            // 21.2 Only after the ventive prefix (§22.2), {ba} has a slightly different form,
            // because the /b/ of {ba} assimilates to the preceding /m/ of the ventive.
            final_verb.add_ventive(String::from("m"));
            final_verb.add_middle_prefix("ma".to_string());
        }
        if has_ventive && has_indirect_object {
            // 17.2.1 After the ventive prefix (§22.2), the prefix {ba} has a slighly different form,
            // because the /b/ of {ba} assimilates to the preceding /m/ of the ventive:
            match self.slot_7.clone() {
                Some(prefix) => {
                    match prefix {
                        IndirectObjectPrefix::ThirdSingNonHuman
                        | IndirectObjectPrefix::ThirdPlurNonHuman => {
                            final_verb.add_ventive(String::from("m"));
                            final_verb.add_indirect_object(String::from("ma"));
                        }
                        _ => (),
                    };
                }
                None => (),
            }
        }

        if has_preformative {
            // TODO: 24.3.1 they are never found before a prefix with the shape /CV/.
            // Instead of a vocalic prefix we then find zero, that is, no preformative at all.
            // 24.3.2 The prefix {ʔi} may also contract with the verbal stem,
            // if the latter has an initial glottal stop.
            match final_verb.find_previous_morphem(1) {
                // looks for previous morphem, if any
                Some(morphem) => {
                    if morphem.ends_with("u") {
                        final_verb.add_preformative_prefix("u".to_string());
                    }
                }
                None => {
                    // if no previous morphem, looks for the next morphem
                    // and checks if it is the stem
                    match final_verb.find_following_morphem(2) {
                        // looks for previous morphem, if any
                        Some((morphem, morphem_name)) => {
                            if morphem.starts_with("u") && morphem_name == MarkerName::Stem {
                                final_verb.add_preformative_prefix("u".to_string());
                            }
                        }
                        None => (),
                    }
                }
            }
        }

        if has_final_ps_suffix {
            // 14.1 First, the /e/ contracts with a preceding vowel.
            // Secondly, the /e/ may assimilate to a stem vowel /u/ or /i/.
            //TODO: Finally, the /e/ may be reduced in forms with the nominalizing suffix {ʔa}.
            match final_verb.find_previous_morphem(12) {
                Some(morphem) => {
                    match final_verb.find_final_ps_suffix() {
                        None => (),
                        Some(suffix) => {
                            // current suffix in the verbal form
                            // FIXME: leaving "e" untouched for now
                            if suffix.len() > 1
                                && (morphem.ends_with("a")
                                    || morphem.ends_with("i")
                                    || morphem.ends_with("u")
                                    || morphem.ends_with("e"))
                            {
                                let mut truncated_suffix = suffix;
                                truncated_suffix.remove(0);
                                final_verb.add_final_ps_suffix(format!("{}", truncated_suffix));
                            }
                        }
                    }
                }
                None => (),
            }
        }

        if has_initial_person_prefix {
            match self.slot_6.clone() {
                None => (),
                Some(prefix) => {
                    match prefix {
                        // First, the prefix {b} cannot occur between the ventive prefix and a consonant (see §22.4).
                        // Second, between the form /m/ of the ventive and a vowel, the prefix {b} assimilates to the /m/.
                        InitialPersonPrefix::ThirdSingNonHuman => {
                            match self.slot_4.clone() {
                                None => (),
                                Some(_) => {
                                    // has ventive, checks the marker
                                    match self.slot_7.clone() {
                                        Some(marker) => {
                                            // has a dative marker
                                        }
                                        None => match self.slot_8.clone() {
                                            Some(_) => {
                                                // has a comitative marker
                                                final_verb
                                                    .add_initial_person_prefix("".to_string());
                                                final_verb.add_ventive(String::from("m"));
                                            }
                                            None => match self.slot_9.clone() {
                                                Some(_) => {
                                                    // has an adverbial marker
                                                    final_verb
                                                        .add_initial_person_prefix("".to_string());
                                                    final_verb.add_ventive(String::from("m"));
                                                }
                                                None => match self.slot_10.clone() {
                                                    Some(marker) => {
                                                        // has a locative marker
                                                        todo!("remove initial person prefix with locative marker and ventive")
                                                    }
                                                    None => (),
                                                },
                                            },
                                        },
                                    }
                                }
                            }
                        }
                        InitialPersonPrefix::SecondSing => {
                            // 16.2.4 The prefix {e} contracts with a preceding vowel, lengthening that vowel.
                            match final_verb.find_previous_morphem(5) {
                                Some(morphem) => {
                                    if morphem.ends_with("a") {
                                        final_verb.add_initial_person_prefix(String::from("a"))
                                    } else if morphem.ends_with("i") {
                                        final_verb.add_initial_person_prefix(String::from("i"))
                                    } else if morphem.ends_with("u") {
                                        final_verb.add_initial_person_prefix(String::from("u"))
                                    } else {
                                        ()
                                    }
                                }
                                None => (),
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(final_verb.print())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarkerName {
    FirstPrefix,
    Preformative,
    Coordinator,
    Ventive,
    MiddlePrefix,
    InitialPronominalPrefix,
    DativePrefix,
    ComitativePrefix,
    AdverbialPrefix,
    LocativePrefix,
    FinalPersonPrefix,
    Stem,
    EdMarker, // ed_marker
    FinalPersonSuffix,
    Subordinator,
}

// pub const FINAL_VERB: [&str; 15] = ["", "", "", "", "", "", "", "", "", "", "", "", "", "", ""];
pub trait FinalVerbImpl {
    fn add_stem(&mut self, new_stem: String);
    fn add_ed_marker(&mut self, marker: String);
    fn add_final_ps_suffix(&mut self, suffix: String);
    fn add_subordinator(&mut self, subordinator: String);
    fn add_preformative_prefix(&mut self, preformative: String);
    fn add_final_ps_prefix(&mut self, prefix: String);
    fn add_negative_prefix(&mut self);
    fn add_ventive(&mut self, ventive: String);
    fn add_middle_prefix(&mut self, middle_prefix: String);
    fn add_initial_person_prefix(&mut self, prefix: String);
    fn add_indirect_object(&mut self, prefix: String);
    fn add_comitative(&mut self, comitative: String);
    fn add_terminative(&mut self);
    fn add_ablative(&mut self);
    fn find_previous_morphem(&self, starting_slot: usize) -> Option<String>;
    fn find_following_morphem(&self, starting_slot: usize) -> Option<(String, MarkerName)>;
    fn find_final_ps_suffix(&self) -> Option<String>;
    fn name_by_position(position: usize) -> Result<MarkerName, ()>;

    fn print(&mut self) -> String;
}

impl FinalVerbImpl for [String; 15] {
    fn add_stem(&mut self, new_stem: String) {
        self[11] = new_stem;
    }
    fn add_ed_marker(&mut self, marker: String) {
        self[12] = marker;
    }
    fn add_final_ps_suffix(&mut self, suffix: String) {
        self[13] = suffix;
    }
    fn add_subordinator(&mut self, subordinator: String) {
        self[14] = subordinator;
    }
    fn add_preformative_prefix(&mut self, preformative: String) {
        self[1] = preformative;
    }
    fn add_final_ps_prefix(&mut self, prefix: String) {
        self[10] = prefix;
    }
    fn add_negative_prefix(&mut self) {
        self[0] = "nu".to_string();
    }
    fn add_ventive(&mut self, ventive: String) {
        self[3] = ventive;
    }
    fn add_middle_prefix(&mut self, middle_prefix: String) {
        self[4] = middle_prefix;
    }
    fn add_initial_person_prefix(&mut self, prefix: String) {
        self[5] = prefix;
    }
    fn add_indirect_object(&mut self, prefix: String) {
        self[6] = prefix;
    }
    fn add_comitative(&mut self, comitative: String) {
        self[7] = comitative;
    }
    fn add_terminative(&mut self) {
        self[8] = "ši".to_string();
    }
    fn add_ablative(&mut self) {
        self[8] = "ta".to_string();
    }
    fn find_previous_morphem(&self, starting_slot: usize) -> Option<String> {
        for i in (0..starting_slot).rev() {
            if !self[i].is_empty() {
                return Some(self[i].clone());
            }
        }
        None
    }
    fn find_following_morphem(&self, starting_slot: usize) -> Option<(String, MarkerName)> {
        for i in starting_slot..self.len() {
            if !self[i].is_empty() {
                match Self::name_by_position(i) {
                    Ok(name) => return Some((self[i].clone(), name)),
                    Err(_) => (),
                }
            }
        }
        None
    }
    fn find_final_ps_suffix(&self) -> Option<String> {
        let suffix = self[13].clone();
        if suffix.is_empty() {
            return None;
        }
        return Some(suffix);
    }
    fn name_by_position(position: usize) -> Result<MarkerName, ()> {
        match position {
            0 => Ok(MarkerName::FirstPrefix),
            1 => Ok(MarkerName::Preformative),
            2 => Ok(MarkerName::Coordinator),
            3 => Ok(MarkerName::Ventive),
            4 => Ok(MarkerName::MiddlePrefix),
            5 => Ok(MarkerName::InitialPronominalPrefix),
            6 => Ok(MarkerName::DativePrefix),
            7 => Ok(MarkerName::ComitativePrefix),
            8 => Ok(MarkerName::AdverbialPrefix),
            9 => Ok(MarkerName::LocativePrefix),
            10 => Ok(MarkerName::FinalPersonPrefix),
            11 => Ok(MarkerName::Stem),
            12 => Ok(MarkerName::EdMarker),
            13 => Ok(MarkerName::FinalPersonSuffix),
            14 => Ok(MarkerName::Subordinator),
            _ => Err(()),
        }
    }

    fn print(&mut self) -> String {
        format!("{}", self.join(""))
    }
}

type Stem = String;

#[derive(Debug, Clone)]
pub enum Preformative {
    A,
    I,
    U,
}
impl Preformative {
    pub fn output(&self) -> String {
        match self {
            Preformative::A => "a".to_string(),
            Preformative::I => "i".to_string(),
            Preformative::U => "u".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IndirectObjectPrefix {
    FirstSing,
    SecondSing,
    ThirdSingHuman,
    ThirdSingNonHuman,
    FirstPlur,
    SecondPlur,
    ThirdPlurHuman,
    ThirdPlurNonHuman,
}

#[derive(Debug, Clone)]
pub enum InitialPersonPrefix {
    FirstSing,
    SecondSing,
    ThirdSingHuman,
    ThirdSingNonHuman,
    FirstPlur,
    SecondPlur,
    ThirdPlurHuman,
    ThirdPlurNonHuman,
}

#[derive(Debug, Clone)]
pub enum FinalPersonPrefix {
    FirstSingHuman,
    SecondSingHuman,
    ThirdSingHuman,
    ThirdSingNonHuman,
}
impl FinalPersonPrefix {
    fn output(&self) -> String {
        match self {
            FinalPersonPrefix::FirstSingHuman => "ʔ".to_string(),
            FinalPersonPrefix::SecondSingHuman => "e".to_string(),
            FinalPersonPrefix::ThirdSingHuman => "n".to_string(),
            FinalPersonPrefix::ThirdSingNonHuman => "b".to_string(),
        }
    }
}

// a person suffix is the only one that is basically always present.
#[derive(Debug, Clone)]
pub enum FinalPersonSuffix {
    FirstSingHuman,
    SecondSingHuman,
    ThirdSingHuman,
    ThirdSingNonHuman,
    FirstPlurHuman,
    SecondPlurHuman,
    ThirdPlurHuman,
    ThirdPlurNonHuman,
}
impl FinalPersonSuffix {
    fn output(&self, verb: &FiniteVerbalForm) -> String {
        match self {
            FinalPersonSuffix::FirstSingHuman => "en".to_string(),
            FinalPersonSuffix::SecondSingHuman => "en".to_string(),
            FinalPersonSuffix::ThirdSingHuman | FinalPersonSuffix::ThirdSingNonHuman => {
                // can be Ø or "e"
                if verb.is_transitive && !verb.is_perfective {
                    return "e".to_string();
                } else {
                    return "".to_string();
                }
            }
            FinalPersonSuffix::FirstPlurHuman => "enden".to_string(),
            FinalPersonSuffix::SecondPlurHuman => "enzen".to_string(),
            FinalPersonSuffix::ThirdPlurHuman => {
                // can be "eš" or "enē"
                // return "enē".to_string();
                return "eš".to_string();
            }
            FinalPersonSuffix::ThirdPlurNonHuman => {
                return "".to_string(); // 15.3.3
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FirstPrefix {
    Negative, // nu
    Modal,    // ha
}

#[derive(Debug, Clone)]
pub struct Coordinator;

#[derive(Debug, Clone)]
pub struct Ventive;

#[derive(Debug, Clone)]
pub struct DativePrefix;

#[derive(Debug, Clone)]
pub struct ComitativePrefix;

#[derive(Debug, Clone)]
pub enum AdverbialPrefix {
    Ablative,
    Terminative,
}

#[derive(Debug, Clone)]
pub struct LocativePrefix;

#[derive(Debug, Clone)]
pub struct MiddlePrefix;

/*
    ARGUMENTS FOR VERB CONSTRUCTION
*/
#[derive(Debug, Clone)]
pub enum Person {
    FirstSing,
    SecondSing,
    ThirdSingHuman,
    ThirdSingNonHuman,
    FirstPlur,
    SecondPlur,
    ThirdPlurHuman,
    ThirdPlurNonHuman,
}

#[derive(Debug, Clone)]
pub struct DimensionalPrefixes {
    pub indirect_object: Option<Person>,
    pub comitative: bool,
    pub locative: bool,
    pub ablative: bool,
    pub terminative: bool,
}
impl DimensionalPrefixes {
    pub fn all_false() -> Self {
        DimensionalPrefixes {
            indirect_object: None,
            comitative: false,
            locative: false,
            ablative: false,
            terminative: false,
        }
    }
    pub fn with_comitative() -> Self {
        DimensionalPrefixes {
            indirect_object: None,
            comitative: true,
            locative: false,
            ablative: false,
            terminative: false,
        }
    }
    pub fn with_locative() -> Self {
        DimensionalPrefixes {
            indirect_object: None,
            comitative: false,
            locative: true,
            ablative: false,
            terminative: false,
        }
    }
    pub fn with_ablative() -> Self {
        DimensionalPrefixes {
            indirect_object: None,
            comitative: false,
            locative: false,
            ablative: true,
            terminative: false,
        }
    }
    pub fn with_terminative() -> Self {
        DimensionalPrefixes {
            indirect_object: None,
            comitative: false,
            locative: false,
            ablative: false,
            terminative: true,
        }
    }
    pub fn with_indirect_object(indirect_object: Person) -> Self {
        DimensionalPrefixes {
            indirect_object: Some(indirect_object),
            comitative: false,
            locative: false,
            ablative: false,
            terminative: false,
        }
    }
}

pub fn build_verb(
    stem: String,
    subordinate: bool,
    is_perfective: bool,
    subject: Person,
    object: Option<Person>,
    is_transitive: Option<bool>, // this is optional because verb may be in the dictionary
    preformative: Option<Preformative>,
    initial_person_prefix: Option<Person>,
    is_negative: bool,
    has_ventive: bool,
    has_middle_prefix: bool,
    dimensional_prefixes: DimensionalPrefixes,
) -> Result<String, String> {
    // finds the verb in the lexicon
    let verb_info = match find_verb(&stem) {
        Ok(verb) => verb.clone(),
        Err(_) => match is_transitive {
            Some(transitive_state) => VerbForm {
                stem: &stem,
                meaning: "",
                transitive: transitive_state,
                sing: VerbAspect {
                    perf: &stem,
                    imperf: &stem,
                },
                plur: VerbAspect {
                    perf: &stem,
                    imperf: &stem,
                },
            },
            None => Err(
                "Transitivity must be declared for verbs that are not in the dictionary"
                    .to_string(),
            )?,
        },
    };
    let mut verb = {
        if is_perfective {
            FiniteVerbalForm::new(
                verb_info.sing.perf.to_string(),
                is_perfective,
                verb_info.transitive,
            )
        } else {
            FiniteVerbalForm::new(
                verb_info.sing.imperf.to_string(),
                is_perfective,
                verb_info.transitive,
            )
        }
    };
    verb.slot_15 = subordinate;
    // Copies the preformative prefix
    verb.slot_2 = preformative;
    // Adds negative prefix if needed
    if is_negative {
        verb.slot_1 = Some(FirstPrefix::Negative);
    }
    if has_ventive {
        verb.slot_4 = Some(Ventive);
    }
    if has_middle_prefix {
        verb.slot_5 = Some(MiddlePrefix);
    }
    // DIMENSIONAL PREFIXES
    if dimensional_prefixes.ablative && dimensional_prefixes.terminative {
        // cannot be ablative and terminative at the same time
        return Err("Cannot have ablative and terminative markers at the same time".to_string());
    }
    match dimensional_prefixes.indirect_object {
        None => (),
        Some(prefix) => match prefix {
            Person::FirstSing => {
                verb.slot_7 = Some(IndirectObjectPrefix::FirstSing);
            }
            Person::SecondSing => {
                verb.slot_7 = Some(IndirectObjectPrefix::SecondSing);
            }
            Person::ThirdSingHuman => {
                verb.slot_7 = Some(IndirectObjectPrefix::ThirdSingHuman);
            }
            Person::ThirdSingNonHuman => {
                verb.slot_7 = Some(IndirectObjectPrefix::ThirdSingNonHuman);
            }
            Person::FirstPlur => {
                verb.slot_7 = Some(IndirectObjectPrefix::FirstPlur);
            }
            Person::SecondPlur => {
                verb.slot_7 = Some(IndirectObjectPrefix::SecondPlur);
            }
            Person::ThirdPlurHuman => {
                verb.slot_7 = Some(IndirectObjectPrefix::ThirdPlurHuman);
            }
            Person::ThirdPlurNonHuman => {
                verb.slot_7 = Some(IndirectObjectPrefix::ThirdPlurNonHuman);
            }
        },
    }
    if dimensional_prefixes.comitative {
        verb.slot_8 = Some(ComitativePrefix);
    }
    if dimensional_prefixes.ablative {
        verb.slot_9 = Some(AdverbialPrefix::Ablative);
    }
    if dimensional_prefixes.terminative {
        verb.slot_9 = Some(AdverbialPrefix::Terminative);
    }

    match initial_person_prefix {
        Some(prefix) => match prefix {
            Person::FirstSing => {
                verb.slot_6 = Some(InitialPersonPrefix::FirstSing);
            }
            Person::SecondSing => {
                verb.slot_6 = Some(InitialPersonPrefix::SecondSing);
            }
            Person::ThirdSingHuman => {
                verb.slot_6 = Some(InitialPersonPrefix::ThirdSingHuman);
            }
            Person::ThirdSingNonHuman => {
                verb.slot_6 = Some(InitialPersonPrefix::ThirdSingNonHuman);
            }
            Person::FirstPlur => {
                verb.slot_6 = Some(InitialPersonPrefix::FirstPlur);
            }
            Person::SecondPlur => {
                verb.slot_6 = Some(InitialPersonPrefix::SecondPlur);
            }
            Person::ThirdPlurHuman => {
                verb.slot_6 = Some(InitialPersonPrefix::ThirdPlurHuman);
            }
            Person::ThirdPlurNonHuman => {
                verb.slot_6 = Some(InitialPersonPrefix::ThirdPlurNonHuman);
            }
        },
        None => (),
    }
    // subjects of transitive verbs are prefixes, objects are suffixes
    // subjects of intransitive verbs are suffixes, no objects
    match (verb_info.transitive, is_perfective) {
        // transitive and perfective
        (true, true) => {
            match subject {
                Person::FirstSing => {
                    verb.slot_11 = Some(FinalPersonPrefix::FirstSingHuman);
                }
                Person::SecondSing => {
                    verb.slot_11 = Some(FinalPersonPrefix::SecondSingHuman);
                }
                Person::ThirdSingHuman => {
                    verb.slot_11 = Some(FinalPersonPrefix::ThirdSingHuman);
                }
                Person::ThirdSingNonHuman => {
                    verb.slot_11 = Some(FinalPersonPrefix::ThirdSingNonHuman);
                }
                _ => todo!("missing person markers for transitive perfective verbs"),
            };
            if let Some(object) = object {
                match object {
                    Person::FirstSing => {
                        verb.slot_14 = Some(FinalPersonSuffix::FirstSingHuman);
                    }
                    Person::SecondSing => {
                        verb.slot_14 = Some(FinalPersonSuffix::SecondSingHuman);
                    }
                    Person::ThirdSingHuman => {
                        verb.slot_14 = Some(FinalPersonSuffix::ThirdSingHuman);
                    }
                    Person::ThirdSingNonHuman => {
                        verb.slot_14 = Some(FinalPersonSuffix::ThirdSingNonHuman);
                    }
                    Person::FirstPlur => {
                        verb.slot_14 = Some(FinalPersonSuffix::FirstPlurHuman);
                    }
                    Person::SecondPlur => {
                        verb.slot_14 = Some(FinalPersonSuffix::SecondPlurHuman);
                    }
                    Person::ThirdPlurHuman => {
                        verb.slot_14 = Some(FinalPersonSuffix::ThirdPlurHuman);
                    }
                    Person::ThirdPlurNonHuman => {
                        verb.slot_14 = Some(FinalPersonSuffix::ThirdPlurNonHuman);
                    }
                }
            }
        }
        // transitive and imperfective
        (true, false) => {
            match subject {
                Person::FirstSing => {
                    verb.slot_14 = Some(FinalPersonSuffix::FirstSingHuman);
                }
                Person::SecondSing => {
                    verb.slot_14 = Some(FinalPersonSuffix::SecondSingHuman);
                }
                Person::ThirdSingHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdSingHuman);
                }
                Person::ThirdSingNonHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdSingNonHuman);
                }
                Person::FirstPlur => {
                    verb.slot_14 = Some(FinalPersonSuffix::FirstPlurHuman);
                }
                Person::SecondPlur => {
                    verb.slot_14 = Some(FinalPersonSuffix::SecondPlurHuman);
                }
                Person::ThirdPlurHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdPlurHuman);
                }
                Person::ThirdPlurNonHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdPlurNonHuman);
                }
            };
            if let Some(object) = object {
                match object {
                    Person::FirstSing => {
                        verb.slot_11 = Some(FinalPersonPrefix::FirstSingHuman);
                    }
                    Person::SecondSing => {
                        verb.slot_11 = Some(FinalPersonPrefix::SecondSingHuman);
                    }
                    Person::ThirdSingHuman => {
                        verb.slot_11 = Some(FinalPersonPrefix::ThirdSingHuman);
                    }
                    Person::ThirdSingNonHuman => {
                        verb.slot_11 = Some(FinalPersonPrefix::ThirdSingNonHuman);
                    }
                    _ => todo!(),
                    // Person::FirstPlur => {
                    //     verb.slot_11 = Some(FinalPersonPrefix::FirstPlurHuman);
                    // }
                    // Person::SecondPlur => {
                    //     verb.slot_11 = Some(FinalPersonPrefix::SecondPlurHuman);
                    // }
                    // Person::ThirdPlurHuman => {
                    //     verb.slot_11 = Some(FinalPersonPrefix::ThirdPlurHuman);
                    // }
                    // Person::ThirdPlurNonHuman => {
                    //     verb.slot_11 = Some(FinalPersonPrefix::ThirdPlurNonHuman);
                    // }
                }
            }
        }
        // intransitive and perfective
        // intransitive and imperfective
        (false, true) | (false, false) => {
            // ed marker
            if !verb_info.transitive && !is_perfective {
                verb.slot_13 = Some("ed".to_string());
            }

            match subject {
                Person::FirstSing => {
                    verb.slot_14 = Some(FinalPersonSuffix::FirstSingHuman);
                }
                Person::SecondSing => {
                    verb.slot_14 = Some(FinalPersonSuffix::SecondSingHuman);
                }
                Person::ThirdSingHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdSingHuman);
                }
                Person::ThirdSingNonHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdSingNonHuman);
                }
                Person::FirstPlur => {
                    verb.slot_14 = Some(FinalPersonSuffix::FirstPlurHuman);
                }
                Person::SecondPlur => {
                    verb.slot_14 = Some(FinalPersonSuffix::SecondPlurHuman);
                }
                Person::ThirdPlurHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdPlurHuman);
                }
                Person::ThirdPlurNonHuman => {
                    verb.slot_14 = Some(FinalPersonSuffix::ThirdPlurNonHuman);
                }
            }
        }
    };

    return verb.print();
}
