use crate::{
    document::Document,
    store::{Directory, InputStream, OutputStream},
};

use super::field_info::FieldInfos;

/// Writes a single document into the index, in a row oriented format.
/// Index file points to specific location of a doc in fields file.
pub struct FieldsWriter<'a, O, D>
{
    pub field_infos: &'a FieldInfos,
    pub dir: &'a D,
    pub fields_stream: O,
    pub index_stream: O,
}

impl<'a, O, I, D> FieldsWriter<'a, O, D>
where
    O: OutputStream,
    I: InputStream,
    D: Directory<Output = O, Input = I>,
{
    pub fn new(dir: &'a D, segment_id: &str, field_infos: &'a FieldInfos) -> Self {
        let fields_stream = dir.create_file(&format!("{}.fdt", segment_id)).unwrap();
        let index_stream = dir.create_file(&format!("{}.fdx", segment_id)).unwrap();

        Self {
            field_infos,
            fields_stream,
            index_stream,
            dir: &dir,
        }
    }

    pub fn add_doc(&mut self, doc: &Document) {
        // 1. Get current file-pointer from fields_stream and write as long to index_stream
        self.index_stream
            .write_long(self.fields_stream.stream_position());

        // 2. Find count of stored fields and write to fields_stream as vInt
        let stored_count = doc.fields.iter().filter(|f| f.is_stored).count();
        self.fields_stream.write_vint(stored_count as u32);

        // 3. For each stored field, write field number, is_tokenized flag and field value.
        doc.fields.iter().for_each(|f| {
            if f.is_stored {
                let field_number = self.field_infos.get_field_number(&f.name).unwrap();
                let is_tokenized = f.is_analyzed as u8;

                self.fields_stream.write_vint(field_number as u32);
                self.fields_stream.write_byte(is_tokenized);
                self.fields_stream.write_string(&f.value);
            }
        });
    }

    pub fn close(&mut self) {
        todo!()
    }
}
