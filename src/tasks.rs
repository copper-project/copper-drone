use cu29::prelude::*;
use cu_msp_lib::structs::MspRequest;
use cu_msp_sink::MspRequestBatch;
use cu_msp_src::MspResponseBatch;

pub struct DroneControl {
}

impl Freezable for DroneControl {}

impl<'cl> CuTask<'cl> for DroneControl {
    type Input = input_msg!('cl, MspResponseBatch);
    type Output = output_msg!('cl, MspRequestBatch);

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
        if let Some(batch) = input.payload() {
            debug!("Received response batch: {}", batch);
        }
        let mut batch = MspRequestBatch::new();
        batch.push(MspRequest::MspBatteryState);
        output.set_payload(batch);
        Ok(()) // outputs another message for downstream
    }
}

