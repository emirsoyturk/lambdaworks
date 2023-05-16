use lambdaworks_fft::polynomial::FFTPoly;
use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsFFTField, IsField},
    },
    polynomial::Polynomial,
};

#[derive(Clone, Debug)]
pub struct ConstraintEvaluationTable<F: IsField> {
    // Inner vectors are rows
    pub evaluations: Vec<Vec<FieldElement<F>>>,
    pub trace_length: usize,
}

impl<F: IsFFTField> ConstraintEvaluationTable<F> {
    pub fn new(_n_cols: usize, domain: &[FieldElement<F>]) -> Self {
        let evaluations = Vec::with_capacity(domain.len());

        ConstraintEvaluationTable {
            evaluations,
            trace_length: domain.len(),
        }
    }

    pub fn compute_composition_poly(
        &self,
        coset_offset: &FieldElement<F>,
    ) -> Polynomial<FieldElement<F>> {
        let merged_evals: Vec<FieldElement<F>> = self
            .evaluations
            .iter()
            .map(|row| row.iter().fold(FieldElement::zero(), |acc, d| acc + d))
            .collect();

        // TODO: remove unwrap
        Polynomial::interpolate_offset_fft(&merged_evals, coset_offset).unwrap()
    }
}
