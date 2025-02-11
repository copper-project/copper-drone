use cu29::prelude::*;
use bincode::{Decode, Encode};
use cu_gstreamer::CuGstBuffer;
use std::thread;
use std::time::Duration;
use rerun::{ColorModel, RecordingStream, RecordingStreamBuilder};
use imageproc::contrast::adaptive_threshold;
use image::GrayImage;

use apriltag::DetectorBuilder;
use apriltag::Family;

#[derive(Default, Debug, Clone, Encode, Decode)]
pub struct MyPayload {
    value: i32,
}

pub struct MyTask {
    rec: RecordingStream,
    detector: apriltag::Detector,
}

impl Freezable for MyTask {}

impl<'cl> CuTask<'cl> for MyTask {
    type Input = input_msg!('cl, CuGstBuffer);
    type Output = output_msg!('cl, MyPayload);

    fn new(_config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        let rec = RecordingStreamBuilder::new("Camera B&W Viz")
            .spawn()
            .unwrap();
        let family: Family = "tag16h5".parse().unwrap();
        let bits_corrected = 1;
        let detector = DetectorBuilder::default().add_family_bits(family, bits_corrected)
               .build().unwrap();
       Ok(Self {rec, detector})
    }

    fn process(
        &mut self,
        _clock: &RobotClock,
        input: Self::Input,
        output: Self::Output,
    ) -> CuResult<()> {
        let payload = input.payload();
        if payload.is_none() {
            thread::sleep(Duration::from_millis(100)); 
            return Ok(());
        }

        let data = input.payload().unwrap().map_readable().unwrap();
        debug!("Received Gstreamer: {}", data.len());

        let width = 1920;
        let height = 1080;
        let local_threshold_size = 107;
        let y_plane_size = width * height;
        let grey_image = &data[0..y_plane_size];

        let instant = std::time::Instant::now();
        let grey_img = GrayImage::from_raw(width as u32, height as u32, grey_image.into()).unwrap();
        let thresholded_img: GrayImage = adaptive_threshold(&grey_img, local_threshold_size);
        println!("Threashold time: {:?}", instant.elapsed());

        let instant = std::time::Instant::now();
        let mut proc_img = apriltag::Image::zeros_with_stride(width, height, width).unwrap();
        proc_img.as_slice_mut().copy_from_slice(thresholded_img.as_ref());
        let detections = self.detector.detect(&proc_img);
        println!("Detection time: {:?}", instant.elapsed());

        for detection in detections {
            if detection.decision_margin() < 200.0 {
                continue;
            }
            println!("Detection: {:?}", detection);
        }

        let image = rerun::Image::from_elements(thresholded_img.as_raw(), [width as u32, height as u32], ColorModel::L);
        self.rec.log("camera/image", &image).unwrap();
        // thread::sleep(Duration::from_millis(100));
        output.set_payload(MyPayload { value: 42 });
        Ok(())
    }
}

#[derive(Default)]
pub struct MySink {}

impl Freezable for MySink {}

impl<'cl> CuSinkTask<'cl> for MySink {
    type Input = input_msg!('cl, MyPayload);

    fn new(_config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
    // don't forget the other lifecycle methods if you need them: start, stop, preprocess, postprocess

    fn process(&mut self, _clock: &RobotClock, input: Self::Input) -> CuResult<()> {
        let payload = input.payload();
        if payload.is_none() {
            return Ok(());
        }
        debug!("Sink Received message: {}", payload.unwrap().value);
        Ok(())
    }
}
