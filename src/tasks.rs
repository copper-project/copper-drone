use cu29::prelude::*;
use cu_apriltag::AprilTagDetections;
use cu_msp_lib::structs::{MspRc, MspRequest, MspResponse};
use cu_msp_sink::MspRequestBatch;
use cu_msp_src::MspResponseBatch;
use cu_pid::PIDController;
use std::time::Duration;

pub struct DroneControl {
    pid: PIDController,
    first_run: bool,
    last_tov: CuTime,
}

impl Freezable for DroneControl {}

impl<'cl> CuTask<'cl> for DroneControl {
    type Input = input_msg!('cl, MspResponseBatch, AprilTagDetections);
    type Output = output_msg!('cl, MspRequestBatch);

    fn new(_config: Option<&ComponentConfig>) -> CuResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            pid: PIDController::new(
                70.0, // kp: max ±20 PWM at full ±0.4m error
                0.5,  // ki: very slow integral to adjust bias over time
                20.0, // kd: mild damping to prevent overshoot
                0.0,  // setpoint: target zero offset from tag
                40.0, // p_limit: proportional max output cap
                10.0, // i_limit: integral cap
                20.0, // d_limit: derivative cap
                40.0, // output_limit: total throttle correction max ±40
                CuDuration::from(Duration::from_millis(50))), // 20 Hz update rate
            // PID stuff
                first_run: true,
                last_tov: CuTime::default(),
        })
    }

    fn process(
        &mut self,
        _clock: &RobotClock,
        input: Self::Input,
        output: Self::Output,
    ) -> CuResult<()> {
        const MID_THROTTLE: f32 = 1280.0;

        let mut batch = MspRequestBatch::new();

        let mut cell_voltage = 0.0;

        let (maybe_batch, maybe_detections) = input;
        if let Some(batch) = maybe_batch.payload() {
            for response in batch.0.iter() {
                match response {
                    MspResponse::MspBatteryState(state) => {
                        cell_voltage = (state.battery_voltage as f32/ state.battery_cell_count as f32) / 10.0;
                    }
                    _ => {}
                }
            }
        }

        let mut throttle = MID_THROTTLE;

        if let Some(detections) = maybe_detections.payload() {

            let tov = match maybe_detections.metadata.tov {
                Tov::Time(single) => single,
                _ => return Err("Unexpected variant for a TOV of PID".into()),
            };
            for detection in detections.ids.0.iter().enumerate() {
                if *detection.1 == 4 {
                    let pose = &detections.poses.0[detection.0];
                    let [_x, y, _z] = pose.translation();
                    // println!("Y = {:?}", y);

                    if self.first_run {
                        self.first_run = false;
                        self.last_tov = tov;
                        self.pid.init_measurement(y.value);
                    }
                    else {
                        let dt = tov - self.last_tov;
                        self.last_tov = tov;
                        let output = self.pid.next_control_output(y.value, dt);
                        throttle = output.output + MID_THROTTLE;
                    }

                    // let output = self.pid.update();
                    // println!("PID output: {:?}", output);
                }
            }
        };
        println!("Cell: {:3} --- Thr {:5}", cell_voltage, throttle);
        let mut rc = MspRc::new();
        rc.set_throttle(throttle as u16);
        batch.push(MspRequest::MspSetRawRc(rc));

        batch.push(MspRequest::MspBatteryState);
        output.set_payload(batch);
        Ok(()) // outputs another message for downstream
    }
}
