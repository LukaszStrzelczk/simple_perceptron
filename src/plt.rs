pub mod plot {
    use plotters::prelude::*;
    #[allow(dead_code)]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("plots/plot1.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("test plot", ("sans-seriff", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
        chart.configure_mesh().draw()?;
        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))?
            .label("y=x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;
        root.present()?;
        Ok(())
    }

    pub fn scatter_plot(
        data: &[(f32, f32, i8)],
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("plots/{}.png", path);
        let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Scatter_plot", ("sans_serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0.0..1.0f32, 0.0..1.0f32)?;
        chart.configure_mesh().draw()?;
        let mut color;
        for (x,y,label) in data{
            if *label == 1{
                color = RED;
            }else {
                color = BLUE;
            }  
        
            chart.draw_series(std::iter::once(Circle::new((*x,*y),3,color.filled())))?;
        }
        root.present()?;
        Ok(())
    }
}
