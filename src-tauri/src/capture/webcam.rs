use serde::{Deserialize, Serialize};
use windows::{
    Devices::Enumeration::{DeviceClass, DeviceInformation},
    Media::Capture::MediaCapture,
};

#[derive(thiserror::Error, Debug)]
pub enum WebcamError {
    #[error("failed to list webcams")]
    ListWebcams(#[source] windows::core::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webcam {
    pub name: String,
    pub id: String,
}

pub async fn get_webcams() -> Result<Vec<Webcam>, WebcamError> {
    let devices = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::VideoCapture)
        .map_err(WebcamError::ListWebcams)?
        .await
        .map_err(WebcamError::ListWebcams)?;

    let mut webcams = Vec::new();

    for dev in devices {
        if !dev.IsEnabled().unwrap_or(false) {
            continue;
        }

        let result: windows::core::Result<Webcam> = (|| {
            let name = dev.Name()?;
            let id = dev.Id()?;

            Ok(Webcam {
                name: name.to_string(),
                id: id.to_string(),
            })
        })();

        match result {
            Ok(cam) => webcams.push(cam),
            Err(err) => {
                eprintln!(
                    "get_webcams: skipping device {:?} due to error: {}",
                    dev.Id().ok(),
                    err
                );
            }
        }
    }

    Ok(webcams)
}
