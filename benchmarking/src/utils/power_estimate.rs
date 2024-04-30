use super::{modbus::SensorData, request::FunctionResult};

// Measured on Raspberry Pi over a given duration without any load, to estimate an average power
// consumption to subtract from each invocation.
// const BASELINE_IDLE_OLD: f32 = 2.4703212;
const BASELINE_IDLE: f32 = 2.4742324;

pub fn associate_power_measurements(
    function_results: Vec<FunctionResult>,
    energy_data: &Vec<SensorData>,
) -> Vec<FunctionResult> {
    let mut processed_results = Vec::new();

    for mut function_result in function_results {
        let start_time = function_result.metrics.as_ref().unwrap().start_since_epoch;
        let end_time = function_result.metrics.as_ref().unwrap().end_since_epoch;

        let mut total_power = 0.0;
        let mut total_isolated_power = 0.0;
        let mut num_readings = 0;

        for data in energy_data {
            if data.start_read >= start_time && data.end_read < end_time {
                total_power += data.power;
                total_isolated_power += data.power - BASELINE_IDLE;
                num_readings += 1;
            }
        }

        let avg_power = if num_readings > 0 {
            total_power / num_readings as f32
        } else {
            0.0
        };

        let avg_isolated_power = if num_readings > 0 {
            total_isolated_power / num_readings as f32
        } else {
            0.0
        };

        let duration = end_time - start_time;
        let energy_consumption_wh = (avg_power * duration as f32) / 3_600_000_000.0;
        let energy_consumption_isolated_wh =
            (avg_isolated_power * duration as f32) / 3_600_000_000.0;

        if total_isolated_power > 0.0 {
            function_result.metrics.as_mut().unwrap().average_power = Some(avg_power);
            function_result
                .metrics
                .as_mut()
                .unwrap()
                .energy_consumption_wh = Some(energy_consumption_wh);
            function_result
                .metrics
                .as_mut()
                .unwrap()
                .average_power_isolated = Some(avg_isolated_power);
            function_result
                .metrics
                .as_mut()
                .unwrap()
                .energy_consumption_isolated_wh = Some(energy_consumption_isolated_wh);

            processed_results.push(function_result);
        }
    }

    processed_results
}
