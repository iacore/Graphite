use super::*;
use crate::consts::MIN_SEPERATION_VALUE;
use crate::ComputeType;

use glam::DVec2;

impl Subpath {
	/// Calculate the point on the subpath based on the parametric `t`-value provided.
	/// Expects `t` to be within the inclusive range `[0, 1]`.
	pub fn evaluate(&self, t: ComputeType) -> DVec2 {
		match t {
			ComputeType::Parametric(t) => {
				assert!((0.0..=1.).contains(&t));

				if let (Some(curve), target_curve_t) = self.find_curve_parametric(t) {
					curve.evaluate(ComputeType::Parametric(target_curve_t))
				} else {
					self.iter().last().unwrap().evaluate(ComputeType::Parametric(1.))
				}
			}
			// TODO: change this implementation to Euclidean compute
			ComputeType::Euclidean(_t) => self.iter().next().unwrap().evaluate(ComputeType::Parametric(0.)),
			ComputeType::EuclideanWithinError { t: _, epsilon: _ } => todo!(),
		}
	}

	/// Calculates the intersection points the subpath has with a given line and returns a list of parameteric `t`-values.
	/// This function expects the following:
	/// - other: a [Bezier] curve to check intersections against
	/// - error: an optional f64 value to provide an error bound
	pub fn intersections(&self, other: &Bezier, error: Option<f64>, minimum_seperation: Option<f64>) -> Vec<f64> {
		// TODO: account for either euclidean or parametric type
		let number_of_curves = self.len_segments() as f64;
		let intersection_t_values: Vec<f64> = self
			.iter()
			.enumerate()
			.flat_map(|(index, bezier)| {
				bezier
					.intersections(other, error, minimum_seperation)
					.into_iter()
					.map(|t| ((index as f64) + t) / number_of_curves)
					.collect::<Vec<f64>>()
			})
			.collect();

		intersection_t_values.iter().fold(Vec::new(), |mut accumulator, t| {
			if !accumulator.is_empty() && (accumulator.last().unwrap() - t).abs() < minimum_seperation.unwrap_or(MIN_SEPERATION_VALUE) {
				accumulator.pop();
			}
			accumulator.push(*t);
			accumulator
		});

		intersection_t_values
	}

	pub fn tangent(&self, t: ComputeType) -> DVec2 {
		match t {
			ComputeType::Parametric(t) => {
				assert!((0.0..=1.).contains(&t));

				if let (Some(curve), target_curve_t) = self.find_curve_parametric(t) {
					curve.tangent(target_curve_t)
				} else {
					self.iter().last().unwrap().tangent(1.)
				}
			}
			ComputeType::Euclidean(_t) => unimplemented!(),
			ComputeType::EuclideanWithinError { t: _, epsilon: _ } => todo!(),
		}
	}

	pub fn normal(&self, t: ComputeType) -> DVec2 {
		match t {
			ComputeType::Parametric(t) => {
				assert!((0.0..=1.).contains(&t));

				if let (Some(curve), target_curve_t) = self.find_curve_parametric(t) {
					curve.normal(target_curve_t)
				} else {
					self.iter().last().unwrap().normal(1.)
				}
			}
			ComputeType::Euclidean(_t) => unimplemented!(),
			ComputeType::EuclideanWithinError { t: _, epsilon: _ } => todo!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Bezier;
	use glam::DVec2;

	use crate::consts::MAX_ABSOLUTE_DIFFERENCE;
	use crate::utils;

	fn normalize_t(n: i64, t: f64) -> f64 {
		t * (n as f64) % 1.
	}

	#[test]
	fn evaluate_one_subpath_curve() {
		let start = DVec2::new(20., 30.);
		let end = DVec2::new(60., 45.);
		let handle = DVec2::new(75., 85.);

		let bezier = Bezier::from_quadratic_dvec2(start, handle, end);
		let subpath = Subpath::new(
			vec![
				ManipulatorGroup {
					anchor: start,
					in_handle: None,
					out_handle: Some(handle),
				},
				ManipulatorGroup {
					anchor: end,
					in_handle: None,
					out_handle: Some(handle),
				},
			],
			false,
		);

		let t0 = 0.;
		assert_eq!(subpath.evaluate(ComputeType::Parametric(t0)), bezier.evaluate(ComputeType::Parametric(t0)));

		let t1 = 0.25;
		assert_eq!(subpath.evaluate(ComputeType::Parametric(t1)), bezier.evaluate(ComputeType::Parametric(t1)));

		let t2 = 0.50;
		assert_eq!(subpath.evaluate(ComputeType::Parametric(t2)), bezier.evaluate(ComputeType::Parametric(t2)));

		let t3 = 1.;
		assert_eq!(subpath.evaluate(ComputeType::Parametric(t3)), bezier.evaluate(ComputeType::Parametric(t3)));
	}

	#[test]
	fn evaluate_multiple_subpath_curves() {
		let start = DVec2::new(20., 30.);
		let middle = DVec2::new(70., 70.);
		let end = DVec2::new(60., 45.);
		let handle1 = DVec2::new(75., 85.);
		let handle2 = DVec2::new(40., 30.);
		let handle3 = DVec2::new(10., 10.);

		let linear_bezier = Bezier::from_linear_dvec2(start, middle);
		let quadratic_bezier = Bezier::from_quadratic_dvec2(middle, handle1, end);
		let cubic_bezier = Bezier::from_cubic_dvec2(end, handle2, handle3, start);

		let mut subpath = Subpath::new(
			vec![
				ManipulatorGroup {
					anchor: start,
					in_handle: Some(handle3),
					out_handle: None,
				},
				ManipulatorGroup {
					anchor: middle,
					in_handle: None,
					out_handle: Some(handle1),
				},
				ManipulatorGroup {
					anchor: end,
					in_handle: None,
					out_handle: Some(handle2),
				},
			],
			false,
		);

		// Test open subpath

		let mut n = (subpath.len() as i64) - 1;

		let t0 = 0.;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t0)),
			linear_bezier.evaluate(ComputeType::Parametric(normalize_t(n, t0))),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		let t1 = 0.25;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t1)),
			linear_bezier.evaluate(ComputeType::Parametric(normalize_t(n, t1))),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		let t2 = 0.50;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t2)),
			quadratic_bezier.evaluate(ComputeType::Parametric(normalize_t(n, t2))),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		let t3 = 0.75;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t3)),
			quadratic_bezier.evaluate(ComputeType::Parametric(normalize_t(n, t3))),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		let t4 = 1.0;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t4)),
			quadratic_bezier.evaluate(ComputeType::Parametric(1.)),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		// Test closed subpath

		subpath.closed = true;
		n = subpath.len() as i64;

		let t5 = 2. / 3.;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t5)),
			cubic_bezier.evaluate(ComputeType::Parametric(normalize_t(n, t5))),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		let t6 = 1.;
		assert!(utils::dvec2_compare(
			subpath.evaluate(ComputeType::Parametric(t6)),
			cubic_bezier.evaluate(ComputeType::Parametric(1.)),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());
	}

	#[test]
	fn intersection_linear_multiple_subpath_curves_test_one() {
		// M 35 125 C 40 40 120 120 43 43 Q 175 90 145 150 Q 70 185 35 125 Z

		let cubic_start = DVec2::new(35., 125.);
		let cubic_handle_1 = DVec2::new(40., 40.);
		let cubic_handle_2 = DVec2::new(120., 120.);
		let cubic_end = DVec2::new(43., 43.);

		let quadratic_1_handle = DVec2::new(175., 90.);
		let quadratic_end = DVec2::new(145., 150.);

		let quadratic_2_handle = DVec2::new(70., 185.);

		let cubic_bezier = Bezier::from_cubic_dvec2(cubic_start, cubic_handle_1, cubic_handle_2, cubic_end);
		let quadratic_bezier_1 = Bezier::from_quadratic_dvec2(cubic_end, quadratic_1_handle, quadratic_end);

		let subpath = Subpath::new(
			vec![
				ManipulatorGroup {
					anchor: cubic_start,
					in_handle: None,
					out_handle: Some(cubic_handle_1),
				},
				ManipulatorGroup {
					anchor: cubic_end,
					in_handle: Some(cubic_handle_2),
					out_handle: None,
				},
				ManipulatorGroup {
					anchor: quadratic_end,
					in_handle: Some(quadratic_1_handle),
					out_handle: Some(quadratic_2_handle),
				},
			],
			true,
		);

		let line = Bezier::from_linear_coordinates(150., 150., 20., 20.);

		let cubic_intersections = cubic_bezier.intersections(&line, None, None);
		let quadratic_1_intersections = quadratic_bezier_1.intersections(&line, None, None);
		let subpath_intersections = subpath.intersections(&line, None, None);

		assert!(utils::dvec2_compare(
			cubic_bezier.evaluate(ComputeType::Parametric(cubic_intersections[0])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[0])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		assert!(utils::dvec2_compare(
			quadratic_bezier_1.evaluate(ComputeType::Parametric(quadratic_1_intersections[0])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[1])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		assert!(utils::dvec2_compare(
			quadratic_bezier_1.evaluate(ComputeType::Parametric(quadratic_1_intersections[1])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[2])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());
	}

	#[test]
	fn intersection_linear_multiple_subpath_curves_test_two() {
		// M34 107 C40 40 120 120 102 29 Q175 90 129 171 Q70 185 34 107 Z
		// M150 150 L 20 20

		let cubic_start = DVec2::new(34., 107.);
		let cubic_handle_1 = DVec2::new(40., 40.);
		let cubic_handle_2 = DVec2::new(120., 120.);
		let cubic_end = DVec2::new(102., 29.);

		let quadratic_1_handle = DVec2::new(175., 90.);
		let quadratic_end = DVec2::new(129., 171.);

		let quadratic_2_handle = DVec2::new(70., 185.);

		let cubic_bezier = Bezier::from_cubic_dvec2(cubic_start, cubic_handle_1, cubic_handle_2, cubic_end);
		let quadratic_bezier_1 = Bezier::from_quadratic_dvec2(cubic_end, quadratic_1_handle, quadratic_end);

		let subpath = Subpath::new(
			vec![
				ManipulatorGroup {
					anchor: cubic_start,
					in_handle: None,
					out_handle: Some(cubic_handle_1),
				},
				ManipulatorGroup {
					anchor: cubic_end,
					in_handle: Some(cubic_handle_2),
					out_handle: None,
				},
				ManipulatorGroup {
					anchor: quadratic_end,
					in_handle: Some(quadratic_1_handle),
					out_handle: Some(quadratic_2_handle),
				},
			],
			true,
		);

		let line = Bezier::from_linear_coordinates(150., 150., 20., 20.);

		let cubic_intersections = cubic_bezier.intersections(&line, None, None);
		let quadratic_1_intersections = quadratic_bezier_1.intersections(&line, None, None);
		let subpath_intersections = subpath.intersections(&line, None, None);

		assert!(utils::dvec2_compare(
			cubic_bezier.evaluate(ComputeType::Parametric(cubic_intersections[0])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[0])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		assert!(utils::dvec2_compare(
			quadratic_bezier_1.evaluate(ComputeType::Parametric(quadratic_1_intersections[0])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[1])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());
	}

	#[test]
	fn intersection_linear_multiple_subpath_curves_test_three() {
		// M35 125 C40 40 120 120 44 44 Q175 90 145 150 Q70 185 35 125 Z

		let cubic_start = DVec2::new(35., 125.);
		let cubic_handle_1 = DVec2::new(40., 40.);
		let cubic_handle_2 = DVec2::new(120., 120.);
		let cubic_end = DVec2::new(44., 44.);

		let quadratic_1_handle = DVec2::new(175., 90.);
		let quadratic_end = DVec2::new(145., 150.);

		let quadratic_2_handle = DVec2::new(70., 185.);

		let cubic_bezier = Bezier::from_cubic_dvec2(cubic_start, cubic_handle_1, cubic_handle_2, cubic_end);
		let quadratic_bezier_1 = Bezier::from_quadratic_dvec2(cubic_end, quadratic_1_handle, quadratic_end);

		let subpath = Subpath::new(
			vec![
				ManipulatorGroup {
					anchor: cubic_start,
					in_handle: None,
					out_handle: Some(cubic_handle_1),
				},
				ManipulatorGroup {
					anchor: cubic_end,
					in_handle: Some(cubic_handle_2),
					out_handle: None,
				},
				ManipulatorGroup {
					anchor: quadratic_end,
					in_handle: Some(quadratic_1_handle),
					out_handle: Some(quadratic_2_handle),
				},
			],
			true,
		);

		let line = Bezier::from_linear_coordinates(150., 150., 20., 20.);

		let cubic_intersections = cubic_bezier.intersections(&line, None, None);
		let quadratic_1_intersections = quadratic_bezier_1.intersections(&line, None, None);
		let subpath_intersections = subpath.intersections(&line, None, None);

		assert!(utils::dvec2_compare(
			cubic_bezier.evaluate(ComputeType::Parametric(cubic_intersections[0])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[0])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		assert!(utils::dvec2_compare(
			quadratic_bezier_1.evaluate(ComputeType::Parametric(quadratic_1_intersections[0])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[1])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());

		assert!(utils::dvec2_compare(
			quadratic_bezier_1.evaluate(ComputeType::Parametric(quadratic_1_intersections[1])),
			subpath.evaluate(ComputeType::Parametric(subpath_intersections[2])),
			MAX_ABSOLUTE_DIFFERENCE
		)
		.all());
	}

	// TODO: add more intersection tests
}
