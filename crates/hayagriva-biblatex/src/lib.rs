use std::borrow::Cow;

use biblatex::{ChunksExt, PermissiveType};
use citationberg::{
    LongShortForm,
    taxonomy::{
        self, DateVariable, NameVariable, NumberVariable, PageVariable, StandardVariable,
    },
};
use hayagriva_core::{EntryLike, MaybeTyped, Numeric, PageRanges, PageRangesPart};
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
            NameVariable::Author => todo!(),
            NameVariable::Chair => todo!(),
            NameVariable::CollectionEditor => todo!(),
            NameVariable::Compiler => todo!(),
            NameVariable::Composer => todo!(),
            NameVariable::ContainerAuthor => todo!(),
            NameVariable::Contributor => todo!(),
            NameVariable::Curator => todo!(),
            NameVariable::Director => todo!(),
            NameVariable::Editor => todo!(),
            NameVariable::EditorialDirector => todo!(),
            NameVariable::EditorTranslator => todo!(),
            NameVariable::ExecutiveProducer => todo!(),
            NameVariable::Guest => todo!(),
            NameVariable::Host => todo!(),
            NameVariable::Illustrator => todo!(),
            NameVariable::Interviewer => todo!(),
            NameVariable::Narrator => todo!(),
            NameVariable::Organizer => todo!(),
            NameVariable::OriginalAuthor => todo!(),
            NameVariable::Performer => todo!(),
            NameVariable::Producer => todo!(),
            NameVariable::Recipient => todo!(),
            NameVariable::ReviewedAuthor => todo!(),
            NameVariable::ScriptWriter => todo!(),
            NameVariable::SeriesCreator => todo!(),
            NameVariable::Translator => todo!(),
        }
    }

    fn resolve_date_variable(
        &self,
        variable: DateVariable,
    ) -> Option<Cow<'_, hayagriva_core::Date>> {
        match variable {
            DateVariable::Accessed => todo!(),
            DateVariable::AvailableDate => todo!(),
            DateVariable::EventDate => todo!(),
            DateVariable::Issued => todo!(),
            DateVariable::OriginalDate => todo!(),
            DateVariable::Submitted => todo!(),
        }
    }

    fn matches_entry_type(&self, kind: taxonomy::Kind) -> bool {
        todo!()
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
