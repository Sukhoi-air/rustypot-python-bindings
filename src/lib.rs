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
    fn read_present_position(&self, ids: Vec<u8>) -> PyResult<Vec<f64>> {
        let mut serial_port = self.serial_port.lock().unwrap();
        feetech_sts3215::sync_read_present_position(&self.io, serial_port.as_mut(), &ids)
            .map(|pos| {
                pos.into_iter()
                    .map(feetech_sts3215::conv::dxl_pos_to_radians)
                    .collect()
            })
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    fn write_goal_position(&self, ids: Vec<u8>, goal_position: Vec<f64>) -> PyResult<()> {
        let mut serial_port = self.serial_port.lock().unwrap();

        let goal_position: Vec<i16> = goal_position
            .into_iter()
            .map(feetech_sts3215::conv::radians_to_dxl_pos)
            .collect();

        feetech_sts3215::sync_write_goal_position(
            &self.io,
            serial_port.as_mut(),
            &ids,
            &goal_position,
        )
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    // fn read_present_velocity(&self, ids: Vec<u8>) -> PyResult<Vec<u16>> {
    //     let mut serial_port = self.serial_port.lock().unwrap();
    //     feetech_sts3215::sync_read_present_speed(&self.io, serial_port.as_mut(), &ids)
    //         .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    // }

    fn set_mode(&self, ids: Vec<u8>, mode: u8) -> PyResult<()> {
        let mut serial_port = self.serial_port.lock().unwrap();

        feetech_sts3215::sync_write_mode(
            &self.io,
            serial_port.as_mut(),
            &ids,
            &vec![mode; ids.len()],
        )
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    fn enable_torque(&self, ids: Vec<u8>) -> PyResult<()> {
        let mut serial_port = self.serial_port.lock().unwrap();

        feetech_sts3215::sync_write_torque_enable(
            &self.io,
            serial_port.as_mut(),
            &ids,
            &vec![true as u8; ids.len()],
        )
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    fn disable_torque(&self, ids: Vec<u8>) -> PyResult<()> {
        let mut serial_port = self.serial_port.lock().unwrap();

        feetech_sts3215::sync_write_torque_enable(
            &self.io,
            serial_port.as_mut(),
            &ids,
            &vec![false as u8; ids.len()],
        )
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    fn set_goal_time(&self, ids: Vec<u8>, goal_time: Vec<u16>) -> PyResult<()> {
        let mut serial_port = self.serial_port.lock().unwrap();

        feetech_sts3215::sync_write_goal_time(&self.io, serial_port.as_mut(), &ids, &goal_time)
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
        .map_err(SerialportError)?;
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
