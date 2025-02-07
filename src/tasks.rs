use cu29::prelude::*;
use bincode::{Decode, Encode};
use cu_gstreamer::CuGstBuffer;

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

    // don't forget the other lifecycle methods if you need them: start, stop, preprocess, postprocess

    fn process(
        &mut self,
        _clock: &RobotClock,
        input: Self::Input,
        output: Self::Output,
    ) -> CuResult<()> {
        println!("Received Gstreamer: {:?}", input.payload().unwrap());
        output.set_payload(MyPayload { value: 42 });
        Ok(()) // outputs another message for downstream
    }
}

// Defines a sink (ie. actualtion)
#[derive(Default)]
pub struct MySink {}

// Needs to be fully implemented if you want to have a stateful task.
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
        debug!("Sink Received message: {}", input.payload().unwrap().value);
        Ok(())
    }
}
