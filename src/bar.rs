const STEPS: [char; 9] = ['⠀', '⡀', '⡄', '⡆', '⡇', '⣇', '⣧', '⣷', '⣿'];
const WIDTH: usize = 20;

pub fn render(pct: f64) -> String {
    let pct = pct.clamp(0.0, 100.0);
    let filled_eighths = (pct * WIDTH as f64 * 8.0 / 100.0) as usize;
    let full = (filled_eighths / 8).min(WIDTH);
    let partial = filled_eighths % 8;

    let mut bar = String::with_capacity(WIDTH * 3);
    for _ in 0..full {
        bar.push(STEPS[8]);
    }
    let remaining = WIDTH - full;
    if remaining > 0 {
        if partial > 0 {
            bar.push(STEPS[partial]);
            for _ in 0..(remaining - 1) {
                bar.push(STEPS[0]);
            }
        } else {
            for _ in 0..remaining {
                bar.push(STEPS[0]);
            }
        }
    }
    bar
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(s: &str, c: char) -> usize {
        s.chars().filter(|&x| x == c).count()
    }

    // (pct, expected_full_cells, expected_empty_cells)
    #[test]
    fn bar_render_matrix() {
        let cases: &[(f64, usize, usize)] = &[
            // --- clamping ---
            (-5.0, 0, WIDTH),
            (150.0, WIDTH, 0),
            // --- boundaries ---
            (0.0, 0, WIDTH),
            (100.0, WIDTH, 0),
            // --- exact cell boundaries ---
            (5.0, 1, 19), // 5% of 20 = 1 full cell (8 eighths)
            (25.0, 5, 15),
            (50.0, 10, 10),
            (75.0, 15, 5),
            // --- near-boundaries with partial step ---
            (0.1, 0, 20),  // 0.16 eighths truncates to 0 - all empty
            (99.9, 19, 0), // almost full: 19 full + 1 partial
            (12.5, 2, 17), // 2 full + 1 partial + 17 empty
        ];
        for &(pct, full, empty) in cases {
            let bar = render(pct);
            assert_eq!(bar.chars().count(), WIDTH, "width at {pct}%");
            assert_eq!(count(&bar, STEPS[8]), full, "full cells at {pct}%");
            assert_eq!(count(&bar, STEPS[0]), empty, "empty cells at {pct}%");
        }
    }
}
