use anchor_lang::prelude::*;

declare_id!("");


// program's entrypoint
pub mod task_management;
pub mod task_crud;
pub mod task_collab;
pub mod task_analytics;



use task_crud::*;
use task_management::*;
use task_collab::*;
use task_analytics::*;


#[program]
pub mod lock_in {
    use super::*;

    pub fn create_todo(
        ctx: Context<AddTask>,
        title: String,
        description: String
    ) -> Result<()> {
        task_crud::create_todo(
            ctx,
            title,
            description)
    }

    pub fn update_todo(
        ctx: Context<UpdateTask>,
        new_title: String,
        new_description: String
    ) -> Result<()> {
        task_crud::update_todo(
            ctx,
            new_title,
            new_description)
    }

    pub fn delete_todo(
        ctx: Context<DeleteTask>
    ) -> Result<()> {
        task_crud::delete_todo(ctx)
    }

    pub fn query_tasks(ctx: Context<QueryTasks>) -> Result<()> {
        task_management::query_tasks(ctx)
    }
}
