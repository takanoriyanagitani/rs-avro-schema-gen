use std::io;

use crate::record::flat::FlatRecord;

pub fn json_sample_bytes2flat_schema2writer<N, S, J, G, W>(
    mut name_source: N,
    mut namespace_source: S,
    mut sample_json_source: J,
    generator: G,
    mut writer: W,
) -> impl FnMut() -> Result<(), io::Error>
where
    N: FnMut() -> Result<String, io::Error>,
    S: FnMut() -> Result<Option<String>, io::Error>,
    J: FnMut() -> Result<Vec<u8>, io::Error>,
    G: Fn(String, Option<String>, Vec<u8>) -> Result<FlatRecord, io::Error>,
    W: FnMut(FlatRecord) -> Result<(), io::Error>,
{
    move || {
        let name: String = name_source()?;
        let namespace: Option<String> = namespace_source()?;
        let sample_json: Vec<u8> = sample_json_source()?;
        let f: FlatRecord = generator(name, namespace, sample_json)?;
        writer(f)
    }
}
