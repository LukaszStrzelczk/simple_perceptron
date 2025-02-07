use plotters::prelude::*;

fn main() {}

fn fake_data() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plots/plot1.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("test plot", ("sans-seriff", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        (-50..=50).map
    ))
    
    Ok(())
}
