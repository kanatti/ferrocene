use crate::codec::{
    Codec, CompoundFormat, DocValuesFormat, FieldInfosFormat, KnnVectorsFormat, LiveDocsFormat,
    NormsFormat, PointsFormat, PostingsFormat, SegmentInfoFormat, StoredFieldsFormat,
    TermVectorsFormat,
};

pub struct SimpleTextCodec {
    name: String,
}

impl SimpleTextCodec {
    pub fn new() -> Self {
        Self {
            name: "SimpleText".to_string(),
        }
    }
}

impl Codec for SimpleTextCodec {
    fn name(&self) -> &str {
        &self.name
    }

    fn postings_format(&self) -> Box<dyn PostingsFormat> {
        todo!()
    }

    fn doc_values_format(&self) -> Box<dyn DocValuesFormat> {
        todo!()
    }

    fn stored_fields_format(&self) -> Box<dyn StoredFieldsFormat> {
        todo!()
    }

    fn term_vectors_format(&self) -> Box<dyn TermVectorsFormat> {
        todo!()
    }

    fn field_infos_format(&self) -> Box<dyn FieldInfosFormat> {
        todo!()
    }

    fn segment_info_format(&self) -> Box<dyn SegmentInfoFormat> {
        todo!()
    }

    fn norms_format(&self) -> Box<dyn NormsFormat> {
        todo!()
    }

    fn live_docs_format(&self) -> Box<dyn LiveDocsFormat> {
        todo!()
    }

    fn compound_format(&self) -> Box<dyn CompoundFormat> {
        todo!()
    }

    fn points_format(&self) -> Box<dyn PointsFormat> {
        todo!()
    }

    fn knn_vectors_format(&self) -> Box<dyn KnnVectorsFormat> {
        todo!()
    }
}
