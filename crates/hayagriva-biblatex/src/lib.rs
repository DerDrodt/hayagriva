use std::borrow::Cow;

use biblatex::{ChunksExt, EditorType, EntryType, PermissiveType};
use citationberg::{
    LongShortForm,
    taxonomy::{
        DateVariable, Kind, NameVariable, NumberVariable, PageVariable, StandardVariable,
    },
};
use hayagriva_core::{
    Date, EntryLike, MaybeTyped, Numeric, PageRanges, PageRangesPart,
    biblatex_conversion as conversion,
};
use unic_langid::LanguageIdentifier;

pub struct Entry(pub biblatex::Entry);

impl EntryLike for Entry {
    fn resolve_number_variable(
        &self,
        variable: NumberVariable,
    ) -> Option<hayagriva_core::MaybeTyped<Cow<'_, hayagriva_core::Numeric>>> {
        match variable {
            NumberVariable::ChapterNumber => todo!(),
            NumberVariable::CitationNumber => todo!(),
            NumberVariable::CollectionNumber => todo!(),
            NumberVariable::Edition => todo!(),
            NumberVariable::FirstReferenceNoteNumber => todo!(),
            NumberVariable::Issue => todo!(),
            NumberVariable::Locator => todo!(),
            NumberVariable::Number => todo!(),
            NumberVariable::NumberOfPages => todo!(),
            NumberVariable::NumberOfVolumes => todo!(),
            NumberVariable::PageFirst => todo!(),
            NumberVariable::PartNumber => todo!(),
            NumberVariable::PrintingNumber => todo!(),
            NumberVariable::Section => todo!(),
            NumberVariable::SupplementNumber => todo!(),
            NumberVariable::Version => todo!(),
            NumberVariable::Volume => todo!(),
        }
    }

    fn resolve_page_variable(
        &self,
        variable: PageVariable,
    ) -> Option<hayagriva_core::MaybeTyped<hayagriva_core::PageRanges>> {
        match variable {
            PageVariable::Page => match self.0.pages() {
                Ok(PermissiveType::Typed(pages)) => {
                    Some(MaybeTyped::Typed(PageRanges::new(
                        pages
                            .into_iter()
                            .map(|p| {
                                if p.start == p.end {
                                    PageRangesPart::SinglePage(Numeric::from(p.start))
                                } else {
                                    PageRangesPart::Range(
                                        Numeric::from(p.start),
                                        Numeric::from(p.end),
                                    )
                                }
                            })
                            .collect(),
                    )))
                }
                Ok(PermissiveType::Chunks(c)) => {
                    Some(MaybeTyped::infallible_from_str(&c.format_verbatim()))
                }
                _ => None,
            },
        }
    }

    fn resolve_standard_variable(
        &self,
        form: LongShortForm,
        variable: StandardVariable,
    ) -> Option<Cow<'_, hayagriva_core::ChunkedString>> {
        match variable {
            StandardVariable::Abstract => todo!(),
            StandardVariable::Annote => todo!(),
            StandardVariable::Archive => todo!(),
            StandardVariable::ArchiveCollection => todo!(),
            StandardVariable::ArchiveLocation => todo!(),
            StandardVariable::ArchivePlace => todo!(),
            StandardVariable::Authority => todo!(),
            StandardVariable::CallNumber => todo!(),
            StandardVariable::CitationKey => todo!(),
            StandardVariable::CitationLabel => todo!(),
            StandardVariable::CollectionTitle => todo!(),
            StandardVariable::ContainerTitle => todo!(),
            StandardVariable::ContainerTitleShort => todo!(),
            StandardVariable::Dimensions => todo!(),
            StandardVariable::Division => todo!(),
            StandardVariable::DOI => todo!(),
            StandardVariable::Event => todo!(),
            StandardVariable::EventTitle => todo!(),
            StandardVariable::EventPlace => todo!(),
            StandardVariable::Genre => todo!(),
            StandardVariable::ISBN => todo!(),
            StandardVariable::ISSN => todo!(),
            StandardVariable::Jurisdiction => todo!(),
            StandardVariable::Keyword => todo!(),
            StandardVariable::Language => todo!(),
            StandardVariable::License => todo!(),
            StandardVariable::Medium => todo!(),
            StandardVariable::Note => todo!(),
            StandardVariable::OriginalPublisher => todo!(),
            StandardVariable::OriginalPublisherPlace => todo!(),
            StandardVariable::OriginalTitle => todo!(),
            StandardVariable::PartTitle => todo!(),
            StandardVariable::PMCID => todo!(),
            StandardVariable::PMID => todo!(),
            StandardVariable::Publisher => todo!(),
            StandardVariable::PublisherPlace => todo!(),
            StandardVariable::References => todo!(),
            StandardVariable::ReviewedGenre => todo!(),
            StandardVariable::ReviewedTitle => todo!(),
            StandardVariable::Scale => todo!(),
            StandardVariable::Source => todo!(),
            StandardVariable::Status => todo!(),
            StandardVariable::Title => todo!(),
            StandardVariable::TitleShort => todo!(),
            StandardVariable::URL => todo!(),
            StandardVariable::VolumeTitle => todo!(),
            StandardVariable::YearSuffix => todo!(),
        }
    }

    fn resolve_name_variable(
        &self,
        variable: NameVariable,
    ) -> Vec<Cow<'_, hayagriva_core::Person>> {
        match variable {
            NameVariable::Author => self.0.author().ok(),
            NameVariable::Chair => None,
            NameVariable::CollectionEditor => None,
            NameVariable::Compiler => self.editors_with_role(EditorType::Collaborator),
            NameVariable::Composer => None,
            NameVariable::ContainerAuthor => self.0.book_author().ok(),
            NameVariable::Contributor => None,
            NameVariable::Curator => None,
            NameVariable::Director => {
                if is_non_standard_type(&self.0.entry_type, "video") {
                    self.editors_with_role(EditorType::Director)
                } else {
                    self.0.author().ok()
                }
            }
            NameVariable::Editor => self.editors_with_role(EditorType::Editor),
            NameVariable::EditorialDirector => None,
            NameVariable::EditorTranslator => None,
            NameVariable::ExecutiveProducer => None,
            NameVariable::Guest => None,
            NameVariable::Host => None,
            NameVariable::Illustrator => None,
            NameVariable::Interviewer => None,
            NameVariable::Narrator => None,
            NameVariable::Organizer => self.editors_with_role(EditorType::Organizer),
            NameVariable::OriginalAuthor => None,
            NameVariable::Performer => None,
            NameVariable::Producer => None,
            NameVariable::Recipient => None,
            NameVariable::ReviewedAuthor => None,
            NameVariable::ScriptWriter => None,
            NameVariable::SeriesCreator => None,
            NameVariable::Translator => self.0.translator().ok(),
        }
        .unwrap_or_default()
        .into_iter()
        .map(|p| Cow::Owned(conversion::person(&p)))
        .collect()
    }

    fn resolve_date_variable(&self, variable: DateVariable) -> Option<Cow<'_, Date>> {
        match variable {
            DateVariable::Accessed => self
                .0
                .url_date()
                .ok()
                .and_then(|d| match d {
                    PermissiveType::Typed(t) => Some(t),
                    PermissiveType::Chunks(_) => None,
                })
                .map(|d| Cow::Owned(conversion::date(d))),
            DateVariable::AvailableDate => None,
            DateVariable::EventDate => self
                .0
                .event_date()
                .ok()
                .and_then(|d| match d {
                    PermissiveType::Typed(t) => Some(t),
                    PermissiveType::Chunks(_) => None,
                })
                .map(|d| Cow::Owned(conversion::date(d))),
            DateVariable::Issued => self
                .0
                .date()
                .ok()
                .and_then(|d| match d {
                    PermissiveType::Typed(t) => Some(t),
                    PermissiveType::Chunks(_) => None,
                })
                .map(|d| Cow::Owned(conversion::date(d))),
            DateVariable::OriginalDate => self
                .0
                .orig_date()
                .ok()
                .and_then(|d| match d {
                    PermissiveType::Typed(t) => Some(t),
                    PermissiveType::Chunks(_) => None,
                })
                .map(|d| Cow::Owned(conversion::date(d))),
            DateVariable::Submitted => None,
        }
    }

    fn matches_entry_type(&self, kind: Kind) -> bool {
        let ty = &self.0.entry_type;
        match kind {
            Kind::Article
            | Kind::ArticleJournal
            | Kind::ArticleMagazine
            | Kind::ArticleNewspaper => ty == &EntryType::Article,
            Kind::Bill => {
                is_non_standard_type(ty, "legislation") && self.0.publisher().is_err()
            }
            Kind::Book => ty == &EntryType::Book,
            Kind::Broadcast => false,
            Kind::Chapter => ty == &EntryType::InCollection,
            Kind::Classic => false,
            Kind::Collection => ty == &EntryType::Collection,
            Kind::Dataset => ty == &EntryType::Dataset,
            Kind::Document => ty == &EntryType::Misc,
            Kind::Entry | Kind::EntryDictionary | Kind::EntryEncyclopedia => {
                ty == &EntryType::InReference
            }
            Kind::Event => false,
            Kind::Figure | Kind::Graphic => is_non_standard_type(ty, "image"),
            Kind::Hearing => false,
            Kind::Interview => ty == &EntryType::Misc,
            Kind::LegalCase => is_non_standard_type(ty, "jurisdiction"),
            Kind::Legislation => is_non_standard_type(ty, "legislation"),
            Kind::Manuscript => ty == &EntryType::Unpublished,
            Kind::Map => false,
            Kind::MotionPicture => is_non_standard_type(ty, "movie"),
            Kind::MusicalScore => is_non_standard_type(ty, "audio"),
            Kind::Pamphlet => ty == &EntryType::Booklet,
            Kind::PaperConference => ty == &EntryType::InProceedings,
            Kind::Patent => ty == &EntryType::Patent,
            Kind::Performance => false,
            Kind::Periodical => ty == &EntryType::Periodical,
            Kind::PersonalCommunication => is_non_standard_type(ty, "letter"),
            Kind::Post | Kind::PostWeblog => ty == &EntryType::Online,
            Kind::Regulation => false,
            Kind::Report => ty == &EntryType::Report,
            Kind::Review | Kind::ReviewBook => is_non_standard_type(ty, "review"),
            Kind::Software => ty == &EntryType::Software,
            Kind::Song => is_non_standard_type(ty, "music"),
            Kind::Speech => false,
            Kind::Standard => false,
            Kind::Thesis => ty == &EntryType::Thesis,
            Kind::Treaty => is_non_standard_type(ty, "legal"),
            Kind::Webpage => ty == &EntryType::Online,
        }
    }

    fn is_english(&self) -> Option<bool> {
        if let Ok(PermissiveType::Typed(id)) = self.0.langid() {
            let id: LanguageIdentifier = id.into();
            Some(id.language == "en")
        } else {
            None
        }
    }

    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.0.key)
    }
}

impl Entry {
    fn editors_with_role(&self, role: EditorType) -> Option<Vec<biblatex::Person>> {
        let mut result = vec![];
        if let Ok(eds_with_roles) = self.0.editors() {
            for (eds, r) in eds_with_roles.iter() {
                if r == &role {
                    for ed in eds {
                        result.push(ed.clone())
                    }
                }
            }
        }
        Some(result)
    }
}

fn is_non_standard_type(ty: &EntryType, label: &str) -> bool {
    if let EntryType::Unknown(s) = ty
        && &s.to_lowercase() == label
    {
        true
    } else {
        false
    }
}
