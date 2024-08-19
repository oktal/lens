import { invoke } from "@tauri-apps/api";
import type { AwsSSOProfile, Database, DatasourceConfig, DataType, Row, StreamId, TimeUnit, TimeZone } from "./types";

export type AwsCredentials = {
  accessKeyId: string,
  secretAccessKey: string,
  sessionToken: string,
}

export type Client = {
  aws: {
    listSSOProfiles: () => Promise<AwsSSOProfile[]>,
    ssoLogin: ({ startUrl, region, accountId, roleName }: Omit<AwsSSOProfile, 'name'>) => Promise<AwsCredentials>,
  }
  create: {
    datasource: (config: DatasourceConfig) => Promise<void>,
  },

  list: {
    datasources: () => Promise<DatasourceConfig[]>,
    databases: () => Promise<Database[]>,
  }

  sql: {
    run: (query: string) => Promise<void>,
    stream: (query: string) => Promise<StreamId>,
    next: (streamId: StreamId) => Promise<Row[]>,
  }
}

export const client: Client = {
  aws: {
    listSSOProfiles: (): Promise<AwsSSOProfile[]> => {
      return invoke<AwsSSOProfile[]>('list_aws_sso_profiles')
    },

    ssoLogin: async ({ startUrl, region, accountId, roleName }: Omit<AwsSSOProfile, 'name'>): Promise<AwsCredentials> => {
      const [accessKeyId, secretAccessKey, sessionToken] = await invoke<[string, string, string]>('aws_sso_login', {
        startUrl,
        region,
        accountId,
        roleName
      });

      return {
        accessKeyId,
        secretAccessKey,
        sessionToken,
      };
    },
  },

  create: {
    datasource: (config: DatasourceConfig): Promise<void> => {
      return invoke('create_datasource', { config })
    },
  },

  list: {
    datasources: (): Promise<DatasourceConfig[]> => {
      return invoke('list_datasources');
    },
    databases: async (): Promise<Database[]> => {
      const toTimeUnit = (tu: any): TimeUnit | undefined => {
        if (typeof tu === "string") {
          switch (tu.toLowerCase()) {
            case "second":
              return "Second";
            case "millisecond":
              return "Millisecond";
            case "microsecond":
              return "Microsecond";
            case "nanosecond":
              return "Nanosecond";
          }
        }

        return undefined;
      }

      const toTimezone = (tz: any): TimeZone => {
        if (typeof tz === "string") {
          return tz;
        }

        return undefined;
      }
      const toDataType = (dt: any): DataType | undefined => {
        if (typeof dt === "string") {
          const dataType = dt.toLowerCase();

          switch (dataType) {
            case "null":
              return {
                kind: dataType,
                logical: "null",
              };
            case "boolean":
              return {
                kind: dataType,
                logical: "boolean",
              };
            case "int8":
            case "int16":
            case "int32":
            case "int64":
            case "uint8":
            case "uint16":
            case "uint32":
            case "uint64":
              return {
                kind: dataType,
                logical: "integer",
              };
            case "float32":
            case "float64":
              return {
                kind: dataType,
                logical: "decimal",
              };
            case "date32":
            case "date64":
              return {
                kind: dataType,
                logical: "date",
              };
            case "utf8":
            case "largeutf8":
              return {
                kind: dataType,
                logical: "string",
              };
          }
        } else if ("Timestamp" in dt) {
          const [unit, tz] = dt["Timestamp"];
          const timeUnit = toTimeUnit(unit);

          if (timeUnit === undefined) {
            return undefined;
          }

          return {
            kind: "timestamp",
            logical: "timestamp",
            unit: timeUnit,
            tz: toTimezone(tz),
          };
        } else if ("Dictionary" in dt) {
          const [keyType, valueType] = dt["Dictionary"]

          const [keyDataType, valueDataType] = [toDataType(keyType), toDataType(valueType)];
          if (typeof (keyDataType) !== 'undefined' && typeof (valueDataType) !== 'undefined') {
            return {
              kind: "dictionary",
              logical: "dictionary",
              keyType: keyDataType,
              valueType: valueDataType,
            };
          }
        }

        return undefined;
      }
      const toDatabase = (db: any): Database => {
        const database: Database = {
          name: db.name,
          schemas: db.schemas.map((schemaModel: any) => {
            const schema = {
              name: schemaModel.name,
              tables: schemaModel.tables.map((tableModel: any) => {
                const tableSchema = {
                  fields: tableModel.schema.fields.map(
                    (fieldModel: any) => {
                      let data_type = toDataType(
                        fieldModel.data_type,
                      );

                      const field = {
                        name: fieldModel.name,
                        data_type: data_type!,
                        nullable: fieldModel.nullable,
                        metadata: fieldModel.metadata,
                      };

                      return field;
                    },
                  ),
                  metadata: tableModel.schema.metadta,
                };

                const table = {
                  name: tableModel.name,
                  schema: tableSchema,
                };

                return table;
              }),
            };

            return schema;
          }),
        };

        return database;
      };

      const dbs = await invoke<Array<any>>('list_databases');
      return dbs.map((db: any) => toDatabase(db));
    }
  },

  sql: {
    run: (query: string): Promise<void> => {
      return invoke('sql', { query })
    },

    stream: (query: string): Promise<StreamId> => {
      return invoke('sql_stream', { query })
    },

    next: (streamId: StreamId): Promise<Row[]> => {
      return invoke('sql_next', { streamId })
    }
  }
}

