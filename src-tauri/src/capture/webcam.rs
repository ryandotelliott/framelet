use windows::{
    Devices::Enumeration::{DeviceClass, DeviceInformation},
    Media::Capture::MediaCapture,
};

#[derive(thiserror::Error, Debug)]
pub enum WebcamError {
    #[error("failed to list webcams")]
    ListWebcams(#[source] windows::core::Error),
}

pub struct Webcam {
    pub name: String,
    pub id: String,
    pub width: u32,
    pub height: u32,
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

            let profiles = MediaCapture::FindAllVideoProfiles(&id)?;

            let largest = profiles
                .into_iter()
                .flat_map(|profile| profile.SupportedRecordMediaDescription().into_iter())
                .flatten()
                .max_by_key(|desc| desc.Width().unwrap_or(0) * desc.Height().unwrap_or(0))
                .ok_or_else(|| {
                    windows::core::Error::new(windows::core::HRESULT(0), "no media profiles")
                })?;

            let width = largest.Width()?;
            let height = largest.Height()?;

            Ok(Webcam {
                name: name.to_string(),
                id: id.to_string(),
                width,
                height,
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
