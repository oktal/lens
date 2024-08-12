export type AmazonS3Config = {
  accessKeyId: string;
  secretAccessKey: string;
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
