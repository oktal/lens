//! Module that defines commands to list previously created or already existing resources

use crate::{
    common::{self, DatasourceConfig},
    lens::{Lens, LensResult},
};

#[tauri::command]
pub fn list_datasources(lens: tauri::State<'_, Lens>) -> Vec<DatasourceConfig> {
    lens.datasources()
}

#[tauri::command]
pub async fn list_databases(lens: tauri::State<'_, Lens>) -> LensResult<Vec<common::Database>> {
    let context = lens.context();
    let mut catalogs = Vec::new();

    for catalog_name in context.catalog_names() {
        let Some(cat) = context.catalog(&catalog_name) else {
            continue;
        };

        let mut schemas = Vec::new();

        for schema_name in cat.schema_names() {
            let Some(schema) = cat.schema(&schema_name) else {
                continue;
            };

            let mut tables = Vec::new();

            for table_name in schema.table_names() {
                let Some(table) = schema.table(&table_name).await? else {
                    continue;
                };

                let schema = table.schema();

                tables.push(common::Table {
                    name: table_name.clone(),
                    schema,
                });
            }

            schemas.push(common::Schema {
                name: schema_name.clone(),
                tables,
            });
        }

        catalogs.push(common::Database {
            name: catalog_name.clone(),
            schemas,
        });
    }

    Ok(catalogs)
}
