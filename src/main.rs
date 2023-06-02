extern crate skim;
use skim::prelude::*;
use std::io::Cursor;
// use std::fs::File;
// use std::path::Path;
use serde::Deserialize;
use itertools::Itertools;
use colored::Colorize;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "lib/"]
struct Asset;

#[derive(Debug, Deserialize, Clone)]
struct Recipe {
    title: String,
    time: String,
    ingredients: Vec<String>,
    steps: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Recipes {
    recipes: Vec<Recipe>,
}

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    // let path = Path::new("src/recipes.json");
    // let file = File::open(path).unwrap();
    // let file = std::fs::read_to_string("src/recipes.json").unwrap();
    let file = Asset::get("recipes.json").unwrap();
    let file_str = std::str::from_utf8(file.data.as_ref()).unwrap();
    let recipes: Recipes = serde_json::from_str(file_str).unwrap();
    let unique_recipes: Vec<Recipe> = recipes.recipes.clone().into_iter().unique_by(|recipe| recipe.title.clone()).collect();

    let titles: Vec<String> = unique_recipes
                                .clone()
                                .into_iter()
                                .map(|recipe| recipe.title)
                                .collect();
    let joined_titles = titles.join("\n");
    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(joined_titles));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        let recipes: Vec<Recipe> = unique_recipes.clone().into_iter().filter(|recipe| recipe.title == item.output()).collect();
        for recipe in recipes {
            println!("\n{}", recipe.title.blue());
            println!("{}\n", recipe.time);
            println!("{}", "Ingredients:".green());
            recipe.ingredients.iter().for_each(|ingredient| println!("{}", ingredient.green()));
            println!("{}", "\nSteps:".purple());
            recipe.steps.iter().for_each(|step| println!("{}\n", step.purple()));
        }
    }
    

}
