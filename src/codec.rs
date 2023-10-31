/// Encodes/decodes an inverted index
pub trait Codec {
    fn name(&self) -> &str;
    fn postings_format(&self) -> Box<dyn PostingsFormat>;
    fn doc_values_format(&self) -> Box<dyn DocValuesFormat>;
    fn stored_fields_format(&self) -> Box<dyn StoredFieldsFormat>;
    fn term_vectors_format(&self) -> Box<dyn TermVectorsFormat>;
    fn field_infos_format(&self) -> Box<dyn FieldInfosFormat>;
    fn segment_info_format(&self) -> Box<dyn SegmentInfoFormat>;
    fn norms_format(&self) -> Box<dyn NormsFormat>;
    fn live_docs_format(&self) -> Box<dyn LiveDocsFormat>;
    fn compound_format(&self) -> Box<dyn CompoundFormat>;
    fn points_format(&self) -> Box<dyn PointsFormat>;
    fn knn_vectors_format(&self) -> Box<dyn KnnVectorsFormat>;
}


pub trait PostingsFormat {}

pub trait DocValuesFormat {}

pub trait StoredFieldsFormat {}

pub trait TermVectorsFormat {}

pub trait FieldInfosFormat {}

pub trait SegmentInfoFormat {}

pub trait NormsFormat {}

pub trait LiveDocsFormat {}

pub trait CompoundFormat {}

pub trait PointsFormat {}

pub trait KnnVectorsFormat {}

pub mod simple_text;
pub use simple_text::SimpleTextCodec;
