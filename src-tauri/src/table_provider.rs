use std::sync::Arc;

use async_trait::async_trait;
use datafusion::arrow::datatypes::DataType;
use datafusion::common::config_datafusion_err;
use datafusion::datasource::listing::{
    ListingOptions, ListingTable, ListingTableConfig, ListingTableUrl,
};
use datafusion::error::Result;
use datafusion::execution::context::SessionState;
use datafusion::{
    datasource::{provider::TableProviderFactory, TableProvider},
    logical_expr::CreateExternalTable,
};

struct LensTableProvider;

#[async_trait]
impl TableProviderFactory for LensTableProvider {
    async fn create(
        &self,
        state: &SessionState,
        cmd: &CreateExternalTable,
    ) -> Result<Arc<dyn TableProvider>> {
        // Retrieve file format for the type of table to create
        let file_format = state
            .get_file_format_factory(cmd.file_type.as_str())
            .ok_or(config_datafusion_err!(
                "Unable to create table with format {}! Could not find FileFormat.",
                cmd.file_type
            ))?
            .create(state, &cmd.options)?;

        // Determine partition columns and their associated type if provided in the columns
        // definition
        let partition_cols = cmd
            .table_partition_cols
            .iter()
            .map(|p| {
                let data_type = if let Some(field) = cmd
                    .schema
                    .fields()
                    .iter()
                    .find(|f| f.name().eq_ignore_ascii_case(p))
                {
                    field.data_type().clone()
                } else {
                    // This is the type that is returned by the default implementation of DataFusion
                    // when no schema is provided for partition columns.
                    DataType::Dictionary(Box::new(DataType::UInt16), Box::new(DataType::Utf8))
                };

                (p.clone(), data_type)
            })
            .collect();

        // Setup table
        let table_path = ListingTableUrl::parse(&cmd.location)?;
        let options = ListingOptions::new(file_format)
            .with_collect_stat(state.config().collect_statistics())
            .with_target_partitions(state.config().target_partitions())
            .with_table_partition_cols(partition_cols)
            .with_file_sort_order(cmd.order_exprs.clone());

        options.validate_partitions(state, &table_path).await?;

        // Infer schema
        let schema = options.infer_schema(state, &table_path).await?;

        // Create table
        let config = ListingTableConfig::new(table_path)
            .with_listing_options(options)
            .with_schema(schema);
        let provider = ListingTable::try_new(config)?
            .with_cache(state.runtime_env().cache_manager.get_file_statistic_cache());
        let table = provider
            .with_definition(cmd.definition.clone())
            .with_constraints(cmd.constraints.clone())
            .with_column_defaults(cmd.column_defaults.clone());
        Ok(Arc::new(table))
    }
}

pub(super) fn factory() -> Arc<dyn TableProviderFactory> {
    Arc::new(LensTableProvider)
}
