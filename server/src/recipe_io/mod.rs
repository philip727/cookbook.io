use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::helpers::is_alnum_whitespace_and_ex_chars;

#[derive(Deserialize, Serialize)]
pub struct RecipeFileJson {
    pub title: String,
    pub description: String,
    pub ingredients: Vec<RecipeMeasurements>,
    pub steps: Vec<RecipeStep>,
}

impl RecipeFileJson {
    pub fn is_valid_recipe(&self) -> Result<(), anyhow::Error> {
        if !self.validate_title() || !self.validate_desc() {
            return Err(anyhow::Error::msg(
                "Invalid title or description, please only use alphanumerical values",
            ));
        };

        let mut seen_orders = HashMap::new();
        for step in self.steps.iter() {
            if !step.validate_step_details() {
                return Err(anyhow::Error::msg(
                    "One of the step details contains invalid characters",
                ));
            };

            if seen_orders.contains_key(&step.order) {
                return Err(anyhow::Error::msg("Steps passed contains duplicate order"));
            }

            seen_orders.insert(step.order, ());
        }

        Ok(())
    }

    fn validate_title(&self) -> bool {
        is_alnum_whitespace_and_ex_chars(&self.title)
    }

    fn validate_desc(&self) -> bool {
        is_alnum_whitespace_and_ex_chars(&self.description)
    }
}

#[derive(Deserialize, Serialize)]
pub struct RecipeMeasurements {
    pub ingredient: String,
    pub measurement: Measurement,
    pub amount: u32,
}

#[derive(Deserialize, Serialize)]
pub enum Measurement {
    Millilitre,
    Litre,
    Teaspoon,
    Tablespoon,
    FluidOz,
    Pint,
    Gallon,
    Milligram,
    Gram,
    Kilogram,
    Pound,
    Ounce,
    Celsius,
    Fahrenheit,
    Piece,
}

#[derive(Deserialize, Serialize)]
pub struct RecipeStep {
    pub order: u32,
    pub step_details: String,
    pub measurements: Option<RecipeMeasurements>,
}

impl RecipeStep {
    pub fn validate_step_details(&self) -> bool {
        is_alnum_whitespace_and_ex_chars(&self.step_details)
    }
}
