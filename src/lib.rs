use std::{sync::Mutex, time::Duration};

use pyo3::{exceptions::PyIOError, prelude::*};

use ::rustypot::{self as r, device::feetech_sts3215};

#[pyclass]
struct IO {
    io: r::DynamixelSerialIO,
    serial_port: Mutex<Box<dyn serialport::SerialPort>>,
}

#[pymethods]
impl IO {
    fn read_pos(&self, ids: Vec<u8>) -> PyResult<Vec<i16>> {
        let mut serial_port = self.serial_port.lock().unwrap();
        feetech_sts3215::sync_read_present_position(&self.io, serial_port.as_mut(), &ids)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
}

struct SerialportError(serialport::Error);
impl From<SerialportError> for PyErr {
    fn from(error: SerialportError) -> Self {
        PyIOError::new_err(error.0.to_string())
    }
}

#[pyfunction]
fn feetech(serialportname: &str, baudrate: u32) -> PyResult<IO> {
    let serial_port = serialport::new(serialportname, baudrate)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| SerialportError(e))?;
    let serial_port = Mutex::new(serial_port);

    let io = r::DynamixelSerialIO::feetech();

    Ok(IO { io, serial_port })
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustypot(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(feetech, m)?)?;
    m.add_class::<IO>()?;

    Ok(())
}
