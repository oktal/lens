# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.2.2](https://github.com/oktal/lens/compare/664ebef7b2e0489c327b74f5fa427941771f3808..v0.2.2) - 2024-09-25
#### Bug Fixes
- properly display datasources in table creation dialog - ([979d529](https://github.com/oktal/lens/commit/979d5294826b98cef8ef4c611a67be1fa93a725b)) - [@octal](https://github.com/octal)
#### Miscellaneous Chores
- **(version)** version 0.2.2 - ([c631ae3](https://github.com/oktal/lens/commit/c631ae37eb34a671fa6dc475e18c53e35fec8056)) - [@octal](https://github.com/octal)
- **(version)** v0.2.1 - ([664ebef](https://github.com/oktal/lens/commit/664ebef7b2e0489c327b74f5fa427941771f3808)) - [@octal](https://github.com/octal)

- - -

## [v0.2.1](https://github.com/oktal/lens/compare/26f342bc4eea9de2a382d2fa0efe93e4543b90bc..v0.2.1) - 2024-09-25
#### Miscellaneous Chores
- **(CHANGELOG)** remove duplicate entry from CHANGELOG - ([232990c](https://github.com/oktal/lens/commit/232990c5518f856c46116f94d4b38db7a5f8f661)) - [@octal](https://github.com/octal)
#### Refactoring
- **(ui)** minor update of how datasources are displayed - ([26f342b](https://github.com/oktal/lens/commit/26f342bc4eea9de2a382d2fa0efe93e4543b90bc)) - [@octal](https://github.com/octal)

- - -

- - -
## [v0.2.0](https://github.com/oktal/lens/compare/0fdcff7d5a13800a33ca17a777a5b1a2482e63a4..v0.2.0) - 2024-09-23
#### Bug Fixes
- **(ui.sidebar)** open project GitHub when clicking github icon button - ([dd8b3ad](https://github.com/oktal/lens/commit/dd8b3ad372e966ad1d2c6d60cd1e7fb6b92d8372)) - [@octal](https://github.com/octal)
- add google gcs datasource configuration options - ([ea782aa](https://github.com/oktal/lens/commit/ea782aa3ac74d43acabb709a76422ad5738acfd6)) - [@octal](https://github.com/octal)
#### Features
- **(ui)** query title can now be updated from history - ([8053c4b](https://github.com/oktal/lens/commit/8053c4bd3775b1d9e349d77dd7699615248f9519)) - [@octal](https://github.com/octal)
#### Miscellaneous Chores
- **(ci)** update release gh workflow tag prefix - ([fdf6b1e](https://github.com/oktal/lens/commit/fdf6b1ed6cb53f8800104e9c7620454e40036171)) - [@octal](https://github.com/octal)
- **(version)** update version in tauri.conf.json - ([66364f0](https://github.com/oktal/lens/commit/66364f09298b5fdc00ddbb7723b33c2a13d31ea8)) - [@octal](https://github.com/octal)
- **(version)** bump version in Cargo.toml - ([a992442](https://github.com/oktal/lens/commit/a9924427e02e65867835c26948884760e0eac3ed)) - [@octal](https://github.com/octal)
- **(version)** v0.2.0 - ([1f7ba32](https://github.com/oktal/lens/commit/1f7ba32ee1bb9d702011c8f5253ea08f87ea473a)) - [@octal](https://github.com/octal)
- configure tag prefix for cog - ([00f1a89](https://github.com/oktal/lens/commit/00f1a8944a0bbe216fff05014ef088854ff1a72d)) - [@octal](https://github.com/octal)
- make window maximized by default - ([2a9e8a2](https://github.com/oktal/lens/commit/2a9e8a2ef0ee8c6da9341f16a9ded6ef031f5ad3)) - [@octal](https://github.com/octal)
- bump CHANGELOG - ([0fdcff7](https://github.com/oktal/lens/commit/0fdcff7d5a13800a33ca17a777a5b1a2482e63a4)) - [@octal](https://github.com/octal)

- - -

## [0.1.0](https://github.com/oktal/lens/compare/881a737c1ae48e89d758d3f8cc43cbe24c3ac41f..0.1.0) - 2024-09-16
#### Bug Fixes
- **(backend)** properly unespace characters before executing sql query - ([4e52725](https://github.com/oktal/lens/commit/4e52725e5ad08ae795c90c0ec4df3f7c54f9d316)) - [@octal](https://github.com/octal)
- **(ci)** attempt to fix build on macos by increasing node heap size - ([d3eabfe](https://github.com/oktal/lens/commit/d3eabfe98d2edf7b69a1a61c612091f904d4e8be)) - [@octal](https://github.com/octal)
- **(components)** make sure that path that represents a directory structure end with a '/' delimiter - ([2526c79](https://github.com/oktal/lens/commit/2526c79e9c36edad32d92f04d648ad9a9347d0a6)) - [@octal](https://github.com/octal)
- **(lens)** properly handle Dictionary DataType - ([6814427](https://github.com/oktal/lens/commit/681442782f093e7c95eb25d67ba2def08ccc5d81)) - [@octal](https://github.com/octal)
- **(ui)** properly expand databases - ([3d3a99b](https://github.com/oktal/lens/commit/3d3a99b0521537b9e684bc310e55743811243ff4)) - [@octal](https://github.com/octal)
- **(ui)** fix import in query pane - ([109afc8](https://github.com/oktal/lens/commit/109afc8aec09e560b073226c1aa046202d1b6591)) - [@octal](https://github.com/octal)
- **(ui)** correctly bind checked property to switch options in create table dialog - ([3b3e8a6](https://github.com/oktal/lens/commit/3b3e8a6a1a200f39dd15a785a240f88ab4d6a1d2)) - [@octal](https://github.com/octal)
- **(ui)** properly display last page number - ([6b6d87f](https://github.com/oktal/lens/commit/6b6d87ffd44d63a707a744a2e1cba41e77eca963)) - [@octal](https://github.com/octal)
- **(ui)** properly use options when creating table - ([9ac1f52](https://github.com/oktal/lens/commit/9ac1f520007c05aa470067ceada0c4c1b4a807b2)) - [@octal](https://github.com/octal)
- **(ui)** make query result table columns reactive - ([77b7505](https://github.com/oktal/lens/commit/77b7505433997c3ff2fed369ab7ec11d829785e4)) - [@octal](https://github.com/octal)
- **(ui)** properly refresh query string when reneweing query from history - ([ab61de4](https://github.com/oktal/lens/commit/ab61de416b431f984ec97cf7782bab9d21355d09)) - [@octal](https://github.com/octal)
- **(ui)** make query results table reactive when stream updates - ([a733f40](https://github.com/oktal/lens/commit/a733f4054533b28a2e2160ad1d90191fa3891e66)) - [@octal](https://github.com/octal)
- **(ui)** make datasource URL reactive in DatasourceDialog and properly type datasource item - ([280db57](https://github.com/oktal/lens/commit/280db576b31d0c0767dac1d681d7232874b7c5a0)) - [@octal](https://github.com/octal)
- **(ui)** add ScrollArea to aws region selection - ([9eb4f63](https://github.com/oktal/lens/commit/9eb4f635ebdbc6cf4de371f274c46f1ea3521468)) - [@octal](https://github.com/octal)
- **(ui.query)** remove tanstack table until we figure how to to properly make data reactive - ([6205a69](https://github.com/oktal/lens/commit/6205a699bc59cdab25b44502f0ef086b4b05a5e8)) - [@octal](https://github.com/octal)
- **(ui.query)** removed `$effect` on query results table - ([be9cc19](https://github.com/oktal/lens/commit/be9cc19f46544545271065cb87e5808c9bbfd35a)) - [@octal](https://github.com/octal)
- add tauri-apps/cli to dependencies - ([95ca564](https://github.com/oktal/lens/commit/95ca56462fab7c66d8ccff7acd05b16c1ce5edfd)) - [@octal](https://github.com/octal)
- add session token to AWS credentials for SSO login - ([51a9eaa](https://github.com/oktal/lens/commit/51a9eaa62d7af2a32234cfcbac64302a65da0bb6)) - [@octal](https://github.com/octal)
#### Continuous Integration
- **(release)** add workflow_dispatch trigger - ([a36ac53](https://github.com/oktal/lens/commit/a36ac530764a091447ccd78b172a637ad2072a72)) - [@octal](https://github.com/octal)
- **(release)** update dependencies for ubuntu 22.04 - ([e6f79a6](https://github.com/oktal/lens/commit/e6f79a6337413b07ab54ca81ca88b6d8ec0d7888)) - [@octal](https://github.com/octal)
- **(release)** go back to yarn to install dependencies - ([51c2f9b](https://github.com/oktal/lens/commit/51c2f9be2e06d845aba7489debb1572465ba0f4b)) - [@octal](https://github.com/octal)
- **(release)** use pnpm to install dependencies - ([ba74528](https://github.com/oktal/lens/commit/ba74528b0b165267c5a5959658bc1a4725a1badb)) - [@octal](https://github.com/octal)
- **(release)** use pnpm cache in release workflow - ([88eba8a](https://github.com/oktal/lens/commit/88eba8a4789f419926e29541ad8944c83a3d5306)) - [@octal](https://github.com/octal)
- prepare release workflow - ([59361bd](https://github.com/oktal/lens/commit/59361bd79278c6a2cb9ae94773a6cbd88b597c0c)) - [@octal](https://github.com/octal)
- add basic build action - ([2ce4b42](https://github.com/oktal/lens/commit/2ce4b42208d96328d89dd211cf70b6dc25e1b045)) - [@octal](https://github.com/octal)
#### Features
- **(backend)** add command to close stream - ([a29e26c](https://github.com/oktal/lens/commit/a29e26ca1883cda3e413b0f91bbd8ebf110d944b)) - [@octal](https://github.com/octal)
- **(ui)** add context menu to drop schema and table - ([9ea1d83](https://github.com/oktal/lens/commit/9ea1d83ae8e26be3d1f08bd09c8d3b56f828ec99)) - [@octal](https://github.com/octal)
- **(ui)** add stopwatch to measure query execution time - ([3f19b52](https://github.com/oktal/lens/commit/3f19b5287b873cd3ddb219982ccbb231d1c06f64)) - [@octal](https://github.com/octal)
- **(ui)** queries can now have a title - ([050382d](https://github.com/oktal/lens/commit/050382dfce2ad80d9641282baf1508178312297c)) - [@octal](https://github.com/octal)
- **(ui)** add support for drag and dropping files directly to the explorer for quick table creation - ([4949b1d](https://github.com/oktal/lens/commit/4949b1d083174905ee1909dc66f407e638b877eb)) - [@octal](https://github.com/octal)
- **(ui)** add "multi lines" option to csv options - ([ea5406c](https://github.com/oktal/lens/commit/ea5406ccf22588a54bd4529e49624ea2e6a5f8d0)) - [@octal](https://github.com/octal)
- **(ui)** add Monaco editor and SQL syntax highlighting - ([6b69fda](https://github.com/oktal/lens/commit/6b69fdaed040c10bd850667cd66754f8a9449941)) - [@octal](https://github.com/octal)
- **(ui)** make both panes closable in split mode - ([12581c0](https://github.com/oktal/lens/commit/12581c01d454243fb59f04a1d658a094c90822a4)) - [@octal](https://github.com/octal)
- **(ui)** add sidebar and light/dark mode toggle dropdown - ([50d3dc3](https://github.com/oktal/lens/commit/50d3dc332be1fb06b7c7da9ca5102267b0cc6a09)) - [@octal](https://github.com/octal)
- **(ui)** add overlay to query pane - ([6aa6e8a](https://github.com/oktal/lens/commit/6aa6e8a3305f77ff54c4d6bf6ce77543bfeea84e)) - [@octal](https://github.com/octal)
- **(ui)** add query renewing - ([fc276a3](https://github.com/oktal/lens/commit/fc276a327ca5de8f60e4715643f8e9d4d622401a)) - [@octal](https://github.com/octal)
- **(ui)** add query history - ([b93d9e4](https://github.com/oktal/lens/commit/b93d9e4db86b7050affe419fed2081ecaf6c3261)) - [@octal](https://github.com/octal)
- **(ui)** show row number in results table - ([1a81fc6](https://github.com/oktal/lens/commit/1a81fc66761304887dd94ca72668fde23e831e8b)) - [@octal](https://github.com/octal)
- **(ui)** add dialog to export query results - ([15d16ff](https://github.com/oktal/lens/commit/15d16ff94f4b8896b8b1e6e36858d7e2df1af8fb)) - [@octal](https://github.com/octal)
- **(ui)** add pagination to raw text display in query results pane - ([8d7779d](https://github.com/oktal/lens/commit/8d7779de98db6da226ac25bb69ec6dba544fc2a3)) - [@octal](https://github.com/octal)
- **(ui)** use TanStack table to display data and allow for column sorting - ([04b2c76](https://github.com/oktal/lens/commit/04b2c76fb62694d9dad04f1758ce6cd4dec466b3)) - [@octal](https://github.com/octal)
- **(ui)** add horizontal and vertical split to query pane - ([b44f57c](https://github.com/oktal/lens/commit/b44f57c4becd06701292f59cb23452371029f321)) - [@octal](https://github.com/octal)
- **(ui)** add progress bar in results table when loading data - ([c84b7ab](https://github.com/oktal/lens/commit/c84b7abc04a4ee5dcb492b3baf0726139d4813ef)) - [@octal](https://github.com/octal)
- **(ui)** add indeterminate progress bar - ([932b3e9](https://github.com/oktal/lens/commit/932b3e97ee307b29e6753a377b4ca353c74a5c0b)) - [@octal](https://github.com/octal)
- **(ui)** add spinner to give user feedback when a table is being created - ([da0ed13](https://github.com/oktal/lens/commit/da0ed13d2f5c8dcba46e8e4a1659a5c52fc5c817)) - [@octal](https://github.com/octal)
- **(ui)** add sql query execution and results table - ([f4d1ecf](https://github.com/oktal/lens/commit/f4d1ecfde8d04063ca127466ec5d62280cbda608)) - [@octal](https://github.com/octal)
- **(ui)** add table creation - ([3095c77](https://github.com/oktal/lens/commit/3095c773a944bd0847efa0176a6da5029eae7b25)) - [@octal](https://github.com/octal)
- **(ui.aws)** add a list of SSO profiles retrieved from configuration file when configuring s3 datasource through SSO - ([f8bfcbe](https://github.com/oktal/lens/commit/f8bfcbe8ed3f5d1a3137e1ea1c03f7c2525138bd)) - [@octal](https://github.com/octal)
- **(ui.table)** add a way to specify type for a partition column - ([e5fbedf](https://github.com/oktal/lens/commit/e5fbedf9cc3ac965c329851488da0afb5fbf2fdb)) - [@octal](https://github.com/octal)
- add aws SSO login - ([b7f2046](https://github.com/oktal/lens/commit/b7f204695e268d9d4dee2fa9cb40a3295a13d4cb)) - [@octal](https://github.com/octal)
#### Miscellaneous Chores
- **(README)** put logo - ([b91ba4a](https://github.com/oktal/lens/commit/b91ba4ac0b28e289aa8e5bce8ccddce791a67173)) - [@octal](https://github.com/octal)
- **(README)** add lens logo as an asset - ([71cfe04](https://github.com/oktal/lens/commit/71cfe0472a3a6a902647ea8ee70a4766d9aa995b)) - [@octal](https://github.com/octal)
- **(backend)** update datafusion and other dependencies - ([ebbfd94](https://github.com/oktal/lens/commit/ebbfd94f3e289fa6114412153a545f66793881d0)) - [@octal](https://github.com/octal)
- **(backend)** rename default database to lens - ([fbd631c](https://github.com/oktal/lens/commit/fbd631cea920c36dcc77996966b4a1cb7350492b)) - [@octal](https://github.com/octal)
- **(backend)** update Cargo toml metadata - ([920ecbb](https://github.com/oktal/lens/commit/920ecbb722874d4670a65007d198ab00ec1cd086)) - [@octal](https://github.com/octal)
- **(ui)** tweak icons - ([502b838](https://github.com/oktal/lens/commit/502b8385d090ea4d7f265b7dc1a270db581136dc)) - [@octal](https://github.com/octal)
- **(ui)** improve query title handling - ([c4fc318](https://github.com/oktal/lens/commit/c4fc31873598f088523a1791a9cf77f83cfc91e3)) - [@octal](https://github.com/octal)
- **(ui)** replace spinner when refreshing entities by infinite progress bar - ([8ad1271](https://github.com/oktal/lens/commit/8ad127197ffd3544232d67574966937f58644869)) - [@octal](https://github.com/octal)
- **(ui)** reduce query pane top bar height - ([0a9509c](https://github.com/oktal/lens/commit/0a9509ccbc15f51040a1a55075a2a0eb0f558d5c)) - [@octal](https://github.com/octal)
- **(ui)** update query panel scroll area - ([93642a0](https://github.com/oktal/lens/commit/93642a0d0e97d7767de3eaaed448536f3e0f5cce)) - [@octal](https://github.com/octal)
- **(ui)** reorganize file structure - ([0747327](https://github.com/oktal/lens/commit/074732765c08fd373a80d7820d0ef3a156ab160f)) - [@octal](https://github.com/octal)
- **(ui)** add scroll area to left panel - ([0db98a2](https://github.com/oktal/lens/commit/0db98a2f98176b9f1d6b916ac9df5e5e3f250223)) - [@octal](https://github.com/octal)
- **(ui)** add icons to left side bar - ([30f4d1a](https://github.com/oktal/lens/commit/30f4d1a51aa2fbe1d5560f4402d91cb97c74cae8)) - [@octal](https://github.com/octal)
- **(ui)** reduce size of table column items - ([0071218](https://github.com/oktal/lens/commit/0071218ff91aa86cdc26823a7dbb637ce513e095)) - [@octal](https://github.com/octal)
- **(ui)** properly animate carret in objects tree - ([e3c2a9c](https://github.com/oktal/lens/commit/e3c2a9c881ea4b14b80faa64867801e681ccf541)) - [@octal](https://github.com/octal)
- **(ui)** reset query state when a query results in an error - ([8f29d0d](https://github.com/oktal/lens/commit/8f29d0d3a1476976d39411008125166bd25ae85a)) - [@octal](https://github.com/octal)
- **(ui)** make main content height full screen - ([dcb9c95](https://github.com/oktal/lens/commit/dcb9c95ebf3ccd55c1fea17ee78403b1f80e50bc)) - [@octal](https://github.com/octal)
- **(ui.query)** properly disable export button when there is no data to export - ([c037e23](https://github.com/oktal/lens/commit/c037e23fec4896ef8d77ae04d5029693de54b7d3)) - [@octal](https://github.com/octal)
- remove CHANGELOG - ([d8ea8e7](https://github.com/oktal/lens/commit/d8ea8e76c03f4903a580bd28cc3f004e611a8806)) - [@octal](https://github.com/octal)
- add CHANGELOG - ([b893330](https://github.com/oktal/lens/commit/b893330cae54177e14bf64d0ae92d779a34a3486)) - [@octal](https://github.com/octal)
- properly type Grid rest props - ([f7409a5](https://github.com/oktal/lens/commit/f7409a5d8d2515efa344c6264db5b97c3837ef81)) - [@octal](https://github.com/octal)
- rename package to lens - ([1cc71de](https://github.com/oktal/lens/commit/1cc71dedea7c0e3a6540f04d1e82c13dc9fbd4df)) - [@octal](https://github.com/octal)
- update icons - ([fe97e7b](https://github.com/oktal/lens/commit/fe97e7b35f2e6b5bda55a29f1c5b015cfa6c2af9)) - [@octal](https://github.com/octal)
- reformatting - ([9bddd61](https://github.com/oktal/lens/commit/9bddd61507c0b123ce639a8a6156c7399543879d)) - [@octal](https://github.com/octal)
- update logo - ([8f2bec6](https://github.com/oktal/lens/commit/8f2bec6850ac1f4a0863ee4113d58abdd6a0aec9)) - [@octal](https://github.com/octal)
- rename to lens - ([1657d7a](https://github.com/oktal/lens/commit/1657d7af017926fa880f441f7e1d3c2cfa147120)) - [@octal](https://github.com/octal)
- update demo gif with higher quality - ([9186b9a](https://github.com/oktal/lens/commit/9186b9a2f0a86153a50e5552387c3581ac8bdf11)) - [@octal](https://github.com/octal)
- update demo gif - ([2fa9d6c](https://github.com/oktal/lens/commit/2fa9d6c55a29f8c374ae391e1bf5118b31be7390)) - [@octal](https://github.com/octal)
- add demo video to assets - ([e4ca28d](https://github.com/oktal/lens/commit/e4ca28d5756c9fc17945d6517f02dc56aade29d4)) - [@octal](https://github.com/octal)
- add funding - ([dd6ca1b](https://github.com/oktal/lens/commit/dd6ca1bde22efdf6b030a8e3e8aa602aaada8d98)) - [@octal](https://github.com/octal)
- update README - ([e81f7aa](https://github.com/oktal/lens/commit/e81f7aa2260c71907dadef633ddcb841c2fa2d83)) - [@octal](https://github.com/octal)
- remove unused import - ([ab2d176](https://github.com/oktal/lens/commit/ab2d176aecb600bec5dd42495c93296dbbc9fe71)) - [@octal](https://github.com/octal)
- add default catalog and schema - ([2fe6402](https://github.com/oktal/lens/commit/2fe640262b89cdef5d636d20a62cb10a86fd769a)) - [@octal](https://github.com/octal)
- add theme and logo - ([eab6195](https://github.com/oktal/lens/commit/eab6195b19b3659fa020530a3f2b7067d1064fb9)) - [@octal](https://github.com/octal)
- create basic README - ([79264ee](https://github.com/oktal/lens/commit/79264ee37dc5fc2488adbd61260f3c1af13a7a23)) - [@octal](https://github.com/octal)
#### Refactoring
- create component for monaco - ([bc32c8a](https://github.com/oktal/lens/commit/bc32c8aa5cba80db47522418254cb277b75a6d0b)) - [@octal](https://github.com/octal)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
