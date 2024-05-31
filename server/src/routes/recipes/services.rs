use crate::{
    database::models::{recipes::RecipeStep, user::User},
    routes::{
        error::PrettyErrorResponse,
        recipes::helpers::{CreateRecipePayload, FullRecipeDetails, GetRecipeQueryParams},
    },
};
use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpRequest, HttpResponse, Responder,
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
    // No point in doing any more queries for something that doesnt exist
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
) -> impl Responder {
    HttpResponse::Ok().body("")
}
