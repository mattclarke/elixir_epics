extern crate flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports, non_camel_case_types)]
#[path = "./f144_logdata_generated.rs"]
mod f144_logdata_generated;
pub use f144_logdata_generated::{root_as_f_144_log_data, f144_LogData, Value};
use crate::f144_logdata_generated::{f144_LogDataArgs, Double, DoubleArgs};

#[rustler::nif]
fn convert_flatbuffer_double(source_name : &str, timestamp: i64, value: f64) -> Vec<u8>{
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let source = builder.create_string(source_name);
    let log_value= Double::create(&mut builder, &DoubleArgs {
        value,
    });

    let logdata = f144_LogData::create(&mut builder, &f144_LogDataArgs{
        source_name: Some(source),
        timestamp,
        value: Some(log_value.as_union_value()),
        value_type: Value::Double,
        ..Default::default()
    });

    builder.finish(logdata, Option::from("f144"));
    let buf = builder.finished_data();
    return buf.to_vec();
}

rustler::init!("Elixir.FlatBuffers", [convert_flatbuffer_double]);