//! Python bindings for `ocel`.
//!
//! Exposes OCEL 2.0 logs to Python as the `ocel` module: reading/writing the
//! three formats, validation, filtering, connected-components sampling, and
//! columnar exports — as plain dicts of columns, or as Arrow tables via the
//! Arrow `PyCapsule` interface (zero-copy into Polars / pyarrow / pandas).

use std::path::PathBuf;
use std::sync::Arc;

use arrow_array::{
    ArrayRef, BooleanArray, Float64Array, Int64Array, RecordBatch, StringArray,
    TimestampMicrosecondArray,
};
use arrow_schema::{DataType, Field, Schema, SchemaRef, TimeUnit};
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3_arrow::PyTable;

use ocel::io::{json, sqlite, xml};
use ocel::{AttrValue, Ocel};

/// An OCEL 2.0 event log.
#[pyclass(frozen)]
struct OcelLog {
    inner: Ocel,
}

fn io_err(e: ocel::IoError) -> PyErr {
    PyIOError::new_err(e.to_string())
}

fn attr_to_py<'py>(py: Python<'py>, value: &AttrValue) -> PyResult<Bound<'py, PyAny>> {
    Ok(match value {
        AttrValue::String(s) => s.into_pyobject(py)?.into_any(),
        AttrValue::Integer(i) => i.into_pyobject(py)?.into_any(),
        AttrValue::Float(f) => f.into_pyobject(py)?.into_any(),
        AttrValue::Boolean(b) => b.into_pyobject(py)?.to_owned().into_any(),
        AttrValue::Time(t) => t.into_pyobject(py)?.into_any(),
    })
}

// ---------------------------------------------------------------------------
// Arrow export helpers
// ---------------------------------------------------------------------------

fn arrow_err(e: impl std::fmt::Display) -> PyErr {
    PyValueError::new_err(e.to_string())
}

fn utf8_field(name: &str) -> Field {
    Field::new(name, DataType::Utf8, false)
}

fn ts_type() -> DataType {
    DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into()))
}

fn utf8_col<'a, I: IntoIterator<Item = &'a str>>(values: I) -> ArrayRef {
    Arc::new(StringArray::from_iter_values(values))
}

fn ts_col(values: Vec<i64>) -> ArrayRef {
    Arc::new(TimestampMicrosecondArray::from(values).with_timezone("UTC"))
}

fn table(fields: Vec<Field>, columns: Vec<ArrayRef>) -> PyResult<PyTable> {
    let schema: SchemaRef = Arc::new(Schema::new(fields));
    let batch = RecordBatch::try_new(schema.clone(), columns).map_err(arrow_err)?;
    PyTable::try_new(vec![batch], schema).map_err(arrow_err)
}

/// Long-format attribute values as typed Arrow columns with nulls
/// (Arrow columns are homogeneous, so the mixed-type `value` splits by type).
#[derive(Default)]
struct AttrColumns {
    strings: Vec<Option<String>>,
    integers: Vec<Option<i64>>,
    floats: Vec<Option<f64>>,
    booleans: Vec<Option<bool>>,
    times: Vec<Option<i64>>,
}

impl AttrColumns {
    fn push(&mut self, value: &AttrValue) {
        self.strings.push(match value {
            AttrValue::String(s) => Some(s.clone()),
            _ => None,
        });
        self.integers.push(match value {
            AttrValue::Integer(i) => Some(*i),
            _ => None,
        });
        self.floats.push(match value {
            AttrValue::Float(f) => Some(*f),
            _ => None,
        });
        self.booleans.push(match value {
            AttrValue::Boolean(b) => Some(*b),
            _ => None,
        });
        self.times.push(match value {
            AttrValue::Time(t) => Some(t.timestamp_micros()),
            _ => None,
        });
    }

    fn fields() -> Vec<Field> {
        vec![
            Field::new("value_string", DataType::Utf8, true),
            Field::new("value_integer", DataType::Int64, true),
            Field::new("value_float", DataType::Float64, true),
            Field::new("value_boolean", DataType::Boolean, true),
            Field::new("value_time", ts_type(), true),
        ]
    }

    fn columns(self) -> Vec<ArrayRef> {
        vec![
            Arc::new(StringArray::from(self.strings)),
            Arc::new(Int64Array::from(self.integers)),
            Arc::new(Float64Array::from(self.floats)),
            Arc::new(BooleanArray::from(self.booleans)),
            Arc::new(TimestampMicrosecondArray::from(self.times).with_timezone("UTC")),
        ]
    }
}

/// Read an OCEL 2.0 log, choosing the format by file extension
/// (`.json`/`.jsonocel`, `.sqlite`/`.db`, `.xml`/`.xmlocel`).
#[pyfunction]
fn read(path: PathBuf) -> PyResult<OcelLog> {
    Ok(OcelLog {
        inner: ocel::io::read_path(path).map_err(io_err)?,
    })
}

/// Read an OCEL 2.0 JSON file.
#[pyfunction]
fn read_json(path: PathBuf) -> PyResult<OcelLog> {
    Ok(OcelLog {
        inner: json::read_path(path).map_err(io_err)?,
    })
}

/// Read an OCEL 2.0 `SQLite` file.
#[pyfunction]
fn read_sqlite(path: PathBuf) -> PyResult<OcelLog> {
    Ok(OcelLog {
        inner: sqlite::read_path(path).map_err(io_err)?,
    })
}

/// Read an OCEL 2.0 XML file.
#[pyfunction]
fn read_xml(path: PathBuf) -> PyResult<OcelLog> {
    Ok(OcelLog {
        inner: xml::read_path(path).map_err(io_err)?,
    })
}

#[pymethods]
impl OcelLog {
    /// Number of events.
    #[getter]
    fn num_events(&self) -> usize {
        self.inner.events.len()
    }

    /// Number of objects.
    #[getter]
    fn num_objects(&self) -> usize {
        self.inner.objects.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "OcelLog(events={}, objects={}, event_types={}, object_types={})",
            self.inner.events.len(),
            self.inner.objects.len(),
            self.inner.event_types.len(),
            self.inner.object_types.len(),
        )
    }

    /// Write as OCEL 2.0 JSON.
    fn write_json(&self, path: PathBuf) -> PyResult<()> {
        json::write_path(&self.inner, path).map_err(io_err)
    }

    /// Write as OCEL 2.0 `SQLite`.
    fn write_sqlite(&self, path: PathBuf) -> PyResult<()> {
        sqlite::write_path(&self.inner, path).map_err(io_err)
    }

    /// Write as OCEL 2.0 XML.
    fn write_xml(&self, path: PathBuf) -> PyResult<()> {
        xml::write_path(&self.inner, path).map_err(io_err)
    }

    /// Spec-conformance violations as human-readable strings (empty = valid).
    fn validate(&self) -> Vec<String> {
        ocel::validate::validate(&self.inner)
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    /// Event columns: `{"id": [...], "type": [...], "time": [...]}`.
    fn events<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let cols = self.inner.event_columns();
        let dict = PyDict::new(py);
        dict.set_item("id", cols.ids)?;
        dict.set_item("type", cols.types)?;
        dict.set_item("time", cols.times)?;
        Ok(dict)
    }

    /// Event attributes in long format:
    /// `{"event_id": [...], "name": [...], "value": [...]}`.
    fn event_attributes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let mut event_ids = Vec::new();
        let mut names = Vec::new();
        let mut values = Vec::new();
        for event in &self.inner.events {
            for attr in &event.attributes {
                event_ids.push(event.id.as_str());
                names.push(attr.name.as_str());
                values.push(attr_to_py(py, &attr.value)?);
            }
        }
        let dict = PyDict::new(py);
        dict.set_item("event_id", event_ids)?;
        dict.set_item("name", names)?;
        dict.set_item("value", values)?;
        Ok(dict)
    }

    /// Object columns: `{"id": [...], "type": [...]}`.
    fn objects<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(
            "id",
            self.inner
                .objects
                .iter()
                .map(|o| o.id.as_str())
                .collect::<Vec<_>>(),
        )?;
        dict.set_item(
            "type",
            self.inner
                .objects
                .iter()
                .map(|o| o.object_type.as_str())
                .collect::<Vec<_>>(),
        )?;
        Ok(dict)
    }

    /// Dynamic object attributes in long format:
    /// `{"object_id": [...], "name": [...], "value": [...], "time": [...]}`.
    fn object_attributes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let mut object_ids = Vec::new();
        let mut names = Vec::new();
        let mut values = Vec::new();
        let mut times = Vec::new();
        for object in &self.inner.objects {
            for attr in &object.attributes {
                object_ids.push(object.id.as_str());
                names.push(attr.name.as_str());
                values.push(attr_to_py(py, &attr.value)?);
                times.push(attr.time);
            }
        }
        let dict = PyDict::new(py);
        dict.set_item("object_id", object_ids)?;
        dict.set_item("name", names)?;
        dict.set_item("value", values)?;
        dict.set_item("time", times)?;
        Ok(dict)
    }

    /// `E2O` relations: `{"event_id": [...], "object_id": [...], "qualifier": [...]}`.
    fn relations<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let mut event_ids = Vec::new();
        let mut object_ids = Vec::new();
        let mut qualifiers = Vec::new();
        for rel in self.inner.e2o() {
            event_ids.push(rel.event_id);
            object_ids.push(rel.object_id);
            qualifiers.push(rel.qualifier);
        }
        let dict = PyDict::new(py);
        dict.set_item("event_id", event_ids)?;
        dict.set_item("object_id", object_ids)?;
        dict.set_item("qualifier", qualifiers)?;
        Ok(dict)
    }

    /// `O2O` relations: `{"source_id": [...], "target_id": [...], "qualifier": [...]}`.
    fn o2o<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let mut source_ids = Vec::new();
        let mut target_ids = Vec::new();
        let mut qualifiers = Vec::new();
        for rel in self.inner.o2o() {
            source_ids.push(rel.source_id);
            target_ids.push(rel.target_id);
            qualifiers.push(rel.qualifier);
        }
        let dict = PyDict::new(py);
        dict.set_item("source_id", source_ids)?;
        dict.set_item("target_id", target_ids)?;
        dict.set_item("qualifier", qualifiers)?;
        Ok(dict)
    }

    /// Keep events of the given types (and the objects they reference).
    fn filter_event_types(&self, names: Vec<String>) -> OcelLog {
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        OcelLog {
            inner: self.inner.filter_event_types(&names),
        }
    }

    /// Keep objects of the given types (and the events still related to them).
    fn filter_object_types(&self, names: Vec<String>) -> OcelLog {
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        OcelLog {
            inner: self.inner.filter_object_types(&names),
        }
    }

    /// Keep the first `n` connected components (deterministic).
    fn sample_components(&self, n: usize) -> OcelLog {
        OcelLog {
            inner: self.inner.sample_components(n),
        }
    }

    /// Connected components of the object interaction graph, as lists of
    /// object ids.
    fn connected_components<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let graph = self.inner.object_graph();
        PyList::new(py, graph.connected_components())
    }

    /// Events as an Arrow table: `id`, `type`, `time` (timestamp[us, UTC]).
    fn events_arrow(&self) -> PyResult<PyTable> {
        let cols = self.inner.event_columns();
        table(
            vec![
                utf8_field("id"),
                utf8_field("type"),
                Field::new("time", ts_type(), false),
            ],
            vec![
                utf8_col(cols.ids),
                utf8_col(cols.types),
                ts_col(
                    cols.times
                        .iter()
                        .map(chrono::DateTime::timestamp_micros)
                        .collect(),
                ),
            ],
        )
    }

    /// Objects as an Arrow table: `id`, `type`.
    fn objects_arrow(&self) -> PyResult<PyTable> {
        table(
            vec![utf8_field("id"), utf8_field("type")],
            vec![
                utf8_col(self.inner.objects.iter().map(|o| o.id.as_str())),
                utf8_col(self.inner.objects.iter().map(|o| o.object_type.as_str())),
            ],
        )
    }

    /// `E2O` relations as an Arrow table: `event_id`, `object_id`, `qualifier`.
    fn relations_arrow(&self) -> PyResult<PyTable> {
        let mut event_ids = Vec::new();
        let mut object_ids = Vec::new();
        let mut qualifiers = Vec::new();
        for rel in self.inner.e2o() {
            event_ids.push(rel.event_id);
            object_ids.push(rel.object_id);
            qualifiers.push(rel.qualifier);
        }
        table(
            vec![
                utf8_field("event_id"),
                utf8_field("object_id"),
                utf8_field("qualifier"),
            ],
            vec![
                utf8_col(event_ids),
                utf8_col(object_ids),
                utf8_col(qualifiers),
            ],
        )
    }

    /// `O2O` relations as an Arrow table: `source_id`, `target_id`, `qualifier`.
    fn o2o_arrow(&self) -> PyResult<PyTable> {
        let mut source_ids = Vec::new();
        let mut target_ids = Vec::new();
        let mut qualifiers = Vec::new();
        for rel in self.inner.o2o() {
            source_ids.push(rel.source_id);
            target_ids.push(rel.target_id);
            qualifiers.push(rel.qualifier);
        }
        table(
            vec![
                utf8_field("source_id"),
                utf8_field("target_id"),
                utf8_field("qualifier"),
            ],
            vec![
                utf8_col(source_ids),
                utf8_col(target_ids),
                utf8_col(qualifiers),
            ],
        )
    }

    /// Event attributes as an Arrow table: `event_id`, `name`, and typed
    /// value columns with nulls (`value_string` / `value_integer` /
    /// `value_float` / `value_boolean` / `value_time`).
    fn event_attributes_arrow(&self) -> PyResult<PyTable> {
        let mut event_ids = Vec::new();
        let mut names = Vec::new();
        let mut values = AttrColumns::default();
        for event in &self.inner.events {
            for attr in &event.attributes {
                event_ids.push(event.id.as_str());
                names.push(attr.name.as_str());
                values.push(&attr.value);
            }
        }
        let mut fields = vec![utf8_field("event_id"), utf8_field("name")];
        fields.extend(AttrColumns::fields());
        let mut columns = vec![utf8_col(event_ids), utf8_col(names)];
        columns.extend(values.columns());
        table(fields, columns)
    }

    /// Dynamic object attributes as an Arrow table: `object_id`, `name`,
    /// typed value columns with nulls, and `time` (timestamp[us, UTC]).
    fn object_attributes_arrow(&self) -> PyResult<PyTable> {
        let mut object_ids = Vec::new();
        let mut names = Vec::new();
        let mut values = AttrColumns::default();
        let mut times = Vec::new();
        for object in &self.inner.objects {
            for attr in &object.attributes {
                object_ids.push(object.id.as_str());
                names.push(attr.name.as_str());
                values.push(&attr.value);
                times.push(attr.time.timestamp_micros());
            }
        }
        let mut fields = vec![utf8_field("object_id"), utf8_field("name")];
        fields.extend(AttrColumns::fields());
        fields.push(Field::new("time", ts_type(), false));
        let mut columns = vec![utf8_col(object_ids), utf8_col(names)];
        columns.extend(values.columns());
        columns.push(ts_col(times));
        table(fields, columns)
    }
}

/// OCEL 2.0 event logs: read, write, validate, filter, and sample.
// The function is named ocel_py to avoid clashing with the `ocel` dependency
// crate; the Python module is still imported as `ocel` via the name attribute.
#[pymodule(name = "ocel")]
fn ocel_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<OcelLog>()?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(read_json, m)?)?;
    m.add_function(wrap_pyfunction!(read_sqlite, m)?)?;
    m.add_function(wrap_pyfunction!(read_xml, m)?)?;
    Ok(())
}
