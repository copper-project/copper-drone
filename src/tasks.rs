use cu29::prelude::*;
use bincode::{Decode, Encode};
use cu_gstreamer::CuGstBuffer;
use std::thread;
use std::time::Duration;


#[derive(Default, Debug, Clone, Encode, Decode)]
pub struct MyPayload {
    value: i32,
}

pub struct MyTask {}

impl Freezable for MyTask {}

impl<'cl> CuTask<'cl> for MyTask {
    type Input = input_msg!('cl, CuGstBuffer);
    type Output = output_msg!('cl, MyPayload);

    fn new(_config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
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
        println!("Received Gstreamer: {:?}", payload.unwrap());
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
