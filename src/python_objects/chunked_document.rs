use plotters::prelude::*;
use pyo3::prelude::*;
use pyo3::exceptions::{PyIndexError, PyRuntimeError};

use crate::python_objects::Chunk;


/// chunked document contains everything a muzungu needs to recieve their chunks.
#[pyclass]
#[derive(Clone)]
pub struct ChunkedDocument {
    #[pyo3(get)]
    pub chunks: Vec<Chunk>,

    #[pyo3(get)]
    pub(crate) punishments: Vec<usize>, // this is used for the anayliser... does it need to be public? does it help if it is??
}


#[pymethods]
impl ChunkedDocument {
    fn __repr__(&self) -> String {
        format!(
            "ChunkedDocument(chunk_count={})",
            self.chunks.len()
        )
    }

    /// create an SVG displaying the punishment function, along with its selected boundaries
    /// useful when picking rules for showing why darn has selected certain start / end points.
    /// currently does not provide a per-rule breakdown - it is the full punishment only
    /// I did not want to learn to write graphing code so i used claude for this one and oh boy does it show
    /// insanely verbose no?
    pub fn analyse(&self, chunk_idx: usize) -> PyResult<String> {
        if chunk_idx >= self.chunks.len() {
            return Err(PyIndexError::new_err(format!(
                "chunk_index {chunk_idx} is out of range \
                 (document has {} chunks)",
                self.chunks.len()
            )));
        }

        let lo = chunk_idx.saturating_sub(1);
        let hi = (chunk_idx + 1).min(self.chunks.len() - 1);
        let window = &self.chunks[lo..=hi];

        let range_start = window.first().unwrap().start_index;

        let range_end = window
            .last()
            .unwrap()
            .end_index
            .min(self.punishments.len());

        let data: Vec<(usize, usize)> = (range_start..range_end)
            .map(|i| (i, self.punishments[i]))
            .collect();

        let y_max = data.iter().map(|&(_, y)| y).max().unwrap_or(0);
        let y_upper = y_max.saturating_add((y_max / 10).max(1));
        let vlines: Vec<usize> = window.iter().map(|c| c.start_index).collect();

        let mut svg = String::new();
        {
            let root =
                SVGBackend::with_string(&mut svg, (960, 480)).into_drawing_area();

            root.fill(&WHITE)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

            let mut chart = ChartBuilder::on(&root)
                .caption(
                    format!("Punishments — chunk {chunk_idx} in context"),
                    ("sans-serif", 18),
                )
                .margin(30)
                .x_label_area_size(45)
                .y_label_area_size(60)
                // Axes are derived entirely from the data; no hard-coded scale.
                .build_cartesian_2d(range_start..range_end, 0usize..y_upper)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

            chart
                .configure_mesh()
                .x_desc("Document index")
                .y_desc("Punishment")
                .draw()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

            // Vertical lines at each chunk boundary — one LineSeries per line
            // so that each sits independently in the data coordinate space.
            for &x in &vlines {
                chart
                    .draw_series(LineSeries::new(
                        [(x, 0usize), (x, y_upper)],
                        ShapeStyle::from(&RED.mix(0.6)).stroke_width(2),
                    ))
                    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            }

            chart
                .draw_series(LineSeries::new(
                    data.iter().copied(),
                    ShapeStyle::from(&BLUE).stroke_width(2),
                ))
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
                .label("Punishment")
                .legend(|(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], BLUE)
                });

            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.85))
                .border_style(BLACK)
                .draw()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

            root.present()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        }

        Ok(svg)
    }
}