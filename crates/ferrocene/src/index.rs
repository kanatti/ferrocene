use analysis::{Analyzer, StandardAnalyzer};
use codecs::{Codec, SimpleTextCodec};

use crate::document::Document;

pub struct IndexWriterConfig<A: Analyzer, C: Codec> {
    pub analyzer: A,
    pub codec: C,
}

impl<A> IndexWriterConfig<A, SimpleTextCodec>
where
    A: Analyzer,
{
    /// Defaults to SimpleTextCodec
    pub fn with_analyzer(analyzer: A) -> IndexWriterConfig<A, SimpleTextCodec> {
        IndexWriterConfig {
            analyzer: analyzer,
            codec: SimpleTextCodec {},
        }
    }
}

impl<C> IndexWriterConfig<StandardAnalyzer, C>
where
    C: Codec,
{
    /// Defaults to StandardAnalyzer
    pub fn with_codec(codec: C) -> IndexWriterConfig<StandardAnalyzer, C> {
        IndexWriterConfig {
            analyzer: StandardAnalyzer {},
            codec: codec,
        }
    }
}

impl IndexWriterConfig<StandardAnalyzer, SimpleTextCodec> {
    /// Defaults to StandardAnalyzer and SimpleTextCodec
    pub fn with_defaults() -> IndexWriterConfig<StandardAnalyzer, SimpleTextCodec> {
        IndexWriterConfig {
            analyzer: StandardAnalyzer {},
            codec: SimpleTextCodec {},
        }
    }
}

impl<A, C> IndexWriterConfig<A, C>
where
    A: Analyzer,
    C: Codec,
{
    pub fn new(analyzer: A, codec: C) -> IndexWriterConfig<A, C> {
        IndexWriterConfig { analyzer, codec }
    }
}

pub struct IndexWriter<A: Analyzer, C: Codec> {
    pub config: IndexWriterConfig<A, C>,
}

impl<A, C> IndexWriter<A, C>
where
    A: Analyzer,
    C: Codec,
{
    pub fn add(&mut self, _doc: Document) {
        todo!()
    }

    pub fn close(&mut self) {
        todo!()
    }
}

