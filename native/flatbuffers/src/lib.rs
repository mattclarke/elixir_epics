extern crate flatbuffers;

#[allow(dead_code, unused_imports, non_camel_case_types)]
#[path = "./f144_logdata_generated.rs"]
mod f144_logdata_generated;
pub use f144_logdata_generated::{f144_LogData, Value, F_144_LOG_DATA_IDENTIFIER};
use crate::f144_logdata_generated::{f144_LogDataArgs, Double, DoubleArgs, ArrayLong, ArrayLongArgs};

#[allow(dead_code, unused_imports, non_camel_case_types)]
#[path = "./al00_alarm_generated.rs"]
mod al00_alarm_generated;
pub use al00_alarm_generated::{Alarm, ALARM_IDENTIFIER};
use crate::al00_alarm_generated::{AlarmArgs, Severity};

#[allow(dead_code, unused_imports, non_camel_case_types)]
#[path = "./ep01_epics_connection_generated.rs"]
mod ep01_epics_connection_generated;
pub use ep01_epics_connection_generated::{EpicsPVConnectionInfo, EPICS_PVCONNECTION_INFO_IDENTIFIER};
use crate::ep01_epics_connection_generated::{EpicsPVConnectionInfoArgs, ConnectionInfo};

use rustler::types::binary::{NewBinary, Binary};
use rustler::{Env};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct LongArray {
    data: Vec<i64>,
}

#[rustler::nif]
fn convert_to_f144_double<'a>(env: Env<'a>, source_name: &str, timestamp: i64, value: f64) -> Binary<'a>{
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

    builder.finish(logdata, Option::from(F_144_LOG_DATA_IDENTIFIER));
    let buf = builder.finished_data();

    let mut new_binary = NewBinary::new(env, buf.len());
    new_binary.copy_from_slice(buf);
    return Binary::from(new_binary);
}

#[rustler::nif]
fn convert_to_f144_long_array<'a>(env: Env<'a>, source_name: &str, timestamp: i64, values: &str) -> Binary<'a>{
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let source = builder.create_string(source_name);

    // make the array look like a JSON object, so we can deserialise it
    let as_json: String = format!("{{\"data\":{}}}", values);
    let obj: LongArray = serde_json::from_str(as_json.as_str()).unwrap();

    let log_values = builder.create_vector(&obj.data);
    let log_value= ArrayLong::create(&mut builder, &ArrayLongArgs {
        value: Some(log_values),
    });

    let logdata = f144_LogData::create(&mut builder, &f144_LogDataArgs{
        source_name: Some(source),
        timestamp,
        value: Some(log_value.as_union_value()),
        value_type: Value::ArrayLong,
        ..Default::default()
    });

    builder.finish(logdata, Option::from(F_144_LOG_DATA_IDENTIFIER));
    let buf = builder.finished_data();

    let mut new_binary = NewBinary::new(env, buf.len());
    new_binary.copy_from_slice(buf);
    return Binary::from(new_binary);
}

#[rustler::nif]
fn convert_to_al00<'a>(env: Env<'a>, source_name: &str, timestamp: i64, severity: i8, message: &str) -> Binary<'a>{
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let source = builder.create_string(source_name);
    let msg = builder.create_string(message);

    let mut sevr = Severity::OK;

    match severity {
        1 => sevr = Severity::MINOR,
        2 => sevr = Severity::MAJOR,
        3 => sevr = Severity::INVALID,
        _ => sevr = Severity::OK
    }

    let alarmdata = Alarm::create(&mut builder, &AlarmArgs{
        source_name: Some(source),
        timestamp,
        severity: sevr,
        message: Some(msg),
        ..Default::default()
    });

    builder.finish(alarmdata, Option::from(ALARM_IDENTIFIER));
    let buf = builder.finished_data();

    let mut new_binary = NewBinary::new(env, buf.len());
    new_binary.copy_from_slice(buf);
    return Binary::from(new_binary);
}

#[rustler::nif]
fn convert_to_ep01<'a>(env: Env<'a>, source_name: &str, timestamp: i64, status: i8) -> Binary<'a>{
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let source = builder.create_string(source_name);

    let mut state = ConnectionInfo::UNKNOWN;

    match status {
        1 => state = ConnectionInfo::NEVER_CONNECTED,
        2 => state = ConnectionInfo::CONNECTED,
        3 => state = ConnectionInfo::DISCONNECTED,
        4 => state = ConnectionInfo::DESTROYED,
        5 => state = ConnectionInfo::CANCELLED,
        6 => state = ConnectionInfo::FINISHED,
        7 => state = ConnectionInfo::REMOTE_ERROR,
        _ => state = ConnectionInfo::UNKNOWN,
    }
    let connection = EpicsPVConnectionInfo::create(&mut builder, &EpicsPVConnectionInfoArgs{
        source_name: Some(source),
        timestamp,
        status: state,
        ..Default::default()
    });

    builder.finish(connection, Option::from(EPICS_PVCONNECTION_INFO_IDENTIFIER));
    let buf = builder.finished_data();

    let mut new_binary = NewBinary::new(env, buf.len());
    new_binary.copy_from_slice(buf);
    return Binary::from(new_binary);
}
rustler::init!("Elixir.FlatBuffers", [convert_to_f144_double, convert_to_f144_long_array, convert_to_al00, convert_to_ep01]);
