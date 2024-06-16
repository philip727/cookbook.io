export type Ingredient = {
    ingredient: string,
    amount: number,
    measurement: Measurement
}

export enum Measurement {
    Millilitre = "Millilitre",
    Litre = "Litre",
    Teaspoon = "Teaspoon",
    Tablespoon = "Tablespoon",
    FluidOz = "FluidOz",
    Pint = "Pint",
    Gallon = "Gallon",
    Milligram = "Milligram",
    Gram = "Gram",
    Kilogram = "Kilogram",
    Pound = "Pound",
    Ounce = "Ounce",
    Celsius = "Celsius",
    Fahrenheit = "Fahrenheit",
    Piece = "Piece",
}
