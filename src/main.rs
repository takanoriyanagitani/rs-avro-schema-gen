use std::env;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::ExitCode;

use rs_avro_schema_gen::bind;
use rs_avro_schema_gen::lift;

use rs_avro_schema_gen::record::flat::FlatRecord;

use rs_avro_schema_gen::record::flat::json_sample_bytes2schema;

use rs_avro_schema_gen::app::json2flat::json_sample_bytes2flat_schema2writer;

pub fn get_env_by_key(key: String) -> impl FnMut() -> Result<String, io::Error> {
    move || env::var(key.as_str()).map_err(io::Error::other)
}

pub fn get_env_opt_by_key(key: String) -> impl FnMut() -> Result<Option<String>, io::Error> {
    let mut key2val = get_env_by_key(key);
    move || {
        let ostr: Option<String> = key2val().ok();
        Ok(ostr)
    }
}

pub fn string2limit(s: String) -> Result<u64, io::Error> {
    str::parse(s.as_str()).map_err(io::Error::other)
}

const JSON_SAMPLE_LIMIT_DEFAULT: u64 = 1_048_576;

pub fn json_sample_size_limit() -> impl FnMut() -> Result<u64, io::Error> {
    bind!(
        get_env_by_key("ENV_JSON_SAMPLE_LIMIT".into()),
        lift!(string2limit)
    )
}

pub fn json_sample_size_limit_default() -> impl FnMut() -> Result<u64, io::Error> {
    move || {
        Ok(json_sample_size_limit()()
            .ok()
            .unwrap_or(JSON_SAMPLE_LIMIT_DEFAULT))
    }
}

pub fn stdin2str_limited(limit: u64) -> impl FnMut() -> Result<Vec<u8>, io::Error> {
    move || {
        let i = io::stdin();
        let il = i.lock();
        let mut limited = il.take(limit);
        let mut buf: Vec<u8> = Vec::new();
        limited.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

pub fn stdin2json_sample_limited_default() -> impl FnMut() -> Result<Vec<u8>, io::Error> {
    bind!(json_sample_size_limit_default(), stdin2str_limited)
}

pub fn name_source() -> impl FnMut() -> Result<String, io::Error> {
    get_env_by_key("ENV_NAME".into())
}

pub fn namespace_source() -> impl FnMut() -> Result<Option<String>, io::Error> {
    get_env_opt_by_key("ENV_NAMESPACE".into())
}

pub fn flat_record_schema2stdout() -> impl FnMut(FlatRecord) -> Result<(), io::Error> {
    move |f: FlatRecord| {
        let o = io::stdout();
        let mut ol = o.lock();
        serde_json::to_writer_pretty(&mut ol, &f).map_err(io::Error::other)?;
        ol.flush()?;
        Ok(())
    }
}

fn sample_json2flat_schema() -> impl FnMut() -> Result<(), io::Error> {
    json_sample_bytes2flat_schema2writer(
        name_source(),
        namespace_source(),
        stdin2json_sample_limited_default(),
        json_sample_bytes2schema,
        flat_record_schema2stdout(),
    )
}

fn sub() -> Result<(), io::Error> {
    sample_json2flat_schema()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
