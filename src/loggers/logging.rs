use log::LogRecord;
use time;
use std::io::{Write, Error};
use ::Config;

#[inline(always)]
pub fn try_log<W>(config: &Config, record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    if config.time.is_some() && config.time.unwrap() <= record.level() {
        try!(write_time(write));
    }

    if config.level.is_some() && config.level.unwrap() <= record.level() {
        try!(write_level(record, write));
    }

    if config.target.is_some() && config.target.unwrap() <= record.level() {
        try!(write_target(record, write));
    }

    if config.location.is_some() && config.location.unwrap() <= record.level() {
        try!(write_location(record, write));
    }

    try!(write_args(record, write));
    try!(write.flush());
    Ok(())
}

#[inline(always)]
pub fn write_time<W>(write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    let cur_time = time::now();
    try!(write!(write, "{:02}:{:02}:{:02} ",
                cur_time.tm_hour,
                cur_time.tm_min,
                cur_time.tm_sec));
    Ok(())
}

#[inline(always)]
pub fn write_level<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(write!(write, "[{}] ", record.level()));
    Ok(())
}

#[inline(always)]
pub fn write_target<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(write!(write, "{}: ", record.target()));
    Ok(())
}

#[inline(always)]
pub fn write_location<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(write!(write, "[{}:{}] ",
        record.location().file(),
        record.location().line()));
    Ok(())
}

#[inline(always)]
pub fn write_args<W>(record: &LogRecord, write: &mut W) -> Result<(), Error>
    where W: Write + Sized
{
    try!(writeln!(write, "{}", record.args()));
    Ok(())
}
