use crate::{
    auth::helpers::{UIDString, UsernameString},
    database::models::{ingredients::Ingredient, recipes::RecipeStep, user::User},
    helpers::is_alnum_whitespace,
    middleware::auth::AuthExtension,
    routes::{
        error::PrettyErrorResponse,
        recipes::helpers::{CreateRecipePayload, FullRecipeDetails, GetRecipeQueryParams},
    },
};
use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use crate::{database::models::recipes::Recipe, pretty_error};

#[get("/all")]
pub async fn get_all_recipes(
    pool: Data<Pool<Postgres>>,
    mut pagination: web::Query<GetRecipeQueryParams>,
) -> impl Responder {
    // Ensure we are getting no more than 10 as this would be costly
    pagination.limit = match pagination.limit {
        Some(limit) => {
            if limit > 10 {
                Some(10)
            } else {
                Some(limit)
            }
        }
        None => Some(10),
    };

    if let None = pagination.offset {
        pagination.offset = Some(0);
    }

    let recipes =
        Recipe::get_with_pagination(&pool, pagination.offset.unwrap(), pagination.limit.unwrap())
            .await;

    if let Err(e) = recipes {
        pretty_error!("Failed to get recipes".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    }

    let recipes = recipes.unwrap();
    // Adds the user information to the request as well
    let mut json_values: Vec<FullRecipeDetails> = Vec::new();
    for recipe in recipes.iter() {
        let user = User::get_by_id(&pool, recipe.user_id).await;
        let Ok(user) = user else {
            continue;
        };

        let Some(user) = user else {
            continue;
        };

        let recipe = FullRecipeDetails::new(recipe.clone(), user);
        json_values.push(recipe);
    }

    HttpResponse::Ok().json(json_values)
}

#[get("/{id}")]
pub async fn get_recipe_by_id(pool: Data<Pool<Postgres>>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let recipe = Recipe::get_by_id(&pool, id).await;

    if let Err(e) = recipe {
        pretty_error!("Failed to get recipe".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    }

    let Some(recipe) = recipe.unwrap() else {
        pretty_error!(
            "No recipe found".to_string(),
            format!("Couldn't find recipe with the id: {}", id),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    let user = User::get_by_id(&pool, recipe.user_id).await;
    if let Err(e) = user {
        pretty_error!("No recipe poster found".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    };

    let Some(user) = user.unwrap() else {
        pretty_error!(
            "No recipe poster found".to_string(),
            format!("Couldn't find user with the id: {}", recipe.user_id),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    let recipe = FullRecipeDetails::new(recipe, user);
    HttpResponse::Ok().json(recipe)
}

#[get("/steps/{id}")]
pub async fn get_recipe_steps_from_recipe(
    pool: Data<Pool<Postgres>>,
    path: Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    if !Recipe::exists(&pool, id).await {
        pretty_error!(
            "No recipe found".to_string(),
            format!("No recipe exists with the id: {}", id),
            error
        );

        return HttpResponse::NotFound().json(error);
    }

    let recipe_steps = RecipeStep::get_recipe_steps(&pool, id).await;

    if let Err(e) = recipe_steps {
        pretty_error!("No recipe steps found".to_string(), e.to_string(), error);

        return HttpResponse::NotFound().json(error);
    }

    let Some(recipe_steps) = recipe_steps.unwrap() else {
        pretty_error!(
            "No steps found".to_string(),
            format!("Couldn't the steps for the recipe with the id: {}", id),
            error
        );

        return HttpResponse::NotFound().json(error);
    };

    if recipe_steps.is_empty() {
        pretty_error!(
            "No steps found".to_string(),
            format!("Couldn't the steps for the recipe with the id: {}", id),
            error
        );

        return HttpResponse::NotFound().json(error);
    }

    HttpResponse::Ok().json(recipe_steps)
}

// #[post(/create)]
pub async fn create_recipe(
    req: HttpRequest,
    payload: web::Json<CreateRecipePayload>,
    pool: Data<Pool<Postgres>>,
) -> impl Responder {
    let extensions = req.extensions();
    let auth = extensions.get::<AuthExtension>();

    // Ensures all details are passed
    if !is_alnum_whitespace(&payload.title) {
        pretty_error!(
            "Invalid title".to_string(),
            "The title to the recipe must be alphanumerical",
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    if !is_alnum_whitespace(&payload.description) {
        pretty_error!(
            "Invalid description".to_string(),
            "The description to the recipe must be alphanumerical",
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    // ARG !
    if payload.steps.is_empty() {
        pretty_error!(
            "No steps given".to_string(),
            "You can't create a recipe with no steps",
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    // Ensures no steps are empty
    for step in payload.steps.iter() {
        if !step.description.is_empty() {
            continue;
        };

        pretty_error!(
            "Invalid step".to_string(),
            format!("Step {} has no valid instructions", step.order),
            error
        );

        return HttpResponse::BadRequest().json(error);
    }

    // Need to make sure we can actually get the auth details from extension
    let Some(auth) = auth else {
        pretty_error!(
            "Unauthorized".to_string(),
            "Unable to get auth details from extension",
            error
        );
        return HttpResponse::Unauthorized().json(error);
    };

    let Ok(uid) = auth.uid.parse::<i32>() else {
        pretty_error!(
            "Unauthorized".to_string(),
            "Invalid uid passed in auth",
            error
        );
        return HttpResponse::InternalServerError().json(error);
    };

    let recipe_id = Recipe::insert(
        &pool,
        payload.title.clone(),
        payload.description.clone(),
        uid,
    )
    .await;

    if let Err(e) = recipe_id {
        pretty_error!("Failed to insert recipe".to_string(), e.to_string(), error);
        // Doesnt need to panic, just attempt to delete
        return HttpResponse::InternalServerError().json(error);
    };

    let recipe_id = recipe_id.unwrap();
    let steps = &payload.steps;

    // Iterate over each step and attempt to insert
    for step in steps.iter() {
        let recipe_step =
            RecipeStep::insert(&pool, recipe_id, step.description.clone(), step.order).await;
        if let Err(e) = recipe_step {
            pretty_error!(
                "Failed to insert recipe steps".to_string(),
                e.to_string(),
                error
            );

            // Doesnt need to panic, just attempt to delete since we dont want the recipe to stay
            // inserted if failed
            // -> Should delete ingredients as well due to on delete cascade
            let _ = Recipe::delete(&pool, recipe_id).await;
            return HttpResponse::InternalServerError().json(error);
        }
    }

    let ingredients = &payload.ingredients;
    for ingredient in ingredients.iter() {
        let ingredient =
            Ingredient::insert(&pool, ingredient.name.clone(), ingredient.amount, recipe_id).await;

        if let Err(e) = ingredient {
            pretty_error!(
                "Failed to insert measurement".to_string(),
                e.to_string(),
                error
            );

            // Doesnt need to panic, just attempt to delete since we dont want the recipe to stay
            // inserted if failed
            // -> Should delete ingredients as well due to on delete cascade
            let _ = Recipe::delete(&pool, recipe_id).await;
            return HttpResponse::InternalServerError().json(error);
        }
    }

    HttpResponse::Ok().body("Succesfully created recipe")
}

pub async fn get_ingredients() {}
