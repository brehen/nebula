use plotters::{prelude::*, style::full_palette::PURPLE};

use super::efficiency_statistics::EfficiencyMetrics;
const OUT_FILE_NAME: &str = "images/sample.png";
pub fn draw_metrics(
    docker_metrics: Vec<(u32, EfficiencyMetrics)>,
    wasm_metrics: Vec<(u32, EfficiencyMetrics)>,
) -> anyhow::Result<()> {
    println!("{:?}", wasm_metrics.first().unwrap());

    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1000, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .x_label_area_size(75)
        .y_label_area_size(75)
        .right_y_label_area_size(50)
        .caption("Efficiency Metrics", ("sans-serif", 40))
        .build_cartesian_2d(0u32..600u32, 0.0..1000.0)?;

    // Add padding
    //
    ctx.configure_mesh().draw()?;

    ctx.draw_series(LineSeries::new(
        docker_metrics
            .iter()
            .map(|(x, metrics)| (*x, metrics.startup_time.median / 1000.0)),
        &BLUE,
    ))
    .unwrap();

    // Docker total_runtime mean
    ctx.draw_series(LineSeries::new(
        docker_metrics
            .iter()
            .map(|(x, metrics)| (*x, metrics.total_runtime.median / 1000.0)),
        &BLUE.mix(0.5),
    ))
    .unwrap();

    // WASM startup_time mean
    ctx.draw_series(LineSeries::new(
        wasm_metrics
            .iter()
            .map(|(x, metrics)| (*x, metrics.startup_time.median / 1000.0)),
        &PURPLE,
    ))
    .unwrap();

    // WASM total_runtime mean
    ctx.draw_series(LineSeries::new(
        wasm_metrics
            .iter()
            .map(|(x, metrics)| (*x, metrics.total_runtime.median / 1000.0)),
        &PURPLE.mix(0.5),
    ))
    .unwrap();

    Ok(())
}
