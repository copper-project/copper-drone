use cu29::prelude::*;
use bincode::{Decode, Encode};
use cu_gstreamer::CuGstBuffer;
use std::thread;
use std::time::Duration;
use rerun::{ChannelDatatype, ColorModel, Image, RecordingStream, RecordingStreamBuilder};

#[derive(Default, Debug, Clone, Encode, Decode)]
pub struct MyPayload {
    value: i32,
}

pub struct MyTask {
    rec: RecordingStream,
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
            .connect_tcp_opts(std::net::SocketAddr::from(([192, 168, 1, 181], 9876)), None)
            .unwrap();
        Ok(Self {rec})
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
        let y_plane_size = width * height;
        let grey_image = &data[0..y_plane_size];

        let image = Image::from_color_model_and_bytes(
            grey_image.to_vec(),
            [width as u32, height as u32],
            ColorModel::L,
            ChannelDatatype::U8,
        );
        self.rec.log("camera/image", &image).unwrap();
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
