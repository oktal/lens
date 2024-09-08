export type AmazonS3Config = {
  accessKeyId: string;
  secretAccessKey: string;
  sessionToken?: string;
  bucket?: string;
  region: string;
};

export type GoogleCloudStorageConfig = {
  serviceAccountPath: string;
  serviceAccountKey: string;
  applicationCredentialsPath: string;
}

export type StoreConfig = {
  s3: AmazonS3Config
} |
{ gcs: GoogleCloudStorageConfig }

export type DatasourceConfig = { url: string, store: StoreConfig }

export type Database = {
  name: string,
  schemas: {
    name: string
    tables: {
      name: string,
      schema: {
        fields: {
          name: string,
          data_type: DataType,
          nullable: boolean,
          metadata: Record<string, string>
        }[],
        metadata: Record<string, string>,
      }
    }[]
  }[]
}

export type Null = {
  kind: "null",
  logical: "null"
};

export type Boolean = {
  kind: "boolean",
  logical: "boolean"
}

export type Int8 = {
  kind: "int8",
  logical: "integer"
}

export type Int16 = {
  kind: "int16",
  logical: "integer"
}

export type Int32 = {
  kind: "int32",
  logical: "integer"
}

export type Int64 = {
  kind: "int64",
  logical: "integer"
}

export type UInt8 = {
  kind: "uint8",
  logical: "integer"
}

export type UInt16 = {
  kind: "uint16",
  logical: "integer"
}

export type UInt32 = {
  kind: "uint32",
  logical: "integer"
}

export type UInt64 = {
  kind: "uint64",
  logical: "integer"
}

export type Float32 = {
  kind: "float32",
  logical: "decimal"
}

export type Float64 = {
  kind: "float64",
  logical: "decimal"
}

export type Decimal128 = {
  kind: "decimal128",
  logical: "decimal",
  precision: number,
  scale: number,
}

export type Decimal256 = {
  kind: "decimal256",
  logical: "decimal",
  precision: number,
  scale: number,
}

export type Date32 = {
  kind: "date32",
  logical: "date"
}

export type Date64 = {
  kind: "date64",
  logical: "date"
}

export type Time32 = {
  kind: "time32",
  logical: "time"
  unit: TimeUnit
}

export type Time64 = {
  kind: "time64",
  logical: "time",
  unit: TimeUnit
}

export type Duration = {
  kind: "duration",
  logical: "time",
  unit: TimeUnit
}

export type Timestamp = {
  kind: "timestamp",
  logical: "timestamp",
  unit: TimeUnit,
  tz: TimeZone
};

export type LargeUtf8 = {
  kind: "largeutf8",
  logical: "string"
}

export type Utf8 = {
  kind: "utf8",
  logical: "string"
}

export type Dictionary = {
  kind: "dictionary",
  logical: "dictionary",
  keyType: DataType,
  valueType: DataType,
}

export type DataType =
  Null
  | Boolean
  | Int8
  | Int16
  | Int32
  | Int64
  | UInt8
  | UInt16
  | UInt32
  | UInt64
  | Float32
  | Float64
  | Decimal128
  | Decimal256
  | Date32
  | Date64
  | Time32
  | Time64
  | Duration
  | Timestamp
  | LargeUtf8
  | Utf8
  | Dictionary;

export type LogicalDataType = Extract<DataType, { logical: string }>["logical"];

export type TimeZone = string | undefined;

export type FileType = "csv" | "arrow" | "parquet" | "avro" | "json";
export type TimeUnit = "Second" | "Millisecond" | "Microsecond" | "Nanosecond";

export type StreamId = string;

export type Row = {
  columns: string[],
  values: string[]
}

export type AwsSSOProfile = {
  name: string,
  region: string,
  startUrl: string,
  accountId: string,
  roleName: string,
}

export type ExportFileFormat = Exclude<FileType, 'avro' | 'arrow'>;

export type ExportOptions = {
  format: ExportFileFormat;
  writeOptions: {
    overwrite: boolean;
    singleFile: boolean;
    partitionBy: string[];
  };
  path: string;
};

export type StreamInfo = {
  id: StreamId;
  query: string;
  rows: number;
}
