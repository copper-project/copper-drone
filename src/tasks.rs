use cu29::prelude::*;
use cu_apriltag::AprilTagDetections;
use cu_msp_lib::structs::MspRequest;
use cu_msp_sink::MspRequestBatch;
use cu_msp_src::MspResponseBatch;

pub struct DroneControl {
}

impl Freezable for DroneControl {}

impl<'cl> CuTask<'cl> for DroneControl {
    type Input = input_msg!('cl, MspResponseBatch, AprilTagDetections);
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
        let (maybe_batch, maybe_detections) = input;
        if let Some(batch) = maybe_batch.payload() {
            debug!("Received response batch: {}", batch);
        }
        if let Some(detections) = maybe_detections.payload() {
            debug!("Received detections: {}", detections);
        }
        let mut batch = MspRequestBatch::new();
        batch.push(MspRequest::MspBatteryState);
        output.set_payload(batch);
        Ok(()) // outputs another message for downstream
    }
}

